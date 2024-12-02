#[cfg(test)]
mod bitcoin;
#[cfg(test)]
mod localchain_transfer;
#[cfg(test)]
mod vote_mining;

#[cfg(test)]
pub(crate) mod utils {
	use argon_client::{
		api::{
			runtime_types::{
				argon_primitives::{block_seal, block_seal::RewardDestination},
				argon_runtime::{RuntimeCall, SessionKeys},
				sp_consensus_grandpa as grandpa,
			},
			storage,
			sudo::calls::types,
			tx,
		},
		signer::{Signer, Sr25519Signer},
		ArgonConfig, ArgonOnlineClient,
	};
	use argon_primitives::{prelude::*, BLOCK_SEAL_KEY_TYPE};
	use argon_testing::{ArgonTestNode, ArgonTestNotary};
	use sp_core::{crypto::key_types::GRANDPA, Pair};
	use sp_keyring::{AccountKeyring::Alice, Sr25519Keyring};
	use subxt::tx::TxInBlock;

	pub(crate) async fn transfer_mainchain(
		test_node: &ArgonTestNode,
		from: &Sr25519Signer,
		to: AccountId,
		amount: Balance,
		wait_for_finalized: bool,
	) -> anyhow::Result<TxInBlock<ArgonConfig, ArgonOnlineClient>> {
		let to_account_id = test_node.client.api_account(&to);
		let params = test_node.client.params_with_best_nonce(&from.account_id()).await?.build();
		test_node
			.client
			.submit_tx(
				&tx().balances().transfer_keep_alive(to_account_id.into(), amount),
				from,
				Some(params),
				wait_for_finalized,
			)
			.await
	}

	pub(crate) async fn create_active_notary(
		test_node: &ArgonTestNode,
	) -> anyhow::Result<ArgonTestNotary> {
		let test_notary = ArgonTestNotary::start(test_node, None).await?;
		let owner = test_node.client.api_account(&test_notary.operator.public().into());
		let ferdie_signer: Sr25519Signer = Sr25519Keyring::Ferdie.pair().into();
		// give ferdie signer base amount
		transfer_mainchain(test_node, &ferdie_signer, owner.into(), 1_000_000, false).await?;
		println!("Registering a notary operator");
		test_notary.register_operator(test_node).await?;

		println!("Sudo approving notary");
		let operator_account = test_node.client.api_account(&test_notary.operator.public().into());
		sudo(
			test_node,
			RuntimeCall::Notaries(
				argon_client::api::runtime_types::pallet_notaries::pallet::Call::activate {
					operator_account,
				},
			),
		)
		.await?;
		println!("Sudo approved notary");

		Ok(test_notary)
	}

	pub(crate) async fn sudo(
		test_node: &ArgonTestNode,
		call: types::sudo::Call,
	) -> anyhow::Result<TxInBlock<ArgonConfig, ArgonOnlineClient>> {
		let from = Sr25519Signer::new(Alice.pair());
		let client = test_node.client.clone();
		let params = client.params_with_best_nonce(&from.account_id()).await?.build();
		test_node
			.client
			.submit_tx(&tx().sudo().sudo(call), &from, Some(params), false)
			.await
	}

	pub(crate) async fn bankroll_miners(
		test_node: &ArgonTestNode,
		from: &Sr25519Signer,
		to: Vec<AccountId>,
		wait_for_finalized: bool,
	) -> anyhow::Result<TxInBlock<ArgonConfig, ArgonOnlineClient>> {
		let client = test_node.client.clone();
		let params = client.params_with_best_nonce(&from.account_id()).await?.build();

		let amount = mining_slot_ownership_needed(test_node).await?;

		let account_id: AccountId = from.account_id();
		let sugar_daddy_account_id = client.api_account(&account_id);
		let alice_balance = client
			.fetch_storage(&storage().ownership().account(sugar_daddy_account_id.clone()), None)
			.await?;
		println!("alice balance {:?}", alice_balance);

		let calls = to
			.iter()
			.map(|a| {
				let api_account_id = client.api_account(a);
				RuntimeCall::Ownership(
					argon_client::api::runtime_types::pallet_balances::pallet::Call::transfer_allow_death {
						dest: api_account_id.into(),
						value: amount,
					}
				)
			})
			.collect::<Vec<_>>();
		let ownership_transfer = client
			.submit_tx(&tx().utility().batch_all(calls), from, Some(params), wait_for_finalized)
			.await?;
		println!("ownership transfer {:?}", ownership_transfer.extrinsic_hash());
		Ok(ownership_transfer)
	}

	pub(crate) async fn mining_slot_ownership_needed(
		test_node: &ArgonTestNode,
	) -> anyhow::Result<Balance> {
		Ok(test_node
			.client
			.fetch_storage(&storage().mining_slot().ownership_bond_amount(), None)
			.await?
			.unwrap_or_default())
	}

	pub(crate) async fn register_miner(
		node: &ArgonTestNode,
		miner: Sr25519Keyring,
	) -> anyhow::Result<()> {
		let client = node.client.clone();
		let grandpa_seed = format!("{}//grandpa", miner.to_seed());
		let grandpa_public = node.insert_ed25519_keystore_key(GRANDPA, grandpa_seed).await?;
		let mining_seed = format!("{}//seal", miner.to_seed());
		let seal_public =
			node.insert_ed25519_keystore_key(BLOCK_SEAL_KEY_TYPE, mining_seed).await?;

		// how much ownership is needed
		let ownership_needed = client
			.fetch_storage(&storage().mining_slot().ownership_bond_amount(), None)
			.await?
			.unwrap();
		println!("ownership needed {:?}", ownership_needed);

		println!("Registering miner");
		let register = client
			.submit_tx(
				&tx().mining_slot().bid(
					None,
					RewardDestination::Owner,
					SessionKeys {
						grandpa: grandpa::app::Public(grandpa_public),
						block_seal_authority: block_seal::app::Public(seal_public),
					},
				),
				&Sr25519Signer::new(miner.pair()),
				None,
				true,
			)
			.await?;
		println!(
			"miner registered. ext hash: {:?}, block {:?}",
			register.extrinsic_hash(),
			register.block_hash()
		);
		Ok(())
	}
}
