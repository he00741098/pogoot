use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::*;
//the database abstraction
pub struct Database{

}
impl Database{
    pub fn verify_password(){

    }
    fn try_to_get_secrets(){
        let mut contents = String::new();
        let mut file = File::open("DBSecrets.toml").expect("to open file DBSecrets.toml");
        file.read_to_string(&mut contents).expect("to put file contents in string");
        let db_secrets:DBSecrets = toml::from_str(&contents).unwrap(); 
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DBSecrets{
    turso_url:String,
    auth_token:String,
}
