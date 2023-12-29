mod permissions_management;
mod long_term_user_management;
mod short_term_user_management;
mod user_management_datatypes;

use serde::{Deserialize, Serialize};

use super::database::Database;
// USERNAME text,
// PASSWORD text,
// RECENTIP text,
// RAWJSON text,
// VERSION INT
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User{
    pub username:String,
    pub password:String,
    pub most_recent_ip:String,
    pub ips:Vec<String>,
    pub unique_id:String,
    pub is_banned:bool,
    pub deleted:bool,
}
impl User{
    pub fn new(username:String, password:String, recent_ip:String)->Self{
        User { username , password , most_recent_ip: recent_ip.clone(), ips: vec![recent_ip], unique_id: Database::generate_random_id(), is_banned:false, deleted:false }
    }
}
