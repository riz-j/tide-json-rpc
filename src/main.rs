#![allow(dead_code, unused)]

pub mod prelude;
use prelude::*;

use rpc_router::{
    CallError, CallResponse, FromResources, Handler, Request, Resources, Router, RpcParams,
};
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

pub async fn print_hello(ctx: RequestContext, params: TaskCreate) -> Result<()> {
    return Err(Error::FailedOperation);

    println!("Hello, {}!", ctx.username);
    Ok(())
}

#[async_std::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let rpc_router = Router::builder()
        .append("create_task", create_task)
        .append("print_hello", print_hello)
        .build();

    let rpc_request: Request = json!({
        "jsonrpc": "2.0",
        "id": null, // the json rpc id, that will get echoed back, can be null
        "method": "print_hello",
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
        .await;

    match result {
        Ok(result) => println!(
            "ID: {}\nMethod: {}\nValue: {}",
            result.id, result.method, result.value
        ),
        Err(call_error) => {
            println!(
                "Error for request id: {}, method: {}",
                call_error.id, call_error.method
            );
            match call_error.error {
                // It's a application handler type wrapped in a rpc_router CallError
                // we need to know it's type to extract it.
                rpc_router::Error::Handler(mut handler_error) => {
                    // We can remove it not needed anymore
                    if let Some(my_error) = handler_error.remove::<crate::prelude::Error>() {
                        println!("Error: {my_error:?}")
                    } else {
                        println!("Unhandled App Error: {handler_error:?}");
                    }
                }
                // if it is other rpc_router error, can be handled normally
                other => println!("{other:?}"),
            }
        }
    };

    Ok(())
}
