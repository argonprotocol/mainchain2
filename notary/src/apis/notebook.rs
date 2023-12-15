use jsonrpsee::{core::SubscriptionResult, proc_macros::rpc, types::ErrorObjectOwned};
use ulx_primitives::{BalanceProof, BalanceTip, Notebook, NotebookHeader, NotebookNumber};

#[rpc(server, client, namespace = "notebook")]
pub trait NotebookRpc {
	/// EXPERIMENTAL: Get proofs for a set of accounts. Localchain wallets will normally do this
	/// themselves.
	#[method(name = "getBalanceChangeProof")]
	async fn get_balance_proof(
		&self,
		notebook_number: NotebookNumber,
		balance_tip: BalanceTip,
	) -> Result<BalanceProof, ErrorObjectOwned>;

	#[method(name = "getHeader")]
	async fn get_header(
		&self,
		notebook_number: NotebookNumber,
	) -> Result<NotebookHeader, ErrorObjectOwned>;

	#[method(name = "get")]
	async fn get(&self, notebook_number: NotebookNumber) -> Result<Notebook, ErrorObjectOwned>;

	#[method(name = "get_raw_body")]
	async fn get_raw_body(
		&self,
		notebook_number: NotebookNumber,
	) -> Result<Vec<u8>, ErrorObjectOwned>;

	/// Subscription to notebook completed
	#[subscription(name = "subscribeHeaders" => "notebookHeader", item = NotebookHeader)]
	async fn subscribe_headers(&self) -> SubscriptionResult;
}
