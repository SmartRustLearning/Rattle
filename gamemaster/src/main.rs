#![deny(rust_2018_idioms)]
use crate::sandbox::Sandbox;
mod sandbox;
mod logic;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use snafu::prelude::*;

//use futures::{sink::SinkExt, stream::StreamExt};
// use std::{collections::HashSet, sync::Mutex};
// use tokio::sync::broadcast;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ADDRESS: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() {
    // Enable info-level logging by default. env_logger's default is error only.
    // let env_logger_config = env_logger::Env::default().default_filter_or("info");
    // env_logger::Builder::from_env(env_logger_config).init();

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler));

    let addr = DEFAULT_ADDRESS
        .parse()
        .expect("Unable to parse socket addr!");

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html("Hello World!")
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::debug!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    let sb = Sandbox::new().await.context(SandboxCreationSnafu).unwrap();
                    println!("Code received!!: \n'''{:?}\n'''", t);
                    let req = sandbox::CompileRequest {
                        target: sandbox::CompileTarget::LlvmIr,
                        channel: sandbox::Channel::Stable,
                        crate_type: sandbox::CrateType::Binary,
                        mode: sandbox::Mode::Debug,
                        tests: false,
                        code: t.to_string(),
                        edition: None,
                        backtrace: false,
                    };
                    let res = sb.compile(&req);
                    // TODO: check if compilation was succesfull

                    let req = sandbox::ExecuteRequest {
                        channel: sandbox::Channel::Stable,
                        crate_type: sandbox::CrateType::Binary,
                        mode: sandbox::Mode::Debug,
                        tests: false,
                        code: t.to_string(),
                        edition: None,
                        backtrace: false,
                    };
                    let res = sb.execute(&req).await;

                    println!("{:?}", &res);

                    if socket
                        .send(Message::Text(
                            // String::from(res.unwrap().stdout)
                            serde_json::json!(res.unwrap())
                        ))
                        .await
                        .is_err()
                    {
                        println!("client disconnected");
                        return;
                    }
                    println!("tickp");
                }
                Message::Binary(_) => {
                    println!("client sent binary data");
                }
                Message::Ping(_) => {
                    println!("socket ping");
                }
                Message::Pong(_) => {
                    println!("socket pong");
                }
                Message::Close(_) => {
                    println!("client disconnected");
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Sandbox creation failed: {}", source))]
    SandboxCreation { source: sandbox::Error },
    #[snafu(display("Compilation operation failed: {}", source))]
    Compilation { source: sandbox::Error },
    #[snafu(display("Execution operation failed: {}", source))]
    Execution { source: sandbox::Error },
    #[snafu(display("Evaluation operation failed: {}", source))]
    Evaluation { source: sandbox::Error },
    #[snafu(display("Linting operation failed: {}", source))]
    Linting { source: sandbox::Error },
    #[snafu(display("Expansion operation failed: {}", source))]
    Expansion { source: sandbox::Error },
    #[snafu(display("Formatting operation failed: {}", source))]
    Formatting { source: sandbox::Error },
    #[snafu(display("Interpreting operation failed: {}", source))]
    Interpreting { source: sandbox::Error },
    #[snafu(display("Caching operation failed: {}", source))]
    Caching { source: sandbox::Error },
    #[snafu(display("Gist creation failed: {}", source))]
    GistCreation { source: octocrab::Error },
    #[snafu(display("Gist loading failed: {}", source))]
    GistLoading { source: octocrab::Error },
    #[snafu(display("Unable to serialize response: {}", source))]
    Serialization { source: serde_json::Error },
    #[snafu(display("The value {:?} is not a valid target", value))]
    InvalidTarget { value: String },
    #[snafu(display("The value {:?} is not a valid assembly flavor", value))]
    InvalidAssemblyFlavor { value: String },
    #[snafu(display("The value {:?} is not a valid demangle option", value))]
    InvalidDemangleAssembly { value: String },
    #[snafu(display("The value {:?} is not a valid assembly processing option", value))]
    InvalidProcessAssembly { value: String },
    #[snafu(display("The value {:?} is not a valid channel", value,))]
    InvalidChannel { value: String },
    #[snafu(display("The value {:?} is not a valid mode", value))]
    InvalidMode { value: String },
    #[snafu(display("The value {:?} is not a valid edition", value))]
    InvalidEdition { value: String },
    #[snafu(display("The value {:?} is not a valid crate type", value))]
    InvalidCrateType { value: String },
    #[snafu(display("No request was provided"))]
    RequestMissing,
    #[snafu(display("The cache has been poisoned"))]
    CachePoisoned,
}
