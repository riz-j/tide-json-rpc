#![allow(dead_code, unused)]
use rpc_router::{
    FromResources, Handler, HandlerResult, IntoParams, Request, Resources, Router, RpcHandlerError,
    RpcParams,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, RpcHandlerError, Serialize)]
pub enum Error {}

#[derive(Clone)]
pub struct ModelManager {}
impl FromResources for ModelManager {}

#[derive(Clone)]
pub struct RequestContext {
    pub user_id: u32,
    pub org_id: u32,
}
impl FromResources for RequestContext {}

#[derive(Clone)]
pub struct AppState {}
impl FromResources for AppState {}

#[derive(Deserialize)]
pub struct ParamsIded {
    pub id: i64,
}
impl IntoParams for ParamsIded {}

#[derive(Deserialize, RpcParams)]
pub struct TaskCreate {
    pub name: String,
    pub assignee: Option<u32>,
}

pub async fn increment_id(app: AppState, ctx: RequestContext, params: TaskCreate) -> Result<i64> {
    println!("Hello there!, {}", ctx.user_id);
    Ok(35)
}

#[async_std::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    // -- Build the Router with the builder
    let rpc_router = Router::builder()
        // Minor optimization over `.append(...)` to avoid monomorphization
        .append_dyn("increment_id", increment_id.into_dyn())
        .build();

    // -- Build the reqeust
    let rpc_request: Request = json!({
        "jsonrpc": "2.0",
        "id": null, // the json rpc id, that will get echoed back, can be null
        "method": "increment_id",
        "params": {
            // "id": 425
            "name": "Joe Schmoe"
        }
    })
    .try_into()?;

    let app_state = AppState {};
    let request_context = RequestContext {
        user_id: 50131,
        org_id: 172259,
    };

    // -- Build the Resources for this call via the builer
    let rpc_resources = Resources::builder()
        .append(AppState {})
        .append(request_context)
        .build();

    // -- Execute
    let call_result = rpc_router
        .call_with_resources(rpc_request, rpc_resources)
        .await;

    // -- Display result
    match call_result {
        Ok(call_response) => println!(
            "Success: rpc-id {:?}, method: {}, returned value: {:?}",
            call_response.id, call_response.method, call_response.value
        ),
        Err(call_error) => println!(
            "Error: rpc-id {:?}, method: {}, error {:?}",
            call_error.id, call_error.method, call_error.error
        ),
        // To extract app error type, see code below (examples/c00-readme.md)
    }

    Ok(())
}
