use codec::{Codec, Decode, Encode, MaxEncodedLen};
use frame_support::PalletError;
use scale_info::TypeInfo;
use sp_arithmetic::{traits::UniqueSaturatedInto, FixedPointNumber, FixedU128};
use sp_debug_derive::RuntimeDebug;
use sp_runtime::traits::AtLeast32BitUnsigned;

use crate::{
	bitcoin::{BitcoinCosignScriptPubkey, BitcoinHeight, BitcoinXPub, CompressedBitcoinPubkey},
	block_seal::RewardSharing,
	tick::Tick,
	BondId, RewardShare, VaultId,
};

pub trait BondedArgonsProvider {
	type Balance: Codec;
	type AccountId: Codec;

	#[allow(clippy::type_complexity)]
	/// Create bonded argons for a mining seat
	fn create_bonded_argons(
		vault_id: VaultId,
		account_id: Self::AccountId,
		amount: Self::Balance,
		bond_until_tick: Tick,
		modify_bond_id: Option<BondId>,
	) -> Result<(BondId, Option<RewardSharing<Self::AccountId>>, Self::Balance), BondError>;
	/// Return the obligation to the originator with a prorated refund
	fn cancel_obligation(bond_id: BondId) -> Result<(), BondError>;
}

pub trait BitcoinObligationProvider {
	type Balance: Codec + Copy + TypeInfo + MaxEncodedLen + Default + AtLeast32BitUnsigned;
	type AccountId: Codec;

	fn is_owner(vault_id: VaultId, account_id: &Self::AccountId) -> bool;

	/// Return the bond to the originator with a prorated refund
	fn cancel_bond(bond_id: BondId) -> Result<(), BondError>;

	/// Bonds the given amount of funds for the given vault. The fee is calculated based on the
	/// amount and the duration of the bond.
	fn create_bond(
		vault_id: VaultId,
		bond_account_id: &Self::AccountId,
		bond_type: BondType,
		amount: Self::Balance,
		bond_expiration: BondExpiration,
		ticks: Tick,
	) -> Result<Bond<Self::AccountId, Self::Balance>, BondError>;

	/// Recoup funds from the vault. This will be called if a vault does not move cosigned UTXOs in
	/// the appropriate timeframe. Steps are taken to repay the bitcoin holder at the market rate.
	///
	/// This will make the bonded account whole via funds from the vault in the following order:
	/// 1. From the bonded funds
	/// 2. From the allocated funds
	/// 3. From the securitized funds
	/// 4. TODO: From the ownership tokens
	///
	/// The funds will be returned to the bond.bonded_account_id
	///
	/// Returns the amount (still owed, repaid)
	fn compensate_lost_bitcoin(
		bond_id: BondId,
		market_rate: Self::Balance,
		unlock_amount_paid: Self::Balance,
	) -> Result<(Self::Balance, Self::Balance), BondError>;

	/// Burn the funds from the vault. This will be called if a vault moves a bitcoin utxo outside
	/// the system. It is assumed that the vault is in cahoots with the bonded account.
	fn burn_vault_bitcoin_funds(
		bond_id: BondId,
		amount_to_burn: Self::Balance,
	) -> Result<Bond<Self::AccountId, Self::Balance>, BondError>;

	fn create_utxo_script_pubkey(
		vault_id: VaultId,
		owner_pubkey: CompressedBitcoinPubkey,
		vault_claim_height: BitcoinHeight,
		open_claim_height: BitcoinHeight,
		current_height: BitcoinHeight,
	) -> Result<(BitcoinXPub, BitcoinXPub, BitcoinCosignScriptPubkey), BondError>;

	fn modify_pending_bitcoin_funds(
		vault_id: VaultId,
		amount: Self::Balance,
		remove_pending: bool,
	) -> Result<(), BondError>;
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, PalletError)]
pub enum BondError {
	BondNotFound,
	NoMoreBondIds,
	MinimumBondAmountNotMet,
	VaultClosed,
	/// There are too many bond or bond funds expiring in the given expiration block
	ExpirationAtBlockOverflow,
	AccountWouldBeBelowMinimum,
	InsufficientFunds,
	InsufficientVaultFunds,
	/// The vault does not have enough bitcoins to cover the mining bond
	InsufficientBitcoinsForMining,
	ExpirationTooSoon,
	NoPermissions,
	HoldUnexpectedlyModified,
	/// The hold could not be removed - it must have been modified
	UnrecoverableHold,
	VaultNotFound,
	/// No Vault public keys are available
	NoVaultBitcoinPubkeysAvailable,
	/// Unable to generate a new vault public key
	UnableToGenerateVaultBitcoinPubkey,
	/// Unable to decode the vault public key
	UnableToDecodeVaultBitcoinPubkey,
	/// The fee for this bond exceeds the amount of the bond, which is unsafe
	FeeExceedsBondAmount,
	/// Scripting for a bitcoin UTXO failed
	InvalidBitcoinScript,
	/// An internal processing error occurred that is too technical to be useful to the user
	InternalError,
	/// Event handler error
	EventHandlerError,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Vault<
	AccountId: Codec,
	Balance: Codec + Copy + MaxEncodedLen + Default + AtLeast32BitUnsigned + TypeInfo,
> {
	/// The account assigned to operate this vault
	pub operator_account_id: AccountId,
	/// The assignment and allocation of LockedBitcoins
	pub bitcoin_argons: VaultArgons<Balance>,
	/// The additional securitization percent that has been added to the vault (recoverable by
	/// bonder in case of fraud or theft)
	#[codec(compact)]
	pub added_securitization_percent: FixedU128,
	/// The amount of argons that have been securitized
	#[codec(compact)]
	pub added_securitization_argons: Balance,
	/// The assignment and allocation of BondedArgons
	pub bonded_argons: VaultArgons<Balance>,
	/// The percent of argon mining rewards (minted and mined, not including fees) that this vault
	/// "charges"
	#[codec(compact)]
	pub mining_reward_sharing_percent_take: RewardShare,
	/// If the vault is closed, no new bonds can be issued
	pub is_closed: bool,
	/// The terms that are pending to be applied to this vault at the given block number
	pub pending_terms: Option<(Tick, VaultTerms<Balance>)>,
	/// Any pending increase in bonded argons
	pub pending_bonded_argons: Option<(Tick, Balance)>,
	/// Bitcoins pending verification
	pub pending_bitcoins: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct VaultTerms<Balance: Codec + MaxEncodedLen + Clone + TypeInfo + PartialEq + Eq> {
	/// The annual percent rate per argon vaulted for bitcoin bonds
	#[codec(compact)]
	pub bitcoin_annual_percent_rate: FixedU128,
	/// The base fee for a bitcoin bond
	#[codec(compact)]
	pub bitcoin_base_fee: Balance,
	/// The annual percent rate per argon vaulted for bonded argons
	#[codec(compact)]
	pub bonded_argons_annual_percent_rate: FixedU128,
	/// A base fee for bonded argons
	#[codec(compact)]
	pub bonded_argons_base_fee: Balance,
	/// The percent of argon mining rewards (minted and mined, not including fees) that this vault
	/// "charges"
	#[codec(compact)]
	pub mining_reward_sharing_percent_take: FixedU128, // max 100, actual percent
}

impl<
		AccountId: Codec,
		Balance: Codec
			+ Copy
			+ MaxEncodedLen
			+ Default
			+ AtLeast32BitUnsigned
			+ MaxEncodedLen
			+ Clone
			+ TypeInfo
			+ PartialEq
			+ Eq,
	> Vault<AccountId, Balance>
{
	pub fn available_bonded_argons(&self) -> Balance {
		let free = self.bonded_argons.free_balance();
		let mut bitcoins_bonded =
			self.bitcoin_argons.reserved.saturating_sub(self.pending_bitcoins);
		if self.added_securitization_argons > Balance::zero() {
			let allowed_securities = bitcoins_bonded
				.saturating_mul(2u32.into())
				.min(self.added_securitization_argons);
			bitcoins_bonded = bitcoins_bonded.saturating_add(allowed_securities);
		}
		free.min(bitcoins_bonded)
	}

	pub fn get_added_securitization_needed(&self) -> Balance {
		let allocated = if self.is_closed {
			self.bitcoin_argons.reserved
		} else {
			self.bitcoin_argons.allocated
		};

		let argons = self
			.added_securitization_percent
			.saturating_mul_int::<u128>(allocated.unique_saturated_into());

		argons.unique_saturated_into()
	}

	pub fn mut_argons(&mut self, bond_type: &BondType) -> &mut VaultArgons<Balance> {
		match *bond_type {
			BondType::Mining => &mut self.bonded_argons,
			BondType::Bitcoin => &mut self.bitcoin_argons,
		}
	}

	pub fn argons(&self, bond_type: &BondType) -> &VaultArgons<Balance> {
		match *bond_type {
			BondType::Mining => &self.bonded_argons,
			BondType::Bitcoin => &self.bitcoin_argons,
		}
	}
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct VaultArgons<Balance: Codec + Copy + MaxEncodedLen + Default + AtLeast32BitUnsigned> {
	#[codec(compact)]
	pub annual_percent_rate: FixedU128,
	#[codec(compact)]
	pub allocated: Balance,
	#[codec(compact)]
	pub reserved: Balance,
	#[codec(compact)]
	pub base_fee: Balance,
}

impl<Balance> VaultArgons<Balance>
where
	Balance: Codec + Copy + MaxEncodedLen + Default + AtLeast32BitUnsigned,
{
	pub fn destroy_funds(&mut self, amount: Balance) -> Result<(), BondError> {
		if self.reserved < amount {
			return Err(BondError::InsufficientFunds);
		}
		self.reduce_allocated(amount);
		self.reduce_reserved(amount);
		Ok(())
	}

	pub fn destroy_allocated_funds(&mut self, amount: Balance) -> Result<(), BondError> {
		if self.allocated < amount {
			return Err(BondError::InsufficientFunds);
		}
		self.reduce_allocated(amount);
		Ok(())
	}

	pub fn reduce_allocated(&mut self, amount: Balance) {
		self.allocated = self.allocated.saturating_sub(amount);
	}

	pub fn reduce_reserved(&mut self, amount: Balance) {
		self.reserved = self.reserved.saturating_sub(amount);
	}
}

impl<Balance: Codec + Copy + MaxEncodedLen + Default + AtLeast32BitUnsigned> VaultArgons<Balance> {
	pub fn free_balance(&self) -> Balance {
		self.allocated.saturating_sub(self.reserved)
	}
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Bond<AccountId: Codec, Balance: Codec> {
	#[codec(compact)]
	pub bond_id: BondId,
	pub bond_type: BondType,
	#[codec(compact)]
	pub vault_id: VaultId,
	pub bonded_account_id: AccountId,
	#[codec(compact)]
	pub total_fee: Balance,
	#[codec(compact)]
	pub prepaid_fee: Balance,
	#[codec(compact)]
	pub amount: Balance,
	#[codec(compact)]
	pub start_tick: Tick,
	pub expiration: BondExpiration,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum BondExpiration {
	/// The bond will expire at the given tick
	AtTick(#[codec(compact)] Tick),
	/// The bond will expire at a bitcoin block height
	BitcoinBlock(#[codec(compact)] BitcoinHeight),
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum BondType {
	Mining,
	Bitcoin,
}
