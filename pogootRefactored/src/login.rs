use std::{time::Duration, collections::{HashMap, HashSet}};

use nanoid::nanoid;
use tokio::sync::mpsc::{Receiver, Sender};
use chrono::{Utc, DateTime, serde::ts_milliseconds};
use serde::{Serialize, Deserialize};
use tracing::info;

use crate::dataTypes::database::Database;
//login system involves a main thread with a channel that takes login requests and redirects them to
//more private threads that will proccess the request

pub struct Login{
    //.Database abstraction to interact with external database
    database:Database,
    ///Token storage - Purely for not generating matching tokens
    tokens:TokenStorage,
}

struct TokenStorage{
    tokens_short:Vec<String>,
    tokens_long:HashSet<String>,
    ///false for vec, true for hashSet
    hash_mode:bool
}
impl TokenStorage{
    pub fn new()->Self{
        TokenStorage { tokens_short: vec![], tokens_long: HashSet::with_capacity(0), hash_mode: false }
    }
    ///Generates a token that does not exist in the token storage
    fn generate_token(&self)->String{
        let generated_token = nanoid!(20);
        if self.exists(generated_token.clone()){
            self.generate_token()
        }else{
            generated_token
        }
    }
    pub fn exists(&self, token:String)->bool{
        if self.hash_mode{
            self.exists_long(token)
        }else{
            self.exists_short(token)
        }
    }
    fn exists_short(&self, token:String)->bool{
        if self.tokens_short.contains(&token){
            true
        }else{
            false
        }
    }
    fn exists_long(&self, token:String)->bool{
        if self.tokens_long.contains(&token){
            true
        }else{
            false
        }
    }
    fn add_entry(&mut self, token:String){
        if self.hash_mode{
            self.tokens_long.insert(token);
        }else{
            self.tokens_short.push(token);
        }   
    }
    pub fn add_and_get_new_token(&mut self)->String{
        let token = self.generate_token();
        self.add_entry(token.clone());
        self.mode_shifter();
        token
    }
    ///mode shifter will shift the internals to try to "optimize" preformance
    fn mode_shifter(&mut self){
        if !self.hash_mode{
            if self.tokens_short.len()>200{
                self.convert_to_hash_set();
            }
        }else if self.hash_mode{
            if self.tokens_long.len()<180{
                self.convert_to_vec();
            }
        }
    }
    ///converts the internal vector to a hash set
    fn convert_to_hash_set(&mut self){
        if !self.hash_mode{
        self.tokens_long=HashSet::with_capacity(self.tokens_short.len()+20);
        let steal = std::mem::replace(&mut self.tokens_short,Vec::with_capacity(0));
        steal.into_iter().for_each(|x|{self.tokens_long.insert(x);});
        self.hash_mode=true;
        }else{
            info!("Attempted to convert to hash mode when in hash_mode");
        }
    }
    ///converts the internal hashset to a vector
    fn convert_to_vec(&mut self){
        if self.hash_mode{
        self.tokens_short=Vec::with_capacity(self.tokens_long.len()+10);
        let steal = std::mem::replace(&mut self.tokens_long, HashSet::with_capacity(0));
        steal.into_iter().for_each(|x|self.tokens_short.push(x));
        self.hash_mode=false;
        }else{
            info!("Attempted to convert to vec mode when not in hash_mode");
        }
    }
}

pub struct loginRequest{
    ///true for login, false for register
    request_type:bool,
    ///username
    username:String,
    ///password
    password:String,
    ///callback, success or not
    callback:tokio::sync::oneshot::Sender<bool>
}

impl Login{
    ///private function to login users
    fn login(&mut self, login:loginRequest)->bool{
        todo!();
    }
    ///private function to register users - generate user data
    fn register(&mut self, login:loginRequest)->bool{
        //check date, put date in database, check IP, etc
        todo!();
    }
    ///public function to start the login thread
    pub fn start_thread(database:Database)->Sender<loginRequest>{
        let login_thread = Login{database, tokens:TokenStorage::new()};
        let (tx, rx) = tokio::sync::mpsc::channel::<loginRequest>(100);
        tokio::spawn(login_thread.login_thread(rx));
        tx
    }
    ///private function to start thread
    async fn login_thread(self, rx:Receiver<loginRequest>){
        //login, generate token, deal with stuff, send bool over callback
        todo!()
    }

}

///Ip list records the ips logged in and will also keep track of most recent logins.
#[derive(Clone, Serialize, Deserialize, Debug)]
struct IpList{
    //TODO, record dates as well
    ip_list:Vec<IpEntry>
}
#[derive(Clone, Serialize, Deserialize, Debug)]
struct IpEntry{
    ///Ip of the user
    ip:String,
    ///The first time they logged in with the ip
    #[serde(with = "ts_milliseconds")]
    time:DateTime<Utc>,
    ///Most recent time they logged in with the ip
    #[serde(with = "ts_milliseconds")]
    recent_time:DateTime<Utc>,
}
impl IpEntry{
    pub fn update_time(&mut self){
        self.recent_time=Utc::now();
    }
}
impl IpList{
    ///returns true if the vector contains the ip
    pub fn ip_exists(&self, x:&str)->bool{
        if self.ip_list.iter().map(|x|x.ip.clone()).collect::<Vec<String>>().contains(&x.to_string()){
            true
        }else{
            false
        }
    }
    ///Adds an entry -DOES NOT CHECK IF ENTRY ALREADY EXISTS- and sets the time
    pub fn addEntry(&mut self, ip:String){
        let ip_entry = IpEntry{
            ip,
            time:Utc::now(),
            recent_time:Utc::now()
        };
        self.ip_list.push(ip_entry);
    }
    ///Checks if an ip exists in the vector. Adds ip if not exists. Returns true if exists, returns false if does not exist.
    pub fn exists_or_add(&mut self, ip:String)->bool{
        if !self.ip_exists(&ip){
            self.addEntry(ip);
            false
        }else{
            true
        }
    }
}

///Userdata used to keep track of the user
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserData{
    ///List of previous Logged in Ips
    ips:IpList,
    ///Total playtime
    playtime:Duration,
    ///Username
    username:String,
    ///Password
    password:String,
    ///Optional email
    email:Option<String>,
    ///contains a the id to an entry in the history storage (Coming soon?)
    history:String,
    ///User Id - a uuid
    uuid:String,
}

struct Token{
    token:String,
    time_left:Duration,
}