use argon2::{Argon2, PasswordHash, PasswordVerifier};
use libsql::Connection;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use chrono::Utc;

use crate::services::{database, server::pogoots::LoginResponse};

use super::server::LoginDBRequest;

pub struct UserManager {
    ///a map of tokens that lead to a user
    pub tokens: Mutex<HashMap<String, Arc<Mutex<User>>>>,
    ///A map of usernames that lead to a user
    pub users: Mutex<HashMap<String, Arc<Mutex<User>>>>,
    pub connection: Connection,
}
impl UserManager {
    pub async fn proccess_user_auth(
        &self,
        mut reciever: tokio::sync::mpsc::Receiver<LoginDBRequest>,
    ) {
        while let Some(request) = reciever.recv().await {
            match request {
                //Register user into database
                LoginDBRequest::Register(mut req, callback) => {
                    let email = std::mem::take(&mut req.email);
                    if self.users.lock().await.get(&email).is_some() {
                        let result = callback.send(LoginResponse {
                            success: false,
                            mystery: "User Logged In Already".to_string(),
                        });
                        if result.is_err() {
                            println!("Callback errored when user already logged in");
                        }
                        continue;
                    }
                    let password = std::mem::take(&mut req.password);
                    let database_query =
                        database::check_email_exists(&self.connection, &email).await;
                    //Checks have been completed, User can log in possibly
                    if let Ok(None) = database_query {
                        //User is not in the database, registering...
                        let database_store_result =
                            database::store_user_info(email.clone(), password, &self.connection)
                                .await;
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
                //Login user
                LoginDBRequest::Login(mut req, callback) => {
                    let email = std::mem::take(&mut req.password);
                    let password = std::mem::take(&mut req.password);
                    //Check if user exists in hashmap
                    let temp_user = self.users.lock().await;
                    let user = temp_user.get(&email);
                    // let mut exists = false;
                    let user = if user.is_none() {
                        Arc::new(Mutex::new(User {
                            username: email.clone(),
                            login_time: Utc::now(),
                            ips: vec![],
                            auth_tokens: vec![],
                        }))
                    } else {
                        // exists = true;
                        user.unwrap().clone()
                    };
                    drop(temp_user);

                    //Check if user exists in database
                    let database_query =
                        database::check_email_exists(&self.connection, &email).await;
                    if database_query.is_err() {
                        let result = callback.send(LoginResponse {
                            success: false,
                            mystery: "Database Query Failed".to_string(),
                        });
                        if result.is_err() {
                            println!("Callback errored when user already logged in");
                        }
                        //The database could not be queried, continuing
                        continue;
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
                        continue;
                    }
                    //Since the user exists in the database, the query will grant us a password
                    let hashed_password_correct = database_query.unwrap();
                    let argon2 = Argon2::default();
                    let parsed_hash = PasswordHash::new(&hashed_password_correct);
                    if parsed_hash.is_err() {
                        println!("Hashed password is not a valid hash");
                        //TODO: Better error handling
                        continue;
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
                        continue;
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
                //update account information
                LoginDBRequest::Update(req, callback) => {
                    todo!()
                }
            }
        }
    }

    // pub async fn retr_user(&self, username: String) -> Option<User> {
    //     todo!()
    // }
    // pub async fn issue_authentication_token(&self, user: User) -> Result<AuthToken, ()> {
    //     todo!()
    // }
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
