use rpc_router::{FromResources, Request, Resources, Router, RpcHandlerError, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, RpcHandlerError)]
pub enum Error {
    NotFound,
    FailedOperation,
}

#[derive(Clone)]
pub struct AppState {}
impl FromResources for AppState {}

#[derive(Clone)]
pub struct RequestContext {
    pub user_id: u32,
    pub username: String,
}
impl FromResources for RequestContext {}

#[derive(Deserialize, RpcParams)]
pub struct TaskCreate {
    pub name: String,
    pub assignee: Option<u32>,
}

pub async fn print_hello(ctx: RequestContext, params: TaskCreate) -> Result<()> {
    println!(
        "Hello, {}! The name called is: {}",
        ctx.username, params.name
    );

    Ok(())
}

#[async_std::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let rpc_router = Router::builder().append("print_hello", print_hello).build();

    let rpc_request: Request = json!({
        "jsonrpc": "2.0",
        "id": null,
        "method": "print_hello",
        "params": {
            "name": "Joe Schmoe"
        }
    })
    .try_into()?;

    let rpc_resources = Resources::builder()
        .append(AppState {})
        .append(RequestContext {
            user_id: 21,
            username: "JimmyDeSanta25".to_string(),
        })
        .build();

    let result = rpc_router
        .call_with_resources(rpc_request, rpc_resources)
        .await
        .unwrap();

    println!(
        "ID: {}\nMethod: {}\nValue: {}",
        result.id, result.method, result.value
    );

    Ok(())
}
