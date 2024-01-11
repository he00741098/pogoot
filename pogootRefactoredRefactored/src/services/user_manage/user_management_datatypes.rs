use serde::Serialize;
use tokio::sync::oneshot;

use super::permissions_management::Permissions;
///The login request also will be useed for registering
type callback = oneshot::Sender<LoginResponse>;
pub enum LoginRequest{
    ///Username, Password, Ip, Callback
    Login(String, String, String, callback),
    Register(String, String, String, callback),
    ///SessionToken, Username, Ip, Callback
    VerifySessionToken(String, String, String, callback)
}
#[derive(Serialize)]
pub enum LoginResponse{
    ///Session Token
    Success(String),
    ///For the session token verification, returns username
    Verified(String, Permissions),
    Failed
}
