//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-chat
//! ```

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Path,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::{HashSet, VecDeque, HashMap},
    net::SocketAddr,
    sync::{Arc, Mutex, MutexGuard}, hash::Hash,
};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
struct MatchWrapper(PlayerWrapper, PlayerWrapper);

#[derive(Clone, Debug)]
struct PlayerWrapper {
    name: String,
    tx: broadcast::Sender<AppMessage>
}

impl PlayerWrapper {
    fn new(name: &str) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {name: name.to_owned(), tx}
    }
}

impl Eq for PlayerWrapper {}
impl PartialEq for PlayerWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Hash for PlayerWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone)]
enum AppMessage {
    ComponentGone(String),
    ComponentJoined(String),
    ComponentMoved,
    TimeOver,
    MatchStarted(String),
}

// Our shared state
#[derive(Debug)]
struct AppState {
    /// maps match -> player + stream
    match_map: Mutex<HashMap<String, MatchWrapper>>,
    /// maps player -> match
    player_map: Mutex<HashMap<String, String>>,
    /// waiting players
    waiting_players: Mutex<HashSet<PlayerWrapper>>,
}

impl AppState {
    fn join_player(&self, player: &PlayerWrapper) -> PlayerState {
        let mut player_map = self.player_map.lock().unwrap();
        if let Some(mat) = player_map.get(&player.name) {
            PlayerState::Playing(mat.to_owned())
        } else {
            self.insert_player(player, player_map)
        }
    }

    fn insert_player(&self, player: &PlayerWrapper, mut player_map: MutexGuard<HashMap<String, String>>) -> PlayerState {
        let mut match_map = self.match_map.lock().unwrap();
        let mut waiting_players = self.waiting_players.lock().unwrap();
        if let Some(wp) = waiting_players.iter().next().cloned() {
            let match_name = "random match name";
            waiting_players.remove(&wp);
            match_map.insert(match_name.to_owned(), MatchWrapper(wp, player.clone()));
            player_map.insert(player.name.to_owned(), match_name.to_owned());
            // send that person, that game starts
            
            PlayerState::Playing(match_name.to_owned())
        } else {
            waiting_players.insert(player.clone());
            PlayerState::Waiting
        }
    }

    fn remove_player(&self, player: &PlayerWrapper) {
        if let Some(match_name) = self.player_map.lock().unwrap().remove(&player.name) {
            let match_obj = self.match_map.lock().unwrap().remove(&match_name).unwrap();
            let other_player = if player.name == match_obj.0.name { match_obj.1 } else { match_obj.0 };
            other_player.tx.send(AppMessage::ComponentGone(player.name.clone()));
            let mut waiting_player = self.waiting_players.lock().unwrap();
            if let Some(wp) = waiting_player.iter().next().cloned() {
                waiting_player.remove(&wp);
            }
        } //else if let mut waiting_players = state.waiting_players.lock().unwrap();

    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rattle=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let match_map = Mutex::new(HashMap::new());
    let player_map = Mutex::new(HashMap::new());
    let waiting_players = Mutex::new(HashSet::new());

    let app_state = Arc::new(AppState { match_map, player_map, waiting_players });

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket/:1", get(websocket_handler))
        .layer(Extension(app_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(player_id): Path<HashMap<String, String>>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(player_id, socket, state))
}

enum PlayerState {
    Waiting,
    /// playing in the match
    Playing(String),
    DoingNothing,
}

async fn websocket(player_id: HashMap<String, String>, stream: WebSocket, state: Arc<AppState>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let (tx, _rx) = broadcast::channel(100);

    let p = PlayerWrapper { name: player_id.get("id").unwrap_or(&"default".to_string()).to_string(), tx: tx.clone() };

    // let p = PlayerWrapper::new(&player_id);

    let playerstate = state.join_player(&p);

    println!("Here {:?}", state);


    // Subscribe before sending joined message.
    let mut rx = p.tx.subscribe();

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text("Hello".to_string())).await.is_err() {
                break;
            }
        }
    });

    let _ = tx.send(AppMessage::MatchStarted("fake match".to_string()));

    // This task will receive messages from client and send them to broadcast subscribers.
    // let mut recv_task = tokio::spawn(async move {
    //     while let Some(Ok(Message::Text(text))) = receiver.next().await {
    //         // Add username before message.
    //         let _ = tx.send(AppMessage::MatchStarted(format!("{}: {}", player_id, text)));
    //     }
    // });

    println!("Here II");

// If any one of the tasks exit, abort the other.
    // tokio::select! {
    //     _ = (&mut send_task) => recv_task.abort(),
    //     _ = (&mut recv_task) => send_task.abort(),
    // };

    println!("Here III");

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            // seperate by type of json request and change the user state by type
            println!("{:?}", text)
            //
            // Code Submitted -> Waiting for Result
            // Both Code Submitted -> Finished & Waiting for new game...
        }
    }

    println!("Here IV");

    state.remove_player(&p);
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html("Hello RustFi!")
}