use alloc::vec;

use argon_primitives::{
	bitcoin::{BitcoinError, BitcoinSignature, CompressedBitcoinPubkey, Satoshis},
	ensure,
};
use bitcoin::{
	absolute::LockTime,
	bip32::{KeySource, Xpriv},
	ecdsa::Signature,
	key::Secp256k1,
	psbt::Input,
	sighash::SighashCache,
	transaction::Version,
	Amount, EcdsaSighashType, Network, OutPoint, PrivateKey, Psbt, PublicKey, ScriptBuf, Sequence,
	Transaction, TxIn, TxOut, Witness,
};
use k256::ecdsa::signature::Verifier;
use miniscript::psbt::PsbtExt;

use crate::{
	cosign_script::{CosignScript, CosignScriptArgs, UnlockStep},
	errors::Error,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtxoUnlocker {
	pub cosign_script: CosignScript,
	pub unlock_step: UnlockStep,
	pub psbt: Psbt,
}

impl UtxoUnlocker {
	pub fn from_script(
		cosign_script: CosignScript,
		utxo_satoshis: Satoshis,
		utxo_txid: bitcoin::Txid,
		utxo_vout: u32,
		unlock_step: UnlockStep,
		fee: Amount,
		to_script_pubkey: ScriptBuf,
	) -> Result<Self, Error> {
		let lock_time = cosign_script.unlock_height(unlock_step);
		let out_point = OutPoint { txid: utxo_txid, vout: utxo_vout };
		let out_amount = Amount::from_sat(utxo_satoshis);
		let unsigned_tx = Transaction {
			version: Version::TWO, // Post BIP-68.
			lock_time: LockTime::from_height(lock_time)
				.map_err(|_| BitcoinError::InvalidLockTime)?,
			input: vec![TxIn {
				previous_output: out_point,
				sequence: Sequence::ENABLE_LOCKTIME_NO_RBF,
				..TxIn::default()
			}],
			output: vec![TxOut {
				value: out_amount.checked_sub(fee).ok_or(Error::FeeOverflow)?,
				script_pubkey: to_script_pubkey,
			}],
		};

		let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).map_err(Error::PsbtError)?;

		psbt.inputs[0] = Input {
			witness_utxo: Some(TxOut {
				value: out_amount,
				script_pubkey: cosign_script.get_script_pubkey(),
			}),
			witness_script: Some(cosign_script.script.clone()),
			sighash_type: Some(EcdsaSighashType::All.into()),
			..Input::default()
		};
		let descriptor = cosign_script.create_descriptor()?;
		psbt.update_input_with_descriptor(0, &descriptor).map_err(|_| {
			log::error!("Error updating PSBT with descriptor: {:#?}", descriptor);
			Error::PsbtFinalizeError
		})?;

		Ok(Self { cosign_script, unlock_step, psbt })
	}

	#[allow(clippy::too_many_arguments)]
	pub fn new(
		cosign_script_args: CosignScriptArgs,
		utxo_satoshis: Satoshis,
		utxo_txid: bitcoin::Txid,
		utxo_vout: u32,
		unlock_step: UnlockStep,
		fee: Amount,
		pay_to_script_pubkey: ScriptBuf,
		network: Network,
	) -> Result<Self, Error> {
		Self::from_script(
			CosignScript::new(cosign_script_args, network)?,
			utxo_satoshis,
			utxo_txid,
			utxo_vout,
			unlock_step,
			fee,
			pay_to_script_pubkey,
		)
	}

	pub fn add_signature(&mut self, pubkey: PublicKey, signature: Signature) {
		self.psbt.inputs[0].partial_sigs.insert(pubkey, signature);
	}

	/// No std friendly version of verifying a signature
	pub fn verify_signature_raw(
		&self,
		pubkey: CompressedBitcoinPubkey,
		signature_der_bytes: &BitcoinSignature,
	) -> Result<bool, Error> {
		let psbt = &self.psbt;
		let mut cache = SighashCache::new(&psbt.unsigned_tx);

		// Get the sighash message
		let (msg, _) = match psbt.sighash_ecdsa(0, &mut cache) {
			Ok(result) => result,
			Err(_) => return Ok(false),
		};

		let (_sighash_type, sigdata) =
			signature_der_bytes.0.split_last().ok_or(Error::InvalidSignatureBytes)?;

		let signature =
			k256::ecdsa::Signature::from_der(sigdata).map_err(|_| Error::InvalidSignatureBytes)?;

		let pubkey = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey.0)
			.map_err(|_| Error::InvalidCompressPubkeyBytes)?;

		Ok(pubkey.verify(msg.as_ref(), &signature).is_ok())
	}

	pub fn sign(&mut self, privkey: PrivateKey) -> Result<(Signature, PublicKey), Error> {
		let psbt = &mut self.psbt;
		let mut cache = SighashCache::new(&psbt.unsigned_tx);
		let (msg, ecdsa_type) = psbt.sighash_ecdsa(0, &mut cache).map_err(Error::SignError)?;
		let secp = Secp256k1::new();
		let sig = secp.sign_ecdsa(&msg, &privkey.inner);
		let signature = Signature { signature: sig, sighash_type: ecdsa_type };
		let pubkey = privkey.public_key(&secp);
		psbt.inputs[0].partial_sigs.insert(pubkey, signature);
		Ok((signature, pubkey))
	}

	pub fn sign_derived(
		&mut self,
		master_xpriv: Xpriv,
		key_source: KeySource,
	) -> Result<(Signature, PublicKey), Error> {
		let psbt = &mut self.psbt;
		let secp = Secp256k1::new();
		let child_xpriv =
			master_xpriv.derive_priv(&secp, &key_source.1).map_err(Error::Bip32Error)?;
		let child_priv = child_xpriv.to_priv();
		let pubkey = child_priv.public_key(&secp);

		psbt.inputs[0].bip32_derivation.insert(pubkey.inner, key_source);

		match psbt.sign(&master_xpriv, &secp) {
			Ok(_) => ensure!(!psbt.inputs[0].partial_sigs.is_empty(), Error::SignatureExpected),
			Err((_, errs)) => return Err(Error::SigningErrors(errs)),
		};

		let Some((_, signature)) =
			psbt.inputs[0].partial_sigs.iter().find(|(k, _)| k.inner == pubkey.inner)
		else {
			return Err(Error::DerivedKeySignError);
		};

		Ok((*signature, pubkey))
	}

	pub fn create_witness(&mut self) -> Result<(), Error> {
		let mut witness = Witness::new();
		let psbt = &mut self.psbt;
		let partial_sigs = &psbt.inputs[0].partial_sigs;
		let owner_pubkey = self.cosign_script.script_args.bitcoin_owner_pubkey()?;

		let vault_pubkey = self.cosign_script.script_args.bitcoin_vault_pubkey()?;

		let vault_claim_pubkey = self.cosign_script.script_args.bitcoin_vault_claim_pubkey()?;

		if let Some(sig) = partial_sigs.get(&vault_pubkey) {
			witness.push(sig.to_vec());
		}
		if let Some(sig) = partial_sigs.get(&vault_claim_pubkey) {
			witness.push(sig.to_vec());
		}

		if let Some(sig) = partial_sigs.get(&owner_pubkey) {
			witness.push(sig.to_vec());
		}
		witness.push(self.cosign_script.script.clone());

		psbt.inputs[0].final_script_witness = Some(witness);
		Ok(())
	}

	pub fn extract_tx(&mut self) -> Result<Transaction, Error> {
		let tx = {
			let mut psbt = self.psbt.clone();
			psbt = psbt.finalize(&Secp256k1::new()).map_err(|(_, e)| {
				log::error!("Error finalizing PSBT: {:#?}", e);
				Error::PsbtFinalizeError
			})?;
			psbt.extract_tx().map_err(Error::ExtractTxError)?
		};

		// Clear all the data fields as per the spec.
		{
			let psbt = &mut self.psbt;
			psbt.inputs[0].partial_sigs.clear();
			psbt.inputs[0].sighash_type = None;
			psbt.inputs[0].redeem_script = None;
			psbt.inputs[0].witness_script = None;
			psbt.inputs[0].bip32_derivation.clear();
		}

		Ok(tx)
	}
}

#[cfg(feature = "hwi")]
mod hwi {
	use hwi::{types::HWIDevice, HWIClient};

	impl UtxoUnlocker {
		pub fn sign_hwi(
			&mut self,
			key_source: KeySource,
			device: Option<HWIDevice>,
			network: Network,
		) -> Result<(Signature, PublicKey)> {
			let psbt = &mut self.psbt;
			let mut device = device;
			if device.is_none() {
				let devices = HWIClient::enumerate()
					.map_err(|e| anyhow!("Error enumerating devices: {:?}", e))?;

				for d in devices.into_iter().flatten() {
					device = Some(d);
				}
			};
			let device = device.ok_or(anyhow!("No device found"))?;

			let client = HWIClient::get_client(&device, false, network.into())?;
			let x_pubkey = client.get_xpub(&key_source.1, false)?;
			let pubkey = x_pubkey.public_key;

			psbt.inputs[0].bip32_derivation.insert(pubkey, key_source);

			psbt.combine(client.sign_tx(psbt)?.psbt)?;
			let Some((_, signature)) =
				psbt.inputs[0].partial_sigs.iter().find(|(k, _)| k.inner == pubkey)
			else {
				bail!("Could not sign with hardware wallet");
			};

			Ok((*signature, pubkey.into()))
		}
	}
}
