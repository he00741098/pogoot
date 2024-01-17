use std::sync::Arc;
use std::net::SocketAddr;
use axum::http::StatusCode;
use tokio::sync::mpsc::Sender;
use axum::{extract::ws::WebSocket, Json};
use axum::{Router, routing::{post, get}, extract::{WebSocketUpgrade, State}, response::{Response, IntoResponse}};
use axum_client_ip::{SecureClientIpSource, SecureClientIp};
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;
use tokio::sync::oneshot;

use crate::services::notecard::storage_controller::NotecardStorageManager;

use self::datatypes::{NoteCardUploadRequest, NoteCardGetRequest, NoteCardGetRequestPart2};

use super::notecard::NoteCardVariants;
use super::{database::Database, user_manage::{user_management_datatypes::LoginRequest, self}};


type Callback = oneshot::Sender<user_manage::user_management_datatypes::LoginResponse>;
mod datatypes;
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
    ///Username, TOken
    VerifySessionToken(String, String)
}
#[test]
fn jsonify(){
    let login_json = FromClientRequest::Login("GGs".to_string(), "Poggins".to_string());
    let thing = serde_json::to_string(&login_json);
    println!("Login Json: {:?}", thing);
}
impl FromClientRequest{
    pub fn to_regular_request(self, ip:String, callback:Callback)->LoginRequest{
        match self{
            FromClientRequest::Login(username, password) => LoginRequest::Login(username, password, ip, callback),
            FromClientRequest::Register(username, password) => LoginRequest::Register(username, password, ip, callback),
            FromClientRequest::VerifySessionToken(username, token) => LoginRequest::VerifySessionToken(token,username, ip, callback),
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
        .route("/ntcdup", post(Self::upload_note_card))
        .route("/ntlist", post(Self::list_note_card_ids))
        .route("/ntcdget", post(Self::get_note_card))
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
    pub async fn login_handler(State(state): State<Arc<CoordinatorState>>, SecureClientIp(ip): SecureClientIp, Json(json):Json<FromClientRequest>)->impl IntoResponse{
        let ip = ip.to_string();
        let (callback, callback_reciever) = oneshot::channel();
        let login_result = state.login_thread_sender.send(json.to_regular_request(ip, callback)).await;
        if login_result.is_err(){
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        let callback_result = callback_reciever.await;
        if callback_result.is_err(){
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }

        return super::to_response_shortcut(callback_result.unwrap()).into_response()
    }
    pub async fn handle_player_socket(socket:WebSocket){
        todo!()
    }
    pub async fn handle_commander_socket(socket:WebSocket){
        todo!()
    }
    pub async fn upload_note_card(State(state):State<Arc<CoordinatorState>>, SecureClientIp(ip):SecureClientIp, Json(json):Json<NoteCardUploadRequest>)->impl IntoResponse{
        //TODO: verify the validity of the session token
        let (callback, callback_reciever) = oneshot::channel();
        let login_send_result = state.login_thread_sender.send(LoginRequest::VerifySessionToken(json.session_token.clone(), json.username.clone(), ip.to_string(), callback)).await;
        if login_send_result.is_err(){
            println!("Login send failed");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }else{
            let callback_result = callback_reciever.await;
            if callback_result.is_err(){
                println!("Callback recieve failed");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }else{
                let callback_final = callback_result.unwrap();
                match callback_final{
                    user_manage::user_management_datatypes::LoginResponse::Verified => {
                        println!("Successful token Verification");
                        let store_result = NotecardStorageManager::notecard_store(json.username, json.set_name, json.notecard_varient, state.db.clone()).await;
                        if store_result.is_ok(){
                            let notecard_id = store_result.unwrap();
                            let register_result = state.login_thread_sender.send(LoginRequest::RegisterNoteCardId(notecard_id, json.session_token)).await;
                            if register_result.is_err(){
                                println!("Note card Register failed");
                            }
                            return StatusCode::OK.into_response();
                        }else{
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    },
                    _=>{
                        println!("Token Verification Failed");
                        return serde_json::to_string(&callback_final).unwrap().into_response();
                    }
                }
            }
        }
    }
    pub async fn get_note_card(State(state):State<Arc<CoordinatorState>>, SecureClientIp(ip):SecureClientIp, Json(json):Json<NoteCardGetRequestPart2>)->impl IntoResponse{
        // pub set_name: String,
        // pub session_token:String,
        // pub username:String,
        let perms = state.db.get_notecard_permissions(&json.notecard_id).await;
        if perms.is_err(){
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        let mut perms = perms.unwrap();
        if !perms.has_permission(&json.username){
            return StatusCode::FORBIDDEN.into_response();
        }
        let fetched = state.db.fetch_note_card(json.notecard_id).await;
        if fetched.is_err(){
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        let raw_json = fetched.unwrap();
        raw_json.into_response()
    }

    pub async fn list_note_card_ids(State(state):State<Arc<CoordinatorState>>, SecureClientIp(ip):SecureClientIp, Json(json):Json<NoteCardGetRequest>)->impl IntoResponse{
        let (callback, callback_reciever) = oneshot::channel();
        let user = state.login_thread_sender.send(LoginRequest::GetUser(json.session_token, callback)).await;
        if user.is_err(){
            println!("Login thread send failed");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        if let Ok(Ok(user)) = callback_reciever.await{
            println!("Successfully found user");
            let locked = user.lock().await;
            return serde_json::to_string(&locked.uploaded_sets).unwrap().into_response();
        }
        println!("Failed to get user");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    async fn shut_down_sequence(State(state):State<Arc<CoordinatorState>>){
        //shut down logins
        let (callback, callback_reciever) = oneshot::channel();
        let _ = state.login_thread_sender.send(LoginRequest::Shutdown(callback)).await;
        let _ = callback_reciever.await;
        todo!("Make graceful shutdown work perfectly");
    }
    

}

