use argon2::{Argon2, PasswordHash, PasswordVerifier};
use libsql::Connection;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{oneshot::Sender, Mutex};

use chrono::Utc;

use crate::services::{database, server::pogoots::LoginResponse};

use super::server::{
    pogoots::{UserLoginRequest, UserRegisterWithEmailRequest},
    LoginDBRequest,
};

#[derive(Clone)]
pub struct UserManager {
    ///a map of tokens that lead to a user
    pub tokens: Arc<Mutex<HashMap<String, Arc<Mutex<User>>>>>,
    ///A map of usernames that lead to a user
    pub users: Arc<Mutex<HashMap<String, Arc<Mutex<User>>>>>,
    pub connection: Connection,
}
impl UserManager {
    //TODO: Rate limiting
    pub async fn proccess_user_auth(
        &self,
        mut reciever: tokio::sync::mpsc::Receiver<LoginDBRequest>,
    ) {
        while let Some(request) = reciever.recv().await {
            match request {
                //Register user into database
                LoginDBRequest::Register(req, callback) => {
                    let temp_manager = self.clone();
                    tokio::spawn(async move {
                        temp_manager.register(req, callback).await;
                    });
                }
                //Login user
                LoginDBRequest::Login(req, callback) => {
                    let temp_manager = self.clone();
                    tokio::spawn(async move {
                        temp_manager.login(req, callback).await;
                    });
                }
                //update account information
                LoginDBRequest::Update(req, callback) => {
                    todo!()
                }
                LoginDBRequest::VerifyToken(token, username, callback) => {
                    let temp_manager = self.clone();
                    tokio::spawn(async move {
                        let lock = temp_manager.tokens.lock().await;
                        if let Some(user) = lock.get(&token) {
                            if user.lock().await.username == username {
                                let _ = callback.send(true);
                            } else {
                                let _ = callback.send(false);
                            }
                        } else {
                            let _ = callback.send(false);
                        }
                    });
                }
            }
        }
    }
    async fn register(
        &self,
        mut req: UserRegisterWithEmailRequest,
        callback: Sender<LoginResponse>,
    ) {
        let email = std::mem::take(&mut req.email);
        if self.users.lock().await.get(&email).is_some() {
            let result = callback.send(LoginResponse {
                success: false,
                mystery: "User Logged In Already".to_string(),
            });
            if result.is_err() {
                println!("Callback errored when user already logged in");
            }
            return;
        }
        let password = std::mem::take(&mut req.password);
        let database_query = database::check_email_exists(&self.connection, &email).await;
        //Checks have been completed, User can log in possibly
        if let Ok(None) = database_query {
            println!("User not in database, Registering...");
            //User is not in the database, registering...
            let database_store_result =
                database::store_user_info(email.clone(), password, &self.connection).await;
            //Stored data, checking if store succeeded
            //TODO: add failure management
            if database_store_result.is_err() {
                let result = callback.send(LoginResponse {
                    success: false,
                    mystery: "Database Store Failed".to_string(),
                });
                //The callback failed, TODO: add error management
                if result.is_err() {
                    println!("Callback errored when user already logged in");
                }
            } else {
                //Callback was sent successfully
                //Generate a new session token for them.
                let random_auth_token = uuid::Uuid::new_v4().to_string();
                //Map username to user
                let user = Arc::new(Mutex::new(User {
                    username: email.clone(),
                    login_time: Utc::now(),
                    ips: vec![],
                    auth_tokens: vec![AuthToken {
                        body: random_auth_token.clone(),
                        expiry: Utc::now() + Duration::from_secs(60 * 60 * 24),
                    }],
                }));
                self.users.lock().await.insert(email.clone(), user.clone());
                //Map token to user
                self.tokens
                    .lock()
                    .await
                    .insert(random_auth_token.clone(), user.clone());

                //User inserted, Callback with the token
                let result = callback.send(LoginResponse {
                    success: true,
                    mystery: random_auth_token,
                });
                //TODO: Add error management
                if result.is_err() {
                    println!("Callback errored when user already logged in");
                }
            }
        }

        // todo!()
    }
    async fn login(&self, mut req: UserLoginRequest, callback: Sender<LoginResponse>) {
        let email = std::mem::take(&mut req.email);
        let password = std::mem::take(&mut req.password);
        //Check if user exists in hashmap
        let temp_user = self.users.lock().await;
        // println!("{:?}\n\n{:?}", *temp_user, email);
        let user = temp_user.get(&email);
        // let mut exists = false;
        let user = if let Some(user) = user {
            // exists = true;
            // println!("User Found In Hashmaps");
            user.clone()
        } else {
            println!("User does not exist in hashmap");
            Arc::new(Mutex::new(User {
                username: email.clone(),
                login_time: Utc::now(),
                ips: vec![],
                auth_tokens: vec![],
            }))
        };

        drop(temp_user);

        //Check if user exists in database
        let database_query = database::check_email_exists(&self.connection, &email).await;
        if database_query.is_err() {
            let result = callback.send(LoginResponse {
                success: false,
                mystery: "Database Query Failed".to_string(),
            });
            if result.is_err() {
                println!("Callback errored when user already logged in");
            }
            //The database could not be queried, continuing
            return;
        }
        //The database query went through successfully
        let database_query = database_query.unwrap();

        if database_query.is_none() {
            //The user does not exist in the database
            let result = callback.send(LoginResponse {
                success: false,
                mystery: "User Not Found".to_string(),
            });
            //Callback sent
            if result.is_err() {
                //Callback failed
                println!("Callback errored when user already logged in");
            }
            //Continuing
            return;
        }
        //Since the user exists in the database, the query will grant us a password
        let hashed_password_correct = database_query.unwrap();
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&hashed_password_correct);
        if parsed_hash.is_err() {
            println!("Hashed password is not a valid hash");
            //TODO: Better error handling
            let _ = callback.send(LoginResponse {
                success: false,
                mystery: "Internal Server Error".to_string(),
            });
            return;
        }
        let parsed_hash = parsed_hash.unwrap();
        let correct = argon2.verify_password(password.as_bytes(), &parsed_hash);
        if correct.is_err() {
            //The password is wrong
            println!("Wrong password");
            let _ = callback.send(LoginResponse {
                success: false,
                mystery: "Wrong Password".to_string(),
            });
            return;
        }
        //Generating tokens and adding them to the user
        let token = uuid::Uuid::new_v4().to_string();
        user.lock().await.auth_tokens.push(AuthToken {
            body: token.clone(),
            expiry: Utc::now() + Duration::from_secs(60 * 60 * 24),
        });
        //Send the token to the User through callback
        let _ = callback.send(LoginResponse {
            success: true,
            mystery: token.clone(),
        });
        //Add the users to the maps
        self.tokens.lock().await.insert(token.clone(), user.clone());
        self.users.lock().await.insert(email.clone(), user.clone());
    }
}
#[derive(Debug)]
pub struct User {
    username: String,
    login_time: chrono::DateTime<Utc>,
    ips: Vec<IpInfo>,
    auth_tokens: Vec<AuthToken>,
}
#[derive(Debug)]
pub struct AuthToken {
    body: String,
    expiry: chrono::DateTime<Utc>,
}
#[derive(Debug)]
pub struct IpInfo {
    body: String,
    initial_login: chrono::DateTime<Utc>,
}
