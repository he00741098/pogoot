use crate::login::UserData;

use super::{config::Config, state_storage::secrets};


///database absctraction
pub struct Database{

    

}

impl Database{
    ///verifies a username and password
    async fn verify_credentials(&self, username:&str, password:&str)->bool{
        //use database connection and argon 2
        todo!()
    }
    pub fn new(config:secrets)->Self{
        Database {  }
    }
    pub async fn generate_uuid(&self)->String{
        todo!()
    }
    pub async fn check_username_exists(&self, username:String)->bool{
        todo!()
    }
    pub async fn store_user_data(&mut self, userData:UserData)->Result<(),()>{
        todo!()
    }
    pub async fn fetch_user_data(&self, username:Option<String>, uuid:Option<String>)->Result<UserData, ()>{
        todo!()
    }
}
