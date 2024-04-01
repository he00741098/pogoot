use chrono::Utc;

use crate::AwsSecrets;

use super::server::LoginDBRequest;
pub async fn proccess_user_auth(
    mut reciever: tokio::sync::mpsc::Receiver<LoginDBRequest>,
    secrets: AwsSecrets,
) {
    while let Some(request) = reciever.recv().await {
        match request {
            //Register user into database
            LoginDBRequest::Register(req, callback) => {
                todo!()
            }
            //Login user
            LoginDBRequest::Login(req, callback) => {
                todo!()
            }
            //update account information
            LoginDBRequest::Update(req, callback) => {
                todo!()
            }
        }
    }
}

async fn retr_user(username: String) -> Option<User> {
    todo!()
}
async fn issue_authentication_token(user: User) -> Result<String, ()> {
    todo!()
}

pub struct User {
    username: String,
    login_time: chrono::DateTime<Utc>,
    ips: Vec<IpInfo>,
    auth_tokens: Vec<AuthToken>,
}
pub struct AuthToken {
    body: String,
    expiry: chrono::DateTime<Utc>,
}
pub struct IpInfo {
    body: String,
    initial_login: chrono::DateTime<Utc>,
}
