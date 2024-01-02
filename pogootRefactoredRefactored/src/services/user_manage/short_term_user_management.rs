use std::{collections::{HashMap, VecDeque}, sync::Arc};

use chrono::{Utc, Duration};
use nanoid::nanoid;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::services::database::{CoreDatatypeError, Database};

use super::{user_management_datatypes::{LoginRequest, LoginResponse}, User};



pub struct LoginSystem{
    // session_tokens:Vec<String>
    logged_in_users:HashMap<String, User>,
    cleanup_logger:VecDeque<(String, chrono::DateTime<Utc>)>,
    renew_log:Vec<String>,
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
                    let result = self.database_access.verify_password(username, password).await;
                    let verified = match result{
                        Ok(bool) => bool,
                        Err(_) => {
                            let callback_result = callback.send(LoginResponse::Failed);
                            if callback_result.is_err(){
                                println!("Callback Was Error After Login Response Failed");
                            }
                            continue
                        },
                    };
                    if verified.0{
                        if verified.1.len()>1{
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
                            let new_token = self.generate_new_token(user).await;
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
                        let callback_result = callback.send(LoginResponse::Failed);
                        if callback_result.is_err(){
                            println!("Callback was Error");
                        }
                        continue;
                    }
                },
                LoginRequest::VerifySessionToken(sessions_token, ip, callback) => {

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
        LoginSystem { logged_in_users: HashMap::new(), database_access: db, cleanup_logger:VecDeque::new(), renew_log:vec![] }
    }
    async fn generate_new_token(&mut self, user:User)->String{
        loop{
            let new_token = nanoid!(30);
            if self.logged_in_users.get(&new_token).is_none(){
                self.logged_in_users.insert(new_token.clone(), user);
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


