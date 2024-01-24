//Starts the command center instead of a normal server

use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

use super::{corporate::{CoordinatorState, FromClientRequest}, database::Database};

pub struct commandCenter{


}
impl commandCenter{
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
        let state = CoordinatorState{ login_thread_sender: login_system_access_point, db: database.clone(), one_server:false, commander:true};
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
        .with_state(database)
        .layer(CorsLayer::permissive())
        // .layer(TraceLayer::new_for_http())
        // .layer(TraceLayer::new_for_http())
        // .layer(SecureClientIpSource::ConnectInfo.into_extension())
        

    }
    pub async fn login_handler(State(state): State<Arc<CoordinatorState>>, SecureClientIp(ip): SecureClientIp, Json(json):Json<FromClientRequest>)->impl IntoResponse{
        //Ip tracking - Determine if the ip is from a known server
        let ip = ip.to_string();
        let (callback, callback_reciever) = oneshot::channel();
        let login_result = state.login_thread_sender.send(json.to_regular_request("".to_string(), callback)).await;
        if login_result.is_err(){
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        let callback_result = callback_reciever.await;
        if callback_result.is_err(){
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }

        return super::to_response_shortcut(callback_result.unwrap()).into_response()
    }
}