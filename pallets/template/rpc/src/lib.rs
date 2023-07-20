use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
pub use pallet_template_runtime_api::TemplateApi as TemplateRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

#[rpc(client, server)]
pub trait TemplateApi<BlockHash> {
	#[method(name = "template_sum5")]
	fn sum_5(&self, at: Option<BlockHash>) -> RpcResult<u32>;

	#[method(name = "template_sumAndStore")]
	fn sum_and_store(&self, at: Option<BlockHash>, value: u32) -> RpcResult<u32>;
}

/// A struct that implements the `TemplateApi`.
pub struct TemplatePallet<C, Block> {
	// If you have more generics, no need to TemplatePallet<C, M, N, P, ...>
	// just use a tuple like TemplatePallet<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TemplatePallet<C, Block> {
	/// Create new `TemplatePallet` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block> TemplateApiServer<<Block as BlockT>::Hash> for TemplatePallet<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: TemplateRuntimeApi<Block>,
{
	fn sum_5(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);
		self.client.runtime_api().sum_5(at_hash).map_err(runtime_error_into_rpc_err)
	}

	fn sum_and_store(&self, at: Option<<Block as BlockT>::Hash>, value: u32) -> RpcResult<u32> {
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);
		self.client
			.runtime_api()
			.sum_and_store(at_hash, value)
			.map_err(runtime_error_into_rpc_err)
	}
}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Runtime error",
		Some(format!("{:?}", err)),
	))
	.into()
}
