use std::collections::{BTreeMap, BTreeSet};

use binary_merkle_tree::{merkle_proof, merkle_root, verify_proof, Leaf};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use sp_core::{
	bounded::BoundedVec,
	ed25519::{Public, Signature},
	Blake2Hasher, RuntimeDebug, H256,
};
use sp_keystore::KeystorePtr;
use sqlx::PgConnection;

use ulx_primitives::{
	ensure, AccountId, AccountOrigin, AccountType, Balance, BalanceTip, BlockVote, ChainTransfer,
	LocalchainAccountId, MaxNotebookNotarizations, MerkleProof, NewAccountOrigin, NotaryId, Note,
	NoteType, Notebook, NotebookNumber,
};

use crate::{
	notebook_closer::notary_sign,
	stores::{
		chain_transfer::ChainTransferStore, notarizations::NotarizationsStore,
		notebook_header::NotebookHeaderStore, notebook_new_accounts::NotebookNewAccountsStore,
		BoxFutureResult,
	},
	Error,
};

pub struct NotebookStore;
impl NotebookStore {
	/// Get proofs for a set of balance tips. This fn should retrieve from the database, not
	/// calculate.
	pub fn get_balance_proof<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		_notary_id: NotaryId,
		notebook_number: NotebookNumber,
		balance_tip: &'a BalanceTip,
	) -> BoxFutureResult<'a, MerkleProof> {
		Box::pin(async move {
			let rows = sqlx::query!(
				"SELECT change_merkle_leafs FROM notebooks WHERE notebook_number = $1 LIMIT 1",
				notebook_number as i32
			)
			.fetch_one(db)
			.await
			.map_err(|_| Error::NotebookNotFinalized)?;

			let merkle_leafs = rows.change_merkle_leafs;

			let record = balance_tip.encode();

			let index = merkle_leafs
				.iter()
				.position(|x| *x == record)
				.ok_or(Error::InvalidBalanceProofRequested)?;

			let proof = merkle_proof::<Blake2Hasher, _, _>(&merkle_leafs, index);

			Ok(MerkleProof {
				proof: BoundedVec::truncate_from(
					proof.proof.into_iter().map(|p| p.into()).collect(),
				),
				leaf_index: index as u32,
				number_of_leaves: merkle_leafs.len() as u32,
			})
		})
	}

	pub async fn get_account_origin(
		db: &mut PgConnection,
		account_id: AccountId,
		account_type: AccountType,
	) -> anyhow::Result<AccountOrigin, Error> {
		let origin = json!([
			{
				"accountId": account_id,
				"accountType": account_type
			}
		]);
		let result = sqlx::query!(
			r#"
			SELECT new_account_origins, notebook_number FROM notebooks
			WHERE new_account_origins @> $1::jsonb 
			ORDER BY notebook_number DESC LIMIT 1
			"#,
			origin
		)
		.fetch_one(db)
		.await
		.map_err(|_| Error::MissingAccountOrigin)?;

		let origins: Vec<NewAccountOrigin> = serde_json::from_value(result.new_account_origins)?;

		let origin = origins
			.iter()
			.find(|a| a.account_type == account_type && a.account_id == account_id)
			.ok_or(Error::MissingAccountOrigin)?;

		Ok(AccountOrigin {
			notebook_number: result.notebook_number as NotebookNumber,
			account_uid: origin.account_uid,
		})
	}

	pub async fn get_block_votes(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
	) -> anyhow::Result<Vec<BlockVote>, Error> {
		let votes_json = sqlx::query_scalar!(
			"SELECT block_votes FROM notebooks WHERE notebook_number = $1 LIMIT 1",
			notebook_number as i32
		)
		.fetch_one(db)
		.await
		.map_err(|_| Error::NotebookNotFinalized)?;

		let block_votes = from_value(votes_json)?;

		Ok(block_votes)
	}

	pub fn get_account_origins<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		notebook_number: NotebookNumber,
	) -> BoxFutureResult<'a, BoundedVec<NewAccountOrigin, MaxNotebookNotarizations>> {
		Box::pin(async move {
			let rows = sqlx::query!(
				"SELECT new_account_origins FROM notebooks WHERE notebook_number = $1 LIMIT 1",
				notebook_number as i32
			)
			.fetch_one(db)
			.await?;

			let new_account_origins: Vec<NewAccountOrigin> = from_value(rows.new_account_origins)?;

			Ok(BoundedVec::truncate_from(new_account_origins))
		})
	}

	pub async fn is_valid_proof<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		balance_tip: &'a BalanceTip,
		notebook_number: NotebookNumber,
		notebook_proof: &'a MerkleProof,
	) -> anyhow::Result<bool, Error> {
		let root = NotebookHeaderStore::get_changed_accounts_root(db, notebook_number).await?;

		let is_valid = verify_proof::<Blake2Hasher, _, _>(
			&root,
			notebook_proof.proof.clone().into_inner(),
			notebook_proof.number_of_leaves as usize,
			notebook_proof.leaf_index as usize,
			Leaf::Value(&balance_tip.encode()),
		);

		Ok(is_valid)
	}

	pub async fn load_finalized(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
	) -> anyhow::Result<Notebook, Error> {
		let header = NotebookHeaderStore::load_with_signature(&mut *db, notebook_number).await?;

		let notarizations = NotarizationsStore::get_for_notebook(&mut *db, notebook_number).await?;

		let rows = sqlx::query!(
			"SELECT new_account_origins, hash, signature FROM notebooks WHERE notebook_number = $1 LIMIT 1",
			notebook_number as i32
		)
		.fetch_one(db)
		.await?;
		let new_account_origins = from_value(rows.new_account_origins)?;

		Ok(Notebook {
			header: header.header,
			hash: H256::from_slice(&rows.hash),
			signature: Signature::try_from(&rows.signature[..])
				.map_err(|e| Error::InternalError(format!("Unable to read signature: {:?}", e)))?,
			notarizations: BoundedVec::truncate_from(notarizations),
			new_account_origins: BoundedVec::truncate_from(new_account_origins),
		})
	}

	pub async fn load_raw(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
	) -> anyhow::Result<Vec<u8>, Error> {
		let rows = sqlx::query!(
			"SELECT encoded FROM notebooks_raw WHERE notebook_number = $1 LIMIT 1",
			notebook_number as i32
		)
		.fetch_one(db)
		.await?;

		let encoded = rows.encoded;

		Ok(encoded)
	}
	pub async fn save_raw(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
		bytes: Vec<u8>,
	) -> anyhow::Result<(), Error> {
		let res = sqlx::query!(
			"INSERT INTO notebooks_raw (notebook_number, encoded) VALUES ($1, $2)",
			notebook_number as i32,
			bytes.as_slice()
		)
		.execute(db)
		.await?;

		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to insert raw notebook".to_string())
		);

		Ok(())
	}

	pub async fn close_notebook(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
		finalized_block: u32,
		public: Public,
		keystore: &KeystorePtr,
	) -> anyhow::Result<(), Error> {
		let notarizations = NotarizationsStore::get_for_notebook(&mut *db, notebook_number).await?;

		let mut changed_accounts =
			BTreeMap::<LocalchainAccountId, (u32, Balance, AccountOrigin, Option<Note>)>::new();
		let mut block_votes = BTreeMap::<(AccountId, u32), BlockVote>::new();
		let new_account_origins =
			NotebookNewAccountsStore::take_notebook_origins(&mut *db, notebook_number).await?;

		let new_account_origin_map =
			BTreeMap::from_iter(new_account_origins.iter().map(|origin| {
				(
					LocalchainAccountId::new(
						origin.account_id.clone(),
						origin.account_type.clone(),
					),
					AccountOrigin { notebook_number, account_uid: origin.account_uid },
				)
			}));

		let mut voting_power = 0u128;
		let mut tax = 0u128;
		let mut blocks_with_votes = BTreeSet::new();
		let mut data_domains = Vec::new();
		// NOTE: rebuild transfers list so it matches the final order
		let mut transfers = vec![];
		for change in notarizations.clone() {
			for change in change.balance_changes {
				let account_id = change.account_id;
				let localchain_account_id =
					LocalchainAccountId::new(account_id.clone(), change.account_type);
				let origin = change
					.previous_balance_proof
					.map(|a| a.account_origin)
					.or_else(|| new_account_origin_map.get(&localchain_account_id).cloned())
					.ok_or(|| {
						Error::InternalError(format!(
							"Could not find origin for account {:?}",
							localchain_account_id
						))
					})
					.map_err(|e| Error::InternalError(e().to_string()))?;

				let mut change_note = None;
				for note in change.notes {
					match note.note_type {
						NoteType::Tax | NoteType::LeaseDomain => tax += note.milligons,
						NoteType::EscrowHold { .. } => change_note = Some(note.clone()),
						NoteType::EscrowSettle { .. } => change_note = None,
						NoteType::ClaimFromMainchain { account_nonce } =>
							transfers.push(ChainTransfer::ToLocalchain {
								account_nonce,
								account_id: account_id.clone(),
							}),
						NoteType::SendToMainchain => transfers.push(ChainTransfer::ToMainchain {
							account_id: account_id.clone(),
							amount: note.milligons,
						}),
						_ => {},
					}
				}

				if !changed_accounts.contains_key(&localchain_account_id) ||
					changed_accounts
						.get(&localchain_account_id)
						.is_some_and(|a| a.0 < change.change_number)
				{
					changed_accounts.insert(
						localchain_account_id.clone(),
						(change.change_number, change.balance, origin, change_note),
					);
				}
			}
			for vote in change.block_votes {
				let block_hash = vote.block_hash.clone();
				let key = (vote.account_id.clone(), vote.index.clone());
				voting_power += vote.power;
				block_votes.insert(key, vote);
				blocks_with_votes.insert(block_hash);
			}
			for domain in change.data_domains {
				data_domains.push(domain);
			}
		}

		let mut account_changelist = vec![];
		let merkle_leafs = changed_accounts
			.into_iter()
			.map(|(localchain_account_id, (nonce, balance, account_origin, escrow_hold_note))| {
				account_changelist.push(account_origin.clone());
				BalanceTip {
					account_id: localchain_account_id.account_id,
					account_type: localchain_account_id.account_type,
					change_number: nonce,
					balance,
					account_origin,
					escrow_hold_note,
				}
				.encode()
			})
			.collect::<Vec<_>>();

		let changes_root = merkle_root::<Blake2Hasher, _>(&merkle_leafs);

		let final_votes = block_votes.clone();

		let votes_merkle_leafs =
			block_votes.into_iter().map(|(_, vote)| vote.encode()).collect::<Vec<_>>();
		let votes_root = merkle_root::<Blake2Hasher, _>(&votes_merkle_leafs);

		let _ = ChainTransferStore::take_for_notebook(&mut *db, notebook_number).await?;

		NotebookHeaderStore::complete_notebook(
			&mut *db,
			notebook_number,
			finalized_block,
			transfers,
			data_domains,
			tax,
			changes_root,
			account_changelist,
			votes_root,
			votes_merkle_leafs.len() as u32,
			blocks_with_votes,
			voting_power,
			|hash| {
				notary_sign(&keystore, &public, &hash)
					.map_err(|e| Error::InternalError(format!("Unable to sign notebook: {:?}", e)))
			},
		)
		.await?;

		let new_account_origins = new_account_origins
			.iter()
			.map(|a| NewAccountOrigin {
				account_id: a.account_id.clone(),
				account_type: a.account_type.clone(),
				account_uid: a.account_uid,
			})
			.collect::<Vec<NewAccountOrigin>>();

		let final_header = NotebookHeaderStore::load(&mut *db, notebook_number).await?;
		let origins_json = json!(new_account_origins);

		let mut full_notebook = Notebook::build(final_header, notarizations, new_account_origins);
		let hash = full_notebook.hash;
		full_notebook.signature = notary_sign(&keystore, &public, &hash)?;

		let raw_body = full_notebook.encode();
		Self::save_raw(db, notebook_number, raw_body).await?;
		let votes_json = json!(final_votes.values().into_iter().collect::<Vec<_>>());

		let res = sqlx::query!(
			r#"
				INSERT INTO notebooks (notebook_number, change_merkle_leafs, new_account_origins, block_votes, hash, signature) 
				VALUES ($1, $2, $3, $4, $5, $6)
			"#,
			notebook_number as i32,
			merkle_leafs.as_slice(),
			origins_json,
			votes_json,
			hash.as_bytes(),
			&full_notebook.signature.0[..]
		)
		.execute(db)
		.await?;
		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to insert notebook".to_string())
		);

		Ok(())
	}
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Serialize, Deserialize)]
struct AccountIdAndOrigin {
	pub key: [u8; 32],
	pub origin: AccountOrigin,
}
#[cfg(test)]
mod tests {
	use std::ops::Add;

	use chrono::{Duration, Utc};
	use sp_core::{bounded_vec, ed25519::Signature};
	use sp_keyring::{
		AccountKeyring::{Alice, Dave},
		Sr25519Keyring::Bob,
	};
	use sp_keystore::{testing::MemoryKeystore, Keystore};
	use sqlx::PgPool;

	use ulx_primitives::{
		AccountOrigin, AccountType::Deposit, BalanceChange, BalanceTip, NewAccountOrigin,
	};

	use crate::{
		notebook_closer::NOTARY_KEYID,
		stores::{
			chain_transfer::ChainTransferStore, notarizations::NotarizationsStore,
			notebook::NotebookStore, notebook_header::NotebookHeaderStore,
			notebook_new_accounts::NotebookNewAccountsStore, registered_key::RegisteredKeyStore,
		},
	};

	#[sqlx::test]
	async fn test_close_notebook(pool: PgPool) -> anyhow::Result<()> {
		// Initialize the logger
		let _ = tracing_subscriber::fmt::try_init();
		let keystore = MemoryKeystore::new();
		let public = keystore.ed25519_generate_new(NOTARY_KEYID, None)?;

		let mut tx = pool.begin().await?;
		RegisteredKeyStore::store_public(&mut *tx, public, 1).await?;
		NotebookHeaderStore::create(
			&mut *tx,
			1,
			1,
			1,
			Utc::now().add(Duration::minutes(1)).timestamp_millis() as u64,
		)
		.await?;
		ChainTransferStore::record_transfer_to_local_from_block(
			&mut *tx,
			100,
			&Bob.to_account_id(),
			1,
			1000,
		)
		.await?;
		ChainTransferStore::take_and_record_transfer_local(
			&mut *tx,
			1,
			&Bob.to_account_id(),
			1,
			1000,
			0,
			0,
		)
		.await?;
		NotarizationsStore::append_to_notebook(
			&mut *tx,
			1,
			vec![
				BalanceChange {
					account_id: Bob.to_account_id(),
					account_type: Deposit,
					change_number: 1,
					balance: 1000,
					previous_balance_proof: None,
					notes: bounded_vec![],
					escrow_hold_note: None,
					signature: Signature([0u8; 64]).into(),
				},
				BalanceChange {
					account_id: Alice.to_account_id(),
					account_type: Deposit,
					change_number: 1,
					balance: 2500,
					previous_balance_proof: None,
					notes: bounded_vec![],
					escrow_hold_note: None,
					signature: Signature([0u8; 64]).into(),
				},
				BalanceChange {
					account_id: Dave.to_account_id(),
					account_type: Deposit,
					change_number: 1,
					balance: 500,
					previous_balance_proof: None,
					notes: bounded_vec![],
					escrow_hold_note: None,
					signature: Signature([0u8; 64]).into(),
				},
			],
			vec![],
			vec![],
		)
		.await?;
		NotebookNewAccountsStore::insert_origin(&mut *tx, 1, &Bob.to_account_id(), &Deposit)
			.await?;
		NotebookNewAccountsStore::insert_origin(&mut *tx, 1, &Alice.to_account_id(), &Deposit)
			.await?;
		NotebookNewAccountsStore::insert_origin(&mut *tx, 1, &Dave.to_account_id(), &Deposit)
			.await?;
		tx.commit().await?;

		let mut tx = pool.begin().await?;
		NotebookStore::close_notebook(&mut *tx, 1, 1, public, &keystore.into()).await?;
		tx.commit().await?;

		let balance_tip = BalanceTip {
			account_id: Bob.to_account_id(),
			account_type: Deposit,
			change_number: 1,
			balance: 1000,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			escrow_hold_note: None,
		};
		let proof = NotebookStore::get_balance_proof(&pool, 1, 1, &balance_tip).await?;

		assert_eq!(proof.number_of_leaves, 3);

		assert_eq!(NotebookStore::is_valid_proof(&pool, &balance_tip, 1, &proof).await?, true);

		assert_eq!(
			NotebookStore::get_account_origins(&pool, 1).await?.into_inner(),
			vec![
				NewAccountOrigin::new(Bob.to_account_id(), Deposit, 1),
				NewAccountOrigin::new(Alice.to_account_id(), Deposit, 2),
				NewAccountOrigin::new(Dave.to_account_id(), Deposit, 3),
			]
		);

		let mut db = pool.acquire().await?;
		assert_eq!(
			NotebookStore::get_account_origin(&mut db, Bob.to_account_id(), Deposit).await?,
			AccountOrigin { notebook_number: 1, account_uid: 1 }
		);

		Ok(())
	}
}
