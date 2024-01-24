use std::sync::Arc;

use async_std::sync::Mutex;
use serde::Serialize;
use tokio::sync::oneshot;

use super::{permissions_management::Permissions, User};
///The login request also will be useed for registering
type callback = oneshot::Sender<LoginResponse>;
pub enum LoginRequest{
    ///Username, Password, Ip, Callback
    Login(String, String, String, callback),
    Register(String, String, String, callback),
    ///SessionToken, Username, Ip, Callback
    VerifySessionToken(String, String, String, callback),
    ///Store all data and shutdown
    Shutdown(callback),
    ///Takes a set id and adds it to the user, token
    RegisterNoteCardId(String, String),
    GetUser(String, oneshot::Sender<Result<Arc<Mutex<User>>,()>>)

}
#[derive(Serialize)]
pub enum LoginResponse{
    ///Session Token
    Success(String),
    ///For the session token verification, returns username
    Verified,
    Failed,
}
