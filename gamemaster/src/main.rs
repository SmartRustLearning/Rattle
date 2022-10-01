#![deny(rust_2018_idioms)]
use crate::sandbox::Sandbox;
mod logic;
mod sandbox;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, TypedHeader,
    },
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Extension, Router,
};
use logic::*;
//use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::json;
use snafu::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
//use tokio::sync::RwLock;

/**
 * Global state
 */
// #[derive(Debug)]
// struct State {
//     matches: Vec<Match>,
// }

#[derive(Deserialize)]
struct MatchPayload {
    username: String,
}

use futures::future::{self, Either};
//use futures::{sink::SinkExt, stream::StreamExt};
// use std::{collections::HashSet, sync::Mutex};
// use tokio::sync::broadc11ast;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ADDRESS: &str = "0.0.0.0:5000";

#[tokio::main]
async fn main() {
    // Enable info-level logging by default. env_logger's default is error only.
    // let env_logger_config = env_logger::Env::default().default_filter_or("info");
    // env_logger::Builder::from_env(env_logger_config).init();

    // hasmap from gameId to (Player, WebSocket)
    let shared_state = Arc::new(RwLock::new(HashMap::<String, Match>::new())); //from([(
                                                                               //     "FOO".to_string(),
                                                                               //     logic::Match::new("foo".to_string(), Exercise::default()),
                                                                               // )])));

    let app = Router::new()
        .route("/", get(index))
        .route("/matches", get(list_matches))
        .route("/match/find", get(find_match))
        .route("/match/:id/join", get(websocket_handler))
        .layer(Extension(shared_state));
    // .route(
    //     "/match/find",
    //     post(
    //         |Json(body): Json<MatchPayload>| -> Json<serde_json::Value> {
    //             let shared_state = Arc::clone(&shared_state);
    //             move |body| find_or_create_match(body, Arc::clone(&shared_state))
    //         },
    //     ),
    // )
    // .route(
    //     "/match/join",
    //     post({
    //         let shared_state = Arc::clone(&shared_state);
    //         move |body| join_match(body, Arc::clone(&shared_state))
    //     }),
    // )

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

async fn list_matches(
    Extension(shared_state): Extension<Arc<RwLock<HashMap<String, Match>>>>,
) -> String {
    let state = &shared_state.read().await;
    return format!("{:?}", &state);
}

async fn find_match(
    Extension(shared_state): Extension<Arc<RwLock<HashMap<String, Match>>>>, //  Json(payload): Json<MatchPayload>,
) -> String {
    println!("FIND MATCHES !");
    // -> Result<(), (StatusCode, String)> {
    //-> Json<serde_json::Value> {

    let mut state = shared_state.write().await;
    let id = if state.len() == 0 {
        let mut m = Match::new("P1".to_string(), Exercise::from_path("./exercises/ex1"));
        let id = m.id.to_string();
        state.insert(m.id.to_string(), m);
        return id;
    } else {
        return state.keys().next().unwrap().to_string();
    };

    //    m.players.push(Player {
    let obj = json!({ "match_id": id });
    format!("{}", serde_json::to_string_pretty(&obj).unwrap())
    //    state.Json(json!({ "new_username": username }));
}

async fn websocket_handler(
    Extension(shared_state): Extension<Arc<RwLock<HashMap<String, Match>>>>,
    Path(params): Path<HashMap<String, String>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::debug!("`{}` connected", user_agent.as_str());
    }

    let id = params.get("id").unwrap().to_string();

    ws.on_upgrade(|mut socket| async move {
        let rwguard = shared_state.read().await;
        let sender = &rwguard.get(&id).unwrap().sender;
        let mut recv = sender.subscribe();

        loop {
            let next_in_msg = socket.recv();
            let next_out_msg = recv.recv();

            // TODO: filter messages
            let either = {
                futures::pin_mut!(next_in_msg);
                futures::pin_mut!(next_out_msg);
                match future::select(next_out_msg, next_in_msg).await {
                    Either::Left((v, _)) => Either::Left(v),
                    Either::Right((v, _)) => Either::Right(v),
                }
            };
            match either {
                Either::Left(v) => {
                    // message from the channel
                    println!("message from chan: {:?}", &v);
                    let v = v.unwrap();
                    socket
                        .send(Message::Text(format!("{:?} {:?}", &v.0, &v.1)))
                        .await
                        .unwrap()
                } // result
                Either::Right(v) => {
                    // message from the socket
                    println!("message from sock {:?}", &v);
                    let txt = match v.unwrap().unwrap() {
                        Message::Text(s) => s,
                        _ => panic!("ping pong close should be implemented!"),
                    };

                    let root: serde_json::Value = serde_json::from_str(&txt).unwrap();
                    match root.get("message_type").unwrap().as_str().unwrap() {
                        "AUTH_MSG" => {
                            // join plaeryr
                            let game: &mut logic::Match =
                                shared_state.write().await.get(&id).unwrap();
                            game.join(root.get("username").unwrap().as_str().unwrap().to_string());
                        }
                        "SUBMIT" => {}
                        _ => {}
                    }
                    // send to everyone
                    sender.send((id.clone(), txt)).unwrap(); // if websocket client closes connection it returns none
                }
            };
        }
    }) //handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {

    // while let Some(msg) = socket.recv().await {
    //     if let Ok(msg) = msg {
    //         match msg {
    //             Message::Text(t) => {
    //                 let sb = Sandbox::new().await.context(SandboxCreationSnafu).unwrap();
    //                 println!("Code received!!: \n'''{:?}\n'''", t);
    //                 let req = sandbox::CompileRequest {
    //                     target: sandbox::CompileTarget::LlvmIr,
    //                     channel: sandbox::Channel::Stable,
    //                     crate_type: sandbox::CrateType::Binary,
    //                     mode: sandbox::Mode::Debug,
    //                     tests: false,
    //                     code: t.to_string(),
    //                     edition: None,
    //                     backtrace: false,
    //                 };
    //                 let res = sb.compile(&req);
    //                 // TODO: check if compilation was succesfull

    //                 let req = sandbox::ExecuteRequest {
    //                     channel: sandbox::Channel::Stable,
    //                     crate_type: sandbox::CrateType::Binary,
    //                     mode: sandbox::Mode::Debug,
    //                     tests: false,
    //                     code: t.to_string(),
    //                     edition: None,
    //                     backtrace: false,
    //                 };
    //                 let res = sb.execute(&req).await;

    //                 println!("{:?}", &res);

    //                 if socket
    //                     .send(Message::Text(
    //                         // String::from(res.unwrap().stdout)
    //                         json!(res.unwrap()).to_string(),
    //                     ))
    //                     .await
    //                     .is_err()
    //                 {
    //                     println!("client disconnected");
    //                     return;
    //                 }
    //                 println!("tickp");
    //             }
    //             Message::Binary(_) => {
    //                 println!("client sent binary data");
    //             }
    //             Message::Ping(_) => {
    //                 println!("socket ping");
    //             }
    //             Message::Pong(_) => {
    //                 println!("socket pong");
    //             }
    //             Message::Close(_) => {
    //                 println!("client disconnected");
    //                 return;
    //             }
    //         }
    //     } else {
    //         println!("client disconnected");
    //         return;
    //     }
    // }
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
