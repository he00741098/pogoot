use crate::login::UserData;

use super::{config::Config, state_storage::secrets, questionList};


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
    ///Generates a unique user Id that is not present in the table
    pub async fn generate_uuid(&self)->String{
        todo!()
    }
    ///Checks if the username is currently in the database
    pub async fn check_username_exists(&self, username:String)->bool{
        todo!()
    }
    ///Stores userdata in the database
    pub async fn store_user_data(&mut self, userData:UserData)->Result<(),()>{
        todo!()
    }
    ///Gets userdata from the database
    pub async fn fetch_user_data(&self, username:Option<String>, uuid:Option<String>)->Result<UserData, ()>{
        todo!()
    }
    ///Stores note card and returns generated id
    pub async fn store_note_card_set(&self, questionsList:questionList, username:String)->String{
        todo!()
    }
    ///Takes an ID and full list of questions. Replaces current entry. If current entry does not exist,
    /// it will add a new entry
    /// Requires user to have own or have permissions to the set
    pub async fn edit_note_card_set(&self, id:String, editedList:questionList, token:String){
        todo!()
    }
}
