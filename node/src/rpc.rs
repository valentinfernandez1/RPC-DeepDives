//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use node_template_runtime::{opaque::Block, AccountId, Balance, Index};
use pallet_template_rpc::{TemplateApiServer, TemplatePallet};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_template_rpc::TemplateRuntimeApi<Block>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	module.merge(SillyRpcServer::into_rpc(Silly::new(client.clone())))?;
	module.merge(TemplateApiServer::into_rpc(TemplatePallet::new(client)))?;

	Ok(module)
}

use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};

#[rpc(client, server)]
pub trait SillyRpc {
	///Returns 5
	#[method(name = "silly_get5")]
	fn silly_get_5(&self) -> RpcResult<u64>;

	///Checks if the received block has already been finalized
	#[method(name = "silly_isBlockFinalized")]
	fn is_block_finalized(&self, is_finalized: u64) -> RpcResult<bool>;
}

///Silly rpc test
pub struct Silly<C> {
	client: Arc<C>,
}

impl<C> Silly<C> {
	/// Create new `Silly` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

impl<C> SillyRpcServer for Silly<C>
where
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
{
	fn silly_get_5(&self) -> Result<u64, JsonRpseeError> {
		Ok(5)
	}

	fn is_block_finalized(&self, is_finalized: u64) -> RpcResult<bool> {
		let last_finalized: u64 = self.client.info().finalized_number.into();
		Ok(is_finalized <= last_finalized)
	}
}