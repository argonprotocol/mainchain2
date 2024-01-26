use std::{collections::BTreeMap, default::Default, marker::PhantomData, sync::Arc};

use codec::Codec;
use log::{info, trace};
use sc_client_api::AuxStore;
use sc_utils::mpsc::TracingUnboundedSender;
use sp_api::ProvideRuntimeApi;
use sp_core::blake2_256;
use sp_runtime::traits::Block as BlockT;
use tokio::sync::Mutex;

use ulx_node_runtime::{NotaryRecordT, NotebookVerifyError};
use ulx_notary::apis::notebook::{NotebookRpcClient, RawHeadersSubscription};
use ulx_primitives::{
	notary::NotaryNotebookVoteDetails, notebook::NotebookNumber, tick::Tick, BlockNumber,
	BlockSealApis, BlockSealAuthorityId, NotaryApis, NotaryId, NotebookApis, NotebookDigest,
	NotebookHeaderData,
};

use crate::{
	aux::{NotebookAuditResult, UlxAux},
	error::Error,
};

pub struct NotaryClient<B: BlockT, C: AuxStore, AC> {
	client: Arc<C>,
	pub notary_client_by_id: Arc<Mutex<BTreeMap<NotaryId, Arc<ulx_notary::Client>>>>,
	pub notaries_by_id: Arc<Mutex<BTreeMap<NotaryId, NotaryRecordT>>>,
	pub subscriptions_by_id: Arc<Mutex<BTreeMap<NotaryId, RawHeadersSubscription>>>,
	header_stream: TracingUnboundedSender<(NotaryId, NotebookNumber, Vec<u8>)>,
	aux_client: UlxAux<B, C>,
	_block: PhantomData<AC>,
}

const LOG_TARGET: &str = "node::notary_client";

impl<B, C, AC> NotaryClient<B, C, AC>
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + AuxStore,
	C::Api: NotaryApis<B, NotaryRecordT>
		+ NotebookApis<B, NotebookVerifyError>
		+ BlockSealApis<B, AC, BlockSealAuthorityId>,
	AC: Clone + Codec,
{
	pub fn new(
		client: Arc<C>,
		aux_client: UlxAux<B, C>,
		header_stream: TracingUnboundedSender<(NotaryId, NotebookNumber, Vec<u8>)>,
	) -> Self {
		Self {
			client,
			subscriptions_by_id: Default::default(),
			notary_client_by_id: Default::default(),
			notaries_by_id: Default::default(),
			header_stream,
			aux_client,
			_block: PhantomData,
		}
	}

	pub async fn update_notaries(&self, block_hash: &B::Hash) -> Result<(), Error<B>> {
		let mut needs_connect = vec![];
		{
			let notaries = self.client.runtime_api().notaries(block_hash.clone())?;
			let mut clients = self.notary_client_by_id.lock().await;
			let mut notaries_by_id = self.notaries_by_id.lock().await;
			let next_notaries_by_id =
				notaries.iter().map(|n| (n.notary_id, n.clone())).collect::<BTreeMap<_, _>>();

			if next_notaries_by_id != *notaries_by_id {
				let mut subscriptions_by_id = self.subscriptions_by_id.lock().await;
				for notary in notaries {
					if let Some(existing) = notaries_by_id.get(&notary.notary_id) {
						if existing.meta_updated_block < notary.meta_updated_block ||
							!clients.contains_key(&notary.notary_id)
						{
							// need to reconnect
							needs_connect.push(notary.notary_id);
						}
					} else {
						// need to connect
						needs_connect.push(notary.notary_id);
					}
				}
				clients.retain(|id, _| {
					if notaries_by_id.contains_key(id) {
						return true;
					}

					subscriptions_by_id.remove(&id);
					false
				});
				*notaries_by_id = next_notaries_by_id;
				info!(target: LOG_TARGET, "Notaries updated. {} notaries. {} need to connect.", &notaries_by_id.len(), &needs_connect.len());
			} else {
				for notary in notaries {
					if !clients.contains_key(&notary.notary_id) {
						needs_connect.push(notary.notary_id);
					}
				}
				if !needs_connect.is_empty() {
					info!(target: LOG_TARGET, "Notaries unchanged. {} need to re-connect.", &needs_connect.len());
				}
			}
		}
		if needs_connect.is_empty() {
			return Ok(());
		}

		for id in needs_connect {
			match self.sync_notebooks(id).await {
				Err(e) => {
					self.disconnect(&id, Some(format!("Notary {} sync failed. {:?}", id, e))).await;
				},
				_ => {},
			}
			match self.subscribe_to_notebooks(id).await {
				Err(e) => {
					self.disconnect(
						&id,
						Some(format!("Notary {} subscription failed. {:?}", id, e)),
					)
					.await;
				},
				_ => {},
			}
		}

		Ok(())
	}

	async fn sync_notebooks(&self, id: NotaryId) -> Result<(), Error<B>> {
		let client = self.get_client(id).await?;
		let notebook_meta = client.metadata().await.map_err(|e| {
			Error::NotaryError(format!("Could not get notebooks from notary - {:?}", e))
		})?;
		let notary_notebooks = self.aux_client.get_notary_audit_history(id)?;
		let latest_stored = notary_notebooks.last().map(|n| n.notebook_number).unwrap_or_default();
		if latest_stored < notebook_meta.finalized_notebook_number.saturating_sub(1) {
			let catchup = client.get_raw_headers(latest_stored).await.map_err(|e| {
				Error::NotaryError(format!("Could not get notebooks from notary - {:?}", e))
			})?;
			for (notebook_number, header) in catchup {
				self.header_stream.unbounded_send((id, notebook_number, header)).map_err(|e| {
					Error::NotaryError(format!("Could not send header to stream - {:?}", e))
				})?;
			}
		}

		Ok(())
	}

	pub async fn disconnect(&self, notary_id: &NotaryId, reason: Option<String>) {
		let mut clients = self.notary_client_by_id.lock().await;
		info!(target: LOG_TARGET, "Notary client disconnected from notary #{} (or could not connect). Reason? {:?}", notary_id, reason);
		if !clients.contains_key(notary_id) {
			return;
		}
		clients.remove(&notary_id);
		let mut subs = self.subscriptions_by_id.lock().await;
		drop(subs.remove(&notary_id));
	}

	async fn subscribe_to_notebooks(&self, id: NotaryId) -> Result<(), Error<B>> {
		let client = self.get_client(id).await?;
		let stream = client.subscribe_raw_headers().await.map_err(|e| {
			Error::NotaryError(format!("Could not subscribe to notebooks from notary - {:?}", e))
		})?;
		let mut subs = self.subscriptions_by_id.lock().await;
		subs.insert(id, stream);
		Ok(())
	}

	pub async fn try_audit_notebook(
		&self,
		block_hash: &B::Hash,
		vote_details: &NotaryNotebookVoteDetails<B::Hash>,
	) -> Result<NotebookAuditResult, Error<B>> {
		let notary_id = vote_details.notary_id;
		let notebook_number = vote_details.notebook_number;

		info!(
			target: LOG_TARGET,
			"Attempting to audit notebook. Notary {}, #{}.",
			notary_id,
			notebook_number);

		let full_notebook = self.download_notebook(notary_id, notebook_number).await?;

		trace!(
			target: LOG_TARGET,
			"Notebook downloaded. Notary {}, #{}. {} bytes.",
			notary_id,
			notebook_number,
			full_notebook.len()
		);
		let mut vote_minimums = BTreeMap::new();
		for block_hash in &vote_details.blocks_with_votes {
			vote_minimums.insert(
				block_hash.clone(),
				self.client.runtime_api().vote_minimum(block_hash.clone()).map_err(|e| {
					let message = format!(
						"Error getting vote minimums for block {}. Notary {}, notebook {}. {:?}",
						block_hash, notary_id, notebook_number, e
					);
					Error::<B>::StringError(message)
				})?,
			);
		}

		let mut audit_result = NotebookAuditResult {
			tick: vote_details.tick,
			notebook_number,
			is_valid: true,
			body_hash: blake2_256(&full_notebook),
			first_error_reason: None,
		};
		let mut vote_count = 0;
		// audit on the best block at the height of the notebook
		match self.client.runtime_api().audit_notebook_and_get_votes(
			block_hash.clone(),
			vote_details.version,
			notary_id,
			notebook_number,
			vote_details.header_hash.clone(),
			&vote_minimums,
			&full_notebook,
		)? {
			Ok(votes) => {
				vote_count = votes.raw_votes.len();
				self.aux_client.store_votes(vote_details.tick, votes)?;
			},
			Err(error) => {
				audit_result.is_valid = false;
				audit_result.first_error_reason = Some(error);
			},
		}

		trace!(
			target: LOG_TARGET,
			"Notebook audit result - {}. Notary {}, #{}. {} block vote(s).",
			match audit_result.is_valid {
				true => "Valid".to_string(),
				false => format!("Invalid - {:?}", audit_result.first_error_reason),
			},
			notary_id,
			notebook_number,
			vote_count
		);

		Ok(audit_result)
	}

	async fn get_client(&self, notary_id: NotaryId) -> Result<Arc<ulx_notary::Client>, Error<B>> {
		let mut clients = self.notary_client_by_id.lock().await;
		if !clients.contains_key(&notary_id) {
			let notaries = self.notaries_by_id.lock().await;
			let record = notaries.get(&notary_id).ok_or_else(|| {
				Error::NotaryError("No rpc endpoints found for notary".to_string())
			})?;
			let host = record.meta.hosts.get(0).ok_or_else(|| {
				Error::NotaryError("No rpc endpoint found for notary".to_string())
			})?;
			let c = ulx_notary::create_client(host.get_url().as_str()).await.map_err(|e| {
				Error::NotaryError(format!(
					"Could not connect to notary {} ({}) for audit - {:?}",
					notary_id,
					host.get_url(),
					e
				))
			})?;
			let c = Arc::new(c);
			clients.insert(notary_id, c.clone());
		}
		let client = clients.get(&notary_id).ok_or_else(|| {
			Error::NotaryError("Could not connect to notary for audit".to_string())
		})?;
		Ok(client.clone())
	}

	async fn download_notebook(
		&self,
		notary_id: NotaryId,
		notebook_number: NotebookNumber,
	) -> Result<Vec<u8>, Error<B>> {
		let client = self.get_client(notary_id).await?;

		match client.get_raw_body(notebook_number).await {
			Err(err) => {
				self.disconnect(&notary_id, Some(format!("Error downloading notebook: {}", err)))
					.await;
				return Err(Error::NotaryError(format!("Error downloading notebook: {}", err)));
			},
			Ok(body) => Ok(body),
		}
	}
}

pub async fn verify_notebook_audits<B: BlockT, C>(
	aux_client: &UlxAux<B, C>,
	notebook_digest: &NotebookDigest<NotebookVerifyError>,
) -> Result<(), Error<B>>
where
	C: AuxStore,
{
	let mut is_missing_entries = false;
	'retries: for _ in 0..10 {
		for digest_record in &notebook_digest.notebooks {
			let notary_audits = aux_client.get_notary_audit_history(digest_record.notary_id)?;

			match notary_audits
				.iter()
				.find(|a| a.notebook_number == digest_record.notebook_number)
			{
				Some(audit) =>
					if digest_record.audit_first_failure != audit.first_error_reason {
						return Err(Error::<B>::InvalidNotebookDigest(format!(
							"Notary {}, notebook #{} has an audit mismatch \"{:?}\" with local result. \"{:?}\"",
							digest_record.notary_id, digest_record.notebook_number, digest_record.audit_first_failure, audit.first_error_reason
						)))
					},
				None => {
					is_missing_entries = true;
					info!(
						target: LOG_TARGET,
						"Notebook digest record not found in local storage. Delaying to allow import. Notary {}, notebook #{}",
						digest_record.notary_id, digest_record.notebook_number);
					tokio::time::sleep(std::time::Duration::from_secs(1)).await;
					continue 'retries;
				},
			}
		}
		if !is_missing_entries {
			return Ok(());
		}
	}
	Err(Error::<B>::InvalidNotebookDigest(
		"Notebook digest record could not verify all records in local storage".to_string(),
	))
}

pub async fn get_notebook_header_data<B: BlockT, C, AccountId: Codec>(
	client: &Arc<C>,
	aux_client: &UlxAux<B, C>,
	best_hash: &B::Hash,
	submitting_tick: Tick,
) -> Result<NotebookHeaderData<NotebookVerifyError, BlockNumber>, Error<B>>
where
	C: ProvideRuntimeApi<B> + AuxStore,
	C::Api: NotebookApis<B, NotebookVerifyError>
		+ NotaryApis<B, NotaryRecordT>
		+ BlockSealApis<B, AccountId, BlockSealAuthorityId>,
{
	let latest_notebooks_in_runtime = client.runtime_api().latest_notebook_by_notary(*best_hash)?;
	let mut headers = NotebookHeaderData::default();
	let mut tick_notebooks = vec![];

	let notaries = client.runtime_api().notaries(*best_hash)?;
	for notary in notaries {
		let (latest_runtime_notebook_number, _) =
			latest_notebooks_in_runtime.get(&notary.notary_id).unwrap_or(&(0, 0));
		aux_client.get_notary_notebooks_for_header(
			notary.notary_id,
			*latest_runtime_notebook_number,
			submitting_tick,
			&mut headers.signed_headers,
			&mut tick_notebooks,
			&mut headers.notebook_digest.notebooks,
			&mut headers.latest_finalized_block_needed,
		)?;
	}

	headers.vote_digest =
		client
			.runtime_api()
			.create_vote_digest(*best_hash, submitting_tick, tick_notebooks)?;

	Ok(headers)
}
