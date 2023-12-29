use std::collections::HashMap;

use nanoid::nanoid;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::services::database::{CoreDatatypeError, Database};

use super::{user_management_datatypes::{LoginRequest, LoginResponse}, User};



pub struct LoginSystem{
    // session_tokens:Vec<String>
    logged_in_users:HashMap<String, User>,
    database_access:Database
}
impl LoginSystem{

    async fn login(&self, username:String, password:String, ip:String)->Result<String, CoreDatatypeError>{
        todo!()
    }
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
                            let user = match user{
                                Ok(user) => user,
                                Err(_) => {
                                    let callback_result = callback.send(LoginResponse::Failed);
                                    if callback_result.is_err(){
                                        println!("Callback was Error");
                                    }
                                    continue;
                                },
                            };
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

                },
            }
        }
    }
    pub fn new(db:Database)->Self{
        LoginSystem { logged_in_users: HashMap::new(), database_access: db }
    }
    async fn generate_new_token(&mut self, user:User)->String{
        loop{
            let new_token = nanoid!(30);
            if self.logged_in_users.get(&new_token).is_none(){
                self.logged_in_users.insert(new_token.clone(), user);
                return new_token
            }
        }
    }

}


