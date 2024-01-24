use std::{collections::{HashMap, VecDeque}, sync::Arc};

use async_std::sync::Mutex;
use chrono::Utc;
use nanoid::nanoid;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::services::database::Database;

use super::{user_management_datatypes::{LoginRequest, LoginResponse}, User};

trait LoginHandler{
    async fn thread_start(self)->Sender<LoginRequest> where Self: Sized;
    async fn thread(self, rx:Receiver<LoginRequest>);
    async fn generate_new_token(&mut self, user:User)->String;
    fn new(db:Arc<Database>)->Self;
    async fn generate_new_token_for_existing_user(self, user:Arc<Mutex<User>>, username:String)->String;
    async fn cleanup_expired_tokens(self);
}

pub struct LoginSystem{
    // session_tokens:Vec<String>
    logged_in_users:HashMap<String, Arc<Mutex<User>>>,
    //Map of Usernames to tokens
    user_token_map:HashMap<String, VecDeque<String>>,
    cleanup_logger:VecDeque<(String, chrono::DateTime<Utc>)>,
    database_access:Arc<Database>
}
impl LoginSystem{

    
    pub async fn thread_start(self)->Sender<LoginRequest>{
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(self.thread(rx));
        tx
    }
    async fn thread(mut self, mut rx:Receiver<LoginRequest>){
        while let Some(thing) = rx.recv().await{
            match thing{
                LoginRequest::Login(username, password, ip, callback) => {
                    let result = self.database_access.verify_password(username.clone(), password).await;
                    let verified = match result{
                        Ok(bool) => bool,
                        Err(_) => {
                            println!("Password Verify Failed");
                            let callback_result = callback.send(LoginResponse::Failed);
                            if callback_result.is_err(){
                                println!("Callback Was Error After Login Response Failed");
                            }
                            continue
                        },
                    };
                    if verified.0{
                        if verified.1.len()>1{

                            let new_token = if let Some(usernames) = self.user_token_map.get_mut(&username){
                                let mut token = None;
                                for usernamesUsername in usernames.iter(){
                                    if let Some(user) = self.logged_in_users.get(&usernamesUsername.clone()){
                                        let cloned_thing = usernamesUsername.clone();
                                        token = Some(self.generate_new_token_for_existing_user(user.clone(), cloned_thing).await);
                                        break;
                                    }
                                }
                                if token.is_some(){
                                    token.unwrap()
                                }else{
                                    
                            let user = serde_json::from_str(&verified.1);
                            let mut user:User = match user{
                                Ok(user) => user,
                                Err(_) => {
                                    let callback_result = callback.send(LoginResponse::Failed);
                                    if callback_result.is_err(){
                                        println!("Callback was Error");
                                    }
                                    continue;
                                },
                            };
                            user.most_recent_ip=ip.clone();
                            user.ips.push(ip);
                            //token generation sequence, New tokens will be generated each login.
                            //Old tokens will still be able to be used but will expire. All tokens
                            //expire in 1 day, no renewing
                            
                            let new_token = self.generate_new_token(user).await;
                                new_token
                                }
                            }else{
                            let user = serde_json::from_str(&verified.1);
                            let mut user:User = match user{
                                Ok(user) => user,
                                Err(_) => {
                                    let callback_result = callback.send(LoginResponse::Failed);
                                    if callback_result.is_err(){
                                        println!("Callback was Error");
                                    }
                                    continue;
                                },
                            };
                            user.most_recent_ip=ip.clone();
                            user.ips.push(ip);
                            //token generation sequence, New tokens will be generated each login.
                            //Old tokens will still be able to be used but will expire. All tokens
                            //expire in 1 day, no renewing
                            
                            let new_token = self.generate_new_token(user).await;
                                new_token
                            };

                            if let Some(token_vec) = self.user_token_map.get_mut(&username){
                                if token_vec.len()>4{
                                    let popped_token = token_vec.pop_front();
                                    if let Some(popped_token) = popped_token{
                                        self.logged_in_users.remove(&popped_token);
                                    }
                                }
                                token_vec.push_back(new_token.clone())
                            }

                            let callback_result = callback.send(LoginResponse::Success(new_token));
                            if callback_result.is_err(){
                                println!("Callback was Error");
                            }

                        }else{
                            let callback_result = callback.send(LoginResponse::Failed);
                            if callback_result.is_err(){
                                println!("Callback was Error");
                            }
                            continue;
                        }
                    }else{
                        println!("Password is wrong");
                        let callback_result = callback.send(LoginResponse::Failed);
                        if callback_result.is_err(){
                            println!("Callback was Error");
                        }
                        continue;
                    }
                },
                LoginRequest::VerifySessionToken(sessions_token, username, ip, callback) => {
                    todo!("Verify Session Token")
                },
                LoginRequest::Register(username, password,ip, callback) => {
                    let result = self.database_access.register_user(username, password, ip.clone()).await;
                    if result.is_err(){
                        let callback_result = callback.send(LoginResponse::Failed);
                        if callback_result.is_err(){
                            println!("Callback was Error when registering");
                        }
                        continue;
                    }
                    let mut user = result.unwrap();
                    user.most_recent_ip=ip.clone();
                    user.ips.push(ip);
                    let new_token = self.generate_new_token(user).await;
                    let callback_result = callback.send(LoginResponse::Success(new_token));
                    if callback_result.is_err(){
                        println!("Callback was Error");
                    }
                },
            }
        }
    }
    pub fn new(db:Arc<Database>)->Self{
        LoginSystem { logged_in_users: HashMap::new(), database_access: db, cleanup_logger:VecDeque::new(), user_token_map:HashMap::new() }
    }
    async fn generate_new_token(&mut self, user:User)->String{
        loop{
            let new_token = nanoid!(30);
            if self.logged_in_users.get(&new_token).is_none(){
                self.logged_in_users.insert(new_token.clone(), Arc::new(Mutex::new(user)));
                self.cleanup_logger.push_back((new_token.clone(), Utc::now()+chrono::Duration::days(1)));
                return new_token
            }
        }
    }
    ///GENERATES A TOKEN FOR A USER THAT IS ALREADY LOGGED IN
    async fn generate_new_token_for_existing_user(&mut self, user:Arc<Mutex<User>>, username:String)->String{
        loop{
            let new_token = nanoid!(30);
            if self.logged_in_users.get(&new_token).is_none(){
                self.logged_in_users.insert(new_token.clone(), user.clone());
                if let Some(user_vec) = self.user_token_map.get_mut(&username){
                    user_vec.push_back(new_token.clone())
                }
                self.cleanup_logger.push_back((new_token.clone(), Utc::now()+chrono::Duration::days(1)));
                return new_token
            }
        }
    }


    async fn cleanup_expired_tokens(&mut self){
        for i in 0..self.cleanup_logger.len(){
            if self.cleanup_logger[i].1<Utc::now(){
                let popped = self.cleanup_logger.pop_front();
                if popped.is_some(){
                    let popped = popped.unwrap();
                    let user = self.logged_in_users.remove(&popped.0);
                    if user.is_some(){
                        // let locked_user = user.unwrap().lock().await;
                        let result_of_update = self.database_access.update_user_json(user.unwrap()).await;
                        if result_of_update.is_err(){
                            println!("Failed to update -- TODO: Make something to ensure that failed data is not lost\nMaybe store to local file or something?");
                        }

                    }
                    //Implement renew system
                    // if self.renew_log.contains(self.cleanup_logger[i].0){
                    //     self.
                    // }
                }

            }else{
                return;
            }

        }
    }

}

///A struct intended to deal with sending login requests to a central database/login system
/// This will ensure that there are no race conditions.
pub struct remoteLoginSystem{


}
impl remoteLoginSystem{
    ///sends a post request to the login server
    async fn proccess_request(request:LoginRequest)->LoginResponse{
        match request{
            LoginRequest::Login(username, password, ip, _) => {

            },
            LoginRequest::Register(username, password, ip, _) => {

            },
            LoginRequest::VerifySessionToken(token, username, ip, _) => {

            },
        }
    }
}