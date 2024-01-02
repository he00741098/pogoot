use std::sync::Arc;
use std::{net::SocketAddr};
use tokio::sync::{mpsc::Sender};
use axum::{extract::ws::WebSocket, Json};
use axum::{Router, routing::{post, get}, extract::{WebSocketUpgrade, State}, response::{Response, IntoResponse}};
use axum_client_ip::{SecureClientIpSource, SecureClientIp};
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;
use tokio::sync::oneshot;
use super::{database::Database, user_manage::{user_management_datatypes::LoginRequest, self}};


type Callback = oneshot::Sender<user_manage::user_management_datatypes::LoginResponse>;

pub struct Coordinator{
}
pub struct CoordinatorState{
    login_thread_sender:Sender<LoginRequest>,
    db:Arc<Database>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FromClientRequest{
    ///Username, Password, Ip
    Login(String, String),
    Register(String, String),
    ///SessionToken, Ip
    VerifySessionToken(String)
}
impl FromClientRequest{
    pub fn to_regular_request(self, ip:String, callback:Callback)->LoginRequest{
        match self{
            FromClientRequest::Login(username, password) => LoginRequest::Login(username, password, ip, callback),
            FromClientRequest::Register(username, password) => LoginRequest::Register(username, password, ip, callback),
            FromClientRequest::VerifySessionToken(token) => LoginRequest::VerifySessionToken(token, ip, callback),
        }
    }
}

impl Coordinator{

    pub async fn start_all_services(){
        //TODO: deal with user management
        //TODO: Complete all of the database stuff
        //TODO: all notecards to be transfered
        // let notecard_storage_manager =NotecardStorageManager{};
        
        //initialization sequence
        //
        //Init the database
        let database = Arc::new(Database::new(Database::try_to_get_secrets()).await.unwrap());
        let login_system = super::user_manage::short_term_user_management::LoginSystem::new(database.clone());
        let login_system_access_point = login_system.thread_start().await;
        let state = CoordinatorState{ login_thread_sender: login_system_access_point, db: database.clone() };
        let dbstate = Arc::new(state);
        //Init the login/user management service
        //start listening for requests
    let app = Self::start_router(dbstate.clone()).await;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    }
    pub async fn start_router(database:Arc<CoordinatorState>)->Router{
    // let state = Database::new(Database::try_to_get_secrets()).await.unwrap();
    Router::new()
        .route("/login", post(Self::login_handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension()) 
        .route("/hello", get(|| async {"hello!"}))
        .route("/ws", get(Self::player_handler))
        .route("/cws", get(Self::commander_handler))
        .with_state(database)
        .layer(CorsLayer::permissive())
        // .layer(TraceLayer::new_for_http())
        // .layer(TraceLayer::new_for_http())
        // .layer(SecureClientIpSource::ConnectInfo.into_extension())
        

    }
    pub async fn player_handler(ws: WebSocketUpgrade, State(state): State<Arc<CoordinatorState>>)->Response {
        // info!("Handling Websocket Upgrade");
        ws.on_upgrade(|socket| Self::handle_player_socket(socket))
    }
    pub async fn commander_handler(ws:WebSocketUpgrade, State(state): State<Arc<CoordinatorState>>)->Response{
        // info!("Handling commander upgrade");
        ws.on_upgrade(|socket| Self::handle_commander_socket(socket))
    }
    pub async fn login_handler(State(state): State<Arc<Database>>, SecureClientIp(ip): SecureClientIp, Json(json):Json<FromClientRequest>)->impl IntoResponse{
        let ip = ip.to_string();
    }
    pub async fn handle_player_socket(socket:WebSocket){

    }
    pub async fn handle_commander_socket(socket:WebSocket){

    }

}

