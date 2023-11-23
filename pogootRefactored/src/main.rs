use std::pin::pin;
use std::time::{SystemTime, Duration};
use axum::{Router, routing::get};
use futures::{poll};
use futures_util::future::join_all;
use futures_util::stream::FuturesUnordered;
use tracing::info;
use axum::response::Response;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::extract::State;
use std::sync::Arc;
use axum::extract::ws::Message;
use futures_util::{stream::{StreamExt, SplitSink, SplitStream}};
use std::collections::HashMap;
use nanoid::nanoid;
use async_std::sync::RwLock;
use serde_json::to_string;

mod pogoot;
mod dataTypes;
mod util;
mod login;
use util::*;
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

    // let state = Arc::new(Database{
    //     thead_addresses:RwLock::new(HashMap::new())    

    // });
    let app = Router::new()
    // .route("/hello", get(|| async {"hello!"}))
    // .route("/ws", get(handler))
    // .with_state(state)
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

