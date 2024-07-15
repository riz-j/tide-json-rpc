use rpc_router::RpcHandlerError;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, RpcHandlerError)]
pub enum Error {}
