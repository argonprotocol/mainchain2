use std::{sync::Arc, time::Duration};

use crate::{aux_client::ArgonAux, error::Error, notary_client::get_notebook_header_data};
use argon_bitcoin_utxo_tracker::{get_bitcoin_inherent, UtxoTracker};
use argon_node_runtime::{NotaryRecordT, NotebookVerifyError};
use argon_primitives::{
	inherents::{
		BitcoinInherentDataProvider, BlockSealInherentDataProvider, BlockSealInherentNodeSide,
		NotebooksInherentDataProvider,
	},
	tick::Tick,
	Balance, BestBlockVoteSeal, BitcoinApis, BlockSealApis, BlockSealAuthorityId, BlockSealDigest,
	Digestset, NotaryApis, NotebookApis, TickApis, TickDigest, VotingSchedule,
};
use codec::Codec;
use frame_support::CloneNoBound;
use log::*;
use sc_client_api::AuxStore;
use sc_consensus::{BlockImport, BlockImportParams, ImportResult, StateAction, StorageChanges};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{BlockOrigin, Environment, Proposal, Proposer};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT},
	Digest,
};
use sp_timestamp::Timestamp;
use tokio::sync::Mutex;

pub struct CreateTaxVoteBlock<Block: BlockT, AccountId: Clone + Codec> {
	pub current_tick: Tick,
	pub timestamp_millis: u64,
	pub parent_hash: Block::Hash,
	pub vote: BestBlockVoteSeal<AccountId, BlockSealAuthorityId>,
}

#[derive(CloneNoBound)]
pub struct BlockCreator<Block: BlockT, BI: Clone, Client: AuxStore, PF, JS: Clone, A: Clone> {
	/// The block author,
	pub author: A,
	/// Used to actually import blocks.
	pub block_import: BI,
	/// The underlying para client.
	pub client: Arc<Client>,
	/// The underlying block proposer this should call into.
	pub proposer: Arc<Mutex<PF>>,
	/// The amount of time to spend authoring each block.
	pub authoring_duration: Duration,
	pub justification_sync_link: JS,
	pub aux_client: ArgonAux<Block, Client>,
	pub utxo_tracker: Arc<UtxoTracker>,
}

impl<Block: BlockT, BI, C, PF, JS, A, Proof> BlockCreator<Block, BI, C, PF, JS, A>
where
	Block: BlockT + 'static,
	Block::Hash: Send + 'static,
	BI: BlockImport<Block> + Clone + Send + Sync + 'static,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + AuxStore + 'static,
	C::Api: NotebookApis<Block, NotebookVerifyError>
		+ BlockSealApis<Block, A, BlockSealAuthorityId>
		+ NotaryApis<Block, NotaryRecordT>
		+ TickApis<Block>
		+ BitcoinApis<Block, Balance>,
	PF: Environment<Block> + Send + Sync + 'static,
	PF::Proposer: Proposer<Block, Proof = Proof>,
	A: Codec + Clone + Send + Sync + 'static,
	JS: sc_consensus::JustificationSyncLink<Block> + Clone + Send + Sync + 'static,
{
	pub async fn propose(
		&self,
		submitting_tick: Tick,
		timestamp_millis: u64,
		parent_hash: Block::Hash,
		seal_inherent: BlockSealInherentNodeSide,
	) -> Option<BlockProposal<Block, Proof>> {
		let parent_header = match self.client.header(parent_hash) {
			Ok(Some(x)) => x,
			Ok(None) => {
				tracing::warn!("Parent header not found {:?}", parent_hash);
				return None
			},
			Err(err) => {
				tracing::error!(?err, ?parent_hash, "Error while fetching parent header");
				return None
			},
		};

		let (inherent_data, inherent_digest) = self
			.create_inherents(parent_hash, submitting_tick, timestamp_millis, seal_inherent)
			.await
			.ok()?;

		let mut proposer = self.proposer.lock().await;
		let proposer: PF::Proposer = match proposer.init(&parent_header).await {
			Ok(x) => x,
			Err(err) => {
				tracing::warn!(?err, "Unable to propose. Creating proposer failed");
				return None;
			},
		};
		let size_limit = None;
		let proposal = proposer
			.propose(inherent_data, inherent_digest, self.authoring_duration, size_limit)
			.await
			.inspect_err(|err| {
				tracing::warn!(?err, "Unable to propose. Creating proposer failed");
			})
			.ok()?;

		Some(BlockProposal { proposal })
	}

	pub async fn create_inherents(
		&self,
		parent_hash: Block::Hash,
		submitting_tick: Tick,
		timestamp_millis: u64,
		seal_inherent: BlockSealInherentNodeSide,
	) -> Result<(InherentData, Digest), Error> {
		let voting_schedule = VotingSchedule::when_creating_block(submitting_tick);
		let notebook_header_data = get_notebook_header_data(
			&self.client,
			&self.aux_client,
			&parent_hash,
			&voting_schedule,
		)
		.await
		.inspect_err(|err| {
			tracing::warn!(?err, "Unable to get inherent data");
		})?;

		info!(
			"Proposing block at tick {} with {} notebooks",
			submitting_tick,
			notebook_header_data.notebook_digest.notebooks.len()
		);

		let timestamp = sp_timestamp::InherentDataProvider::new(Timestamp::new(timestamp_millis));
		let seal =
			BlockSealInherentDataProvider { seal: Some(seal_inherent.clone()), digest: None };
		let notebooks =
			NotebooksInherentDataProvider { raw_notebooks: notebook_header_data.signed_headers };

		let mut inherent_data =
			(timestamp, seal, notebooks).create_inherent_data().await.inspect_err(|err| {
				tracing::warn!(
					?err,
					"Unable to propose new block for authoring. Creating inherent data failed",
				);
			})?;

		let bitcoin_utxo_sync =
			get_bitcoin_inherent(&self.utxo_tracker, &self.client, &parent_hash).unwrap_or_else(
				|err| {
					tracing::warn!(?err, "Unable to get bitcoin inherent");
					None
				},
			);
		if let Some(bitcoin_utxo_sync) = bitcoin_utxo_sync {
			BitcoinInherentDataProvider { bitcoin_utxo_sync }
				.provide_inherent_data(&mut inherent_data)
				.await
				.inspect_err(|err| {
					tracing::warn!(?err, "Unable to provide bitcoin inherent data");
				})?;
		}

		let inherent_digest = Digestset {
			author: self.author.clone(),
			tick: TickDigest { tick: submitting_tick },
			block_vote: notebook_header_data.vote_digest,
			notebooks: notebook_header_data.notebook_digest,
			voting_key: Default::default(),
		}
		.create_pre_runtime_digest();

		Ok((inherent_data, inherent_digest))
	}

	pub async fn submit_block(
		&self,
		block_proposal: BlockProposal<Block, Proof>,
		block_seal_digest: BlockSealDigest,
	) {
		let BlockProposal { proposal } = block_proposal;

		let (pre_header, body) = proposal.block.deconstruct();
		let pre_hash = pre_header.hash();
		let parent_hash = *pre_header.parent_hash();
		let block_number = *pre_header.number();

		// seal the block.
		let seal = block_seal_digest.to_digest();
		let mut block_import_params = BlockImportParams::new(BlockOrigin::Own, pre_header);

		block_import_params.post_digests.push(seal);
		block_import_params.body = Some(body.clone());
		block_import_params.state_action =
			StateAction::ApplyChanges(StorageChanges::Changes(proposal.storage_changes));
		let post_hash = block_import_params.post_hash();

		tracing::info!(
			"🔖 Pre-sealed block for proposal at {}. Hash now {:?}, previously {:?}.",
			block_number,
			post_hash,
			pre_hash,
		);

		match self.block_import.import_block(block_import_params).await {
			Ok(res) => match res {
				ImportResult::Imported(_) => {
					res.handle_justification(
						&post_hash,
						block_number,
						&self.justification_sync_link,
					);
					tracing::info!(
						"✅ Successfully mined block on top of: {} -> {}",
						parent_hash,
						post_hash
					);
				},
				other => {
					warn!("Import of own block - result not success: {:?}", other);
				},
			},
			Err(e) => {
				tracing::error!(?e, "Failed to produce candidate");
			},
		}
	}
}

pub struct BlockProposal<Block: BlockT, Proof> {
	pub proposal: Proposal<Block, Proof>,
}
