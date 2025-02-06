use anyhow::Context;
use clap::Subcommand;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use sp_runtime::{FixedPointNumber, FixedU128};
use subxt::dynamic::Value;

use argon_client::{
	api::{apis, constants, storage, tx, vaults::storage::types::vaults_by_id::VaultsById},
	conversion::from_api_fixed_u128,
	MainchainClient,
};
use argon_primitives::{bitcoin::SATOSHIS_PER_BITCOIN, KeystoreParams, VaultId};

use crate::{formatters::ArgonFormatter, vault_create};

#[derive(Subcommand, Debug)]
pub enum VaultCommands {
	/// Show vaults that can support the given amount of btc
	List {
		/// The amount of btc to lock
		#[clap(short, long, default_value = "1.0")]
		btc: f32,
	},
	/// Create a new vault
	Create {
		#[clap(flatten)]
		config: vault_create::VaultConfig,
		#[clap(flatten)]
		keypair: KeystoreParams,
	},
	/// List pending release requests (vault claim, cosign)
	PendingRelease {
		/// The vault id to list pending release requests
		#[clap(short, long)]
		vault_id: VaultId,
		#[clap(flatten)]
		keypair: KeystoreParams,
	},
}
impl VaultCommands {
	pub async fn process(self, rpc_url: String) -> anyhow::Result<()> {
		match self {
			VaultCommands::List { btc } => {
				let client = MainchainClient::from_url(&rpc_url)
					.await
					.context("Failed to connect to argon node")?;

				let best_block = client.best_block_hash().await?;

				let satoshis =
					FixedU128::from_float(btc as f64).saturating_mul_int(SATOSHIS_PER_BITCOIN);
				let Some(argons_needed) = client
					.live
					.runtime_api()
					.at(best_block)
					.call(apis().bitcoin_apis().market_rate(satoshis))
					.await?
				else {
					println!("No price conversion found in blockchain for bitcoin to argon");
					return Ok(());
				};

				println!("Showing for: {:#?} btc", btc);
				println!("Current mint value: {} argons", ArgonFormatter(argons_needed));

				// NOTE: the typegen doesn't work, so revert to dynamic storage
				let keys: Vec<Value> = vec![];
				let query = subxt::dynamic::storage("Vaults", "VaultsById", keys);
				let mut vaults = client.live.storage().at_latest().await?.iter(query).await?;

				let mut table = Table::new();
				table
					.load_preset(UTF8_FULL)
					.apply_modifier(UTF8_ROUND_CORNERS)
					.set_content_arrangement(ContentArrangement::Dynamic)
					.set_header(vec![
						"Id",
						"Available argons",
						"Bonded argons",
						"Added Securitization",
						"Fee",
					]);

				while let Some(Ok(kv)) = vaults.next().await {
					let vault: VaultsById = kv.value.as_type()?;
					let Some(vault_id) = kv.keys[0].as_u128().map(|a| a as VaultId) else {
						continue;
					};
					let bitcoin_argons_available =
						vault.bitcoin_argons.allocated - vault.bitcoin_argons.reserved;
					if bitcoin_argons_available >= argons_needed {
						let fee = vault.bitcoin_argons.base_fee +
							from_api_fixed_u128(vault.bitcoin_argons.annual_percent_rate)
								.saturating_mul_int(argons_needed);

						table.add_row(vec![
							vault_id.to_string(),
							ArgonFormatter(bitcoin_argons_available).to_string(),
							ArgonFormatter(vault.bitcoin_argons.reserved).to_string(),
							ArgonFormatter(vault.added_securitization_argons).to_string(),
							ArgonFormatter(fee).to_string(),
						]);
					}
				}

				if table.is_empty() {
					println!("No vaults found that can support {} btc", btc);
				} else {
					println!("{table}");
				}
			},
			VaultCommands::Create { config, keypair } => {
				let client = MainchainClient::from_url(&rpc_url)
					.await
					.context("Failed to connect to argon node")?;

				let is_sharing_enabled = client
					.live
					.constants()
					.at(&constants().vaults().enable_reward_sharing())
					.unwrap_or_default();
				let mut config = config;
				if !config
					.complete_prompt(keypair.keystore_path.is_some(), is_sharing_enabled)
					.await
				{
					return Ok(());
				}
				let call = tx().vaults().create(config.as_call_data());

				let url = client.create_polkadotjs_deeplink(&call)?;
				println!("Vault funds needed: {}", config.argons_needed());
				println!("Link to create transaction:\n\t{}", url);
			},
			VaultCommands::PendingRelease { vault_id, keypair: _ } => {
				let client = MainchainClient::from_url(&rpc_url)
					.await
					.context("Failed to connect to argon node")?;
				let call = storage().bitcoin_locks().locks_pending_release_by_utxo_id();
				let Some(pending) = client.fetch_storage(&call, None).await? else {
					println!("No pending cosign requests found");
					return Ok(());
				};
				let current_block = client.live.blocks().at_latest().await?.number();
				let mut table = Table::new();
				table
					.load_preset(UTF8_FULL)
					.apply_modifier(UTF8_ROUND_CORNERS)
					.set_content_arrangement(ContentArrangement::Dynamic)
					.set_header(vec![
						"Utxo Id",
						"Obligation Id",
						"Cosign Due Block",
						"Type",
						"Redemption Price",
					]);
				for (utxo_id, pending) in pending.0.iter() {
					if pending.vault_id != vault_id {
						continue;
					}
					table.add_row(vec![
						utxo_id.to_string(),
						pending.obligation_id.to_string(),
						pending.cosign_due_block.to_string(),
						"Cosign Request".to_string(),
						ArgonFormatter(pending.redemption_price).to_string(),
					]);
				}

				println!("Pending as of block #{:?}\n\nNOTE: does not include eligible for reclaim by vault.\n\n{table}", current_block);
			},
		}
		Ok(())
	}
}
