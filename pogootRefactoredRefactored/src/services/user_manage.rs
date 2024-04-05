use libsql::Connection;
use std::collections::HashMap;

use chrono::Utc;

use crate::AwsSecrets;

use super::server::LoginDBRequest;

pub struct User_Manager {
    ///a map of tokens that lead to a user
    tokens: HashMap<String, User>,
    ///A map of usernames that lead to a user
    users: HashMap<String, User>,
    connection: Connection,
}
impl User_Manager {
    pub async fn proccess_user_auth(
        &self,
        mut reciever: tokio::sync::mpsc::Receiver<LoginDBRequest>,
        secrets: AwsSecrets,
    ) {
        while let Some(request) = reciever.recv().await {
            match request {
                //Register user into database
                LoginDBRequest::Register(mut req, callback) => {
                    let email = std::mem::take(&mut req.password);
                    let password = std::mem::take(&mut req.password);

                    todo!()
                }
                //Login user
                LoginDBRequest::Login(mut req, callback) => {
                    let email = std::mem::take(&mut req.password);
                    let password = std::mem::take(&mut req.password);
                    todo!()
                }
                //update account information
                LoginDBRequest::Update(req, callback) => {
                    todo!()
                }
            }
        }
    }

    pub async fn retr_user(&self, username: String) -> Option<User> {
        todo!()
    }
    pub async fn issue_authentication_token(&self, user: User) -> Result<AuthToken, ()> {
        todo!()
    }
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
