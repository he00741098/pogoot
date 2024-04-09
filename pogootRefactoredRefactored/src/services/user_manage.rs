use libsql::Connection;
use std::collections::HashMap;
use uuid::uuid;

use chrono::Utc;

use crate::{
    services::{database, server::pogoots::LoginResponse},
    AwsSecrets,
};

use super::server::LoginDBRequest;

pub struct User_Manager {
    ///a map of tokens that lead to a user
    pub tokens: HashMap<String, User>,
    ///A map of usernames that lead to a user
    pub users: HashMap<String, User>,
    pub connection: Connection,
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
                    let email = std::mem::take(&mut req.email);
                    if self.users.get(&email).is_some() {
                        let result = callback
                            .send(LoginResponse {
                                success: false,
                                mystery: "User Logged In Already".to_string(),
                            })
                            .await;
                        if result.is_err() {
                            println!("Callback errored when user already logged in");
                        }
                        continue;
                    }
                    let password = std::mem::take(&mut req.password);
                    let database_query =
                        database::check_email_exists(&self.connection, &email).await;
                    //user can log in
                    if let Ok(None) = database_query {
                        let database_store_result =
                            database::store_user_info(email, password, &self.connection).await;
                        if database_store_result.is_err() {
                            let result = callback
                                .send(LoginResponse {
                                    success: false,
                                    mystery: "Database Store Failed".to_string(),
                                })
                                .await;
                            if result.is_err() {
                                println!("Callback errored when user already logged in");
                            }
                            // continue;
                        } else {
                            let random_auth_token = uuid::Uuid::new_v4().to_string();

                            let result = callback
                                .send(LoginResponse {
                                    success: true,
                                    mystery: random_auth_token,
                                })
                                .await;
                            if result.is_err() {
                                println!("Callback errored when user already logged in");
                            }
                        }
                    }

                    // todo!()
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
