#![allow(dead_code, unused)]

pub mod prelude;
use prelude::*;

use rpc_router::{FromResources, Handler, Request, Resources, Router, RpcParams};
use serde::Deserialize;
use serde_json::json;

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

pub async fn create_task(app: AppState, ctx: RequestContext, params: TaskCreate) -> Result<u32> {
    println!("Hello, {}!", ctx.user_id);
    Ok(25)
}

#[async_std::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let rpc_router = Router::builder()
        .append_dyn("increment_id", create_task.into_dyn())
        .build();

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
