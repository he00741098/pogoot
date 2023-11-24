use std::time::{Duration};
use axum::{Router, routing::{get, post}};
use tracing::info;
use axum::response::Response;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::extract::State;
use std::sync::Arc;
use axum::extract::ws::Message;
use std::collections::HashMap;
use nanoid::nanoid;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
mod pogoot;
mod dataTypes;
mod util;
mod login;
use dataTypes::*;
use login::*;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // Build the subscriber
    .finish();
tracing::subscriber::set_global_default(subscriber)
.expect("setting default subscriber failed");
info!("Initiated subscriber");
    
    let login_sender = Login::start_thread(database::Database {  });
    let state = state_storage::state_storage{
        login_channel:login_sender,
        commander_portals:Arc::new(RwLock::new(HashMap::new())),
        games:Arc::new(RwLock::new(HashMap::new()))
    };

    // let state = Arc::new(Database{
    //     thead_addresses:RwLock::new(HashMap::new())    

    // });
    let app = Router::new()
        .route("/hello", get(|| async {"hello!"}))
        .route("/ws", get(player_handler))
        .route("/cws", get(commander_handler))
        .route("/login", post(Login::login_handler))
        .with_state(Arc::new(state))
        .layer(CorsLayer::permissive())
        // .layer(TraceLayer::new_for_http())
        // .layer(SecureClientIpSource::ConnectInfo.into_extension())
        ;
info!("App initiated");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
//TODO: Make game creation an endpoint instead of websocket.
//TODO: Make game login system
pub async fn player_handler(ws: WebSocketUpgrade, State(state): State<Arc<state_storage::state_storage>>) -> Response {
    info!("Handling Websocket Upgrade");
    ws.on_upgrade(|socket| pogoot::pogootGame::player_thread_start(socket, state))
}
pub async fn commander_handler(ws:WebSocketUpgrade, State(state): State<Arc<state_storage::state_storage>>)->Response{
    info!("Handling commander upgrade");
    ws.on_upgrade(|socket| pogoot::pogootGame::game_commander_handler(socket, state))
}
