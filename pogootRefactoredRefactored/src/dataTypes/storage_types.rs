use std::collections::{HashMap, HashSet};
use async_std::sync::Arc;
use std::sync::Mutex;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use crate::{dataTypes::request_types::login, database::Database};

use super::request_types::{Request, PayloadType, PayloadData, login::{LoginRequest, LoginRequestPayload, LoginResponse, LoginResponsePayload}, Payload, Response};
///Manage the allocated tokens, attempt to request more if neccessary
///This struct is intended to be created once.
pub struct GameTokenStorage{
    ///Allowed area code, Each server will be given a three digit area code xxx
    /// Total length of the token will be 6 digits. This should be enough
    area_code:String,
    allocated:Arc<OptimalGameTokenStorage>,
    game_tokens:OptimalDataStorage<String, Box<dyn Game>>
}
///Implement a super cool thing to absolutely save massively on compute and memory!
///To only be used in the Game Token Storage Struct
struct OptimalGameTokenStorage{
    ///Token stores will break down into 10 different vectors.
    /// 0-99
    /// 100-199
    /// 200-299
    /// 300-399
    /// 400-499
    /// 500-599
    /// 600-699
    /// 700-799
    /// 800-899
    /// 900-999
    /// TODO: Think of a slick way to make this more optimal
    token_stores:[Mutex<Vec<String>>;10],
}

///Manage a logged in user
pub struct UserLoggedInTracker{
    ///the username of the user
    username:String,
    ///the login token of the user
    token:String,
    ///the ip of the user
    ip:String,
    ///the last time logged-in status was checked
    last_check:Arc<Mutex<std::time::Instant>>,
}
///Manage the allocated session tokens, attempt to request more if neccessary
/// First 4 characters indicate server location, maybe 25 characters total per part
/// Note that overlap may occur if restart occurs because tokens are not saved to disk
/// This should be very rare though so probably fine
pub struct SessionTokenStorage{
    location_code:String,
    used_session_tokens:HashMap<String, chrono::DateTime<chrono::Utc>>,
}
///Manage all the logged in users, Intended to be kept in a single thread
///Fields: users, sessions
pub struct TotalUserManagement{
    pub users:Vec<UserLoggedInTracker>,
    pub sessions:SessionTokenStorage,
    pub database_access:Database,
}
///Long term user data, intended to be stored as json in a database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTermUserDataEntry{
    username:String,
    password:String,
    ///Contains all past ips but also the dates in which they logged in with them
    ips:Vec<(String, chrono::DateTime<chrono::Utc>)>,
    ///Increase in progression, len should theoreticaly match up with ips
    progression:Vec<usize>,
}
///state for axum, contains all the thread addresses and stuff
pub struct CrossThreadState{
    //sender to login_thread
    //sender to game management
}
///Attempt to optimize data storage
pub struct OptimalDataStorage<D, G>{
    vec:Vec<(D, G)>,
    hash:HashMap<D, G>,
    mode:bool,
    ///Switches to other structure when reaching amount of items
    switch_threshold:usize,
    ///Leeway to not switch instantly when dipping slightly or going over slightly
    allowance:usize,
}
pub struct OptimalSingleDataStore<D>{
    vec:Vec<D>,
    hash:HashSet<D>,
    ///True = hashmode, false = vecmode
    mode:bool,
    switch_threshold:usize,
    allowance:usize,
    
}
///the game trait that games should implement
trait Game{

}

impl SessionTokenStorage{
    /// has access to "used_session_tokens:HashSet<String>"
    /// Will get and reserve a session token and return it
    /// note that session tokens are intended to be split
    /// note that if the session store gets to full, this method could recursively loop for a long
    /// time
    /// Should theoretically not happen because there are so many tokens available
    pub fn get_new_session_token(&mut self, len:usize)->String{
        let session_token = nanoid::nanoid!(len);
        let session_token = format!("{}{}",self.location_code, session_token);
        if self.used_session_tokens.get(&session_token).is_some(){
            self.get_new_session_token_retry(len, 1)
        }else{
            self.insert(session_token)
        }
    }
    ///Grabs a token and puts it in the used session system
    pub fn get_new_session_token_retry(&mut self, len:usize, attempt:usize)->String{
        let session_token = nanoid::nanoid!(len);
        let session_token = format!("{}{}",self.location_code, session_token);
        if self.used_session_tokens.get(&session_token).is_some(){
            self.get_new_session_token_retry(len, attempt+1)
        }else{
            self.insert(session_token)
        }
    }
    ///Inserts the token into the hashmap
    ///Does not check for anything
    fn insert(&mut self, session_token:String)->String{
        self.used_session_tokens.insert(session_token.clone(), chrono::Utc::now());
        session_token
    }
    ///The token expiration system. Will be called every once in a while to make tokens expire.
    ///Will have to be manually called by system loop
    ///Will expire tokens 6 hours after they are made
    pub fn token_expiration(&mut self){
        let cur_time = chrono::Utc::now();
        let z = std::mem::take(&mut self.used_session_tokens).into_iter().filter(|(_, time)| cur_time-time>chrono::Duration::hours(6)).collect::<HashMap<String, chrono::DateTime<chrono::Utc>>>();
        self.used_session_tokens=z;
    }
    ///Extends the tokens life by about 6 hours
    pub fn renew_token(&mut self, token:&str)->bool{
        if let Some(token) = self.used_session_tokens.get_mut(token){
            *token = chrono::Utc::now();
            true
        }else{
            false
        }
    }
}
impl OptimalGameTokenStorage{

    pub fn get_new_game_token(&mut self)->String{
        todo!()
    }

    pub fn release_game_token(&mut self, token: String)->bool{
        todo!()
    }
}
impl<D,G> OptimalDataStorage<D,G> where D:Eq + Hash, G:Default+Clone{
    pub fn new(thresh:usize, allowance:usize)->Self{
        OptimalDataStorage{
            vec:vec![],
            hash:HashMap::with_capacity(0),
            mode:false,
            switch_threshold:thresh,
            allowance:10,
        }
    }

    pub fn push(&mut self, item:G, key:D){
        // let mut self = self;
        if self.mode{
            self.hash.insert(key, item);
            if self.switch_threshold-self.allowance>self.hash.len(){
                //switch to vec
                Self::switch_to_vec(self);
            }
        }else{
            self.vec.push((key, item));
            if self.switch_threshold+self.allowance<self.vec.len(){
                //switch to hashmap
                Self::switch_to_hash(self);
            }
        }
    }
    pub fn get(&self, key:D)->Option<&G>{
        if self.mode{

            self.hash.get(&key)

        }else{
            let result = self.vec.iter().find(|x|x.0==key);
            match result{
                Some((_,g))=>{Some(&*g)},
                _=>{None}
            }
        }
    }
    ///probaby not optimal
    /// returns the removed value if found
    pub fn remove(&mut self, key:D)->Option<G>{
        if self.mode{

            self.hash.remove(&key)

        }else{
            let mut returns = None;
            self.vec = std::mem::take(&mut self.vec).into_iter().filter(|x|{
                if x.0==key{
                    returns = Some(x.1.clone());
                }
                x.0!=key
            }).collect();
            match returns{
                Some(mut g)=>{Some(std::mem::take(&mut g))},
                _=>{None}
            }
        }
    }

    fn switch_to_vec(&mut self){
        //check if currently in vec mode
        if !self.mode{
            let hash = std::mem::take(&mut self.hash);
            self.vec=hash.into_iter().collect::<Vec<(D, G)>>();
        }
    }
    fn switch_to_hash(&mut self){
        //check if currently in hash mode
        if self.mode{
            let vec = std::mem::take(&mut self.vec);
            let mut hash = HashMap::<D,G>::with_capacity(vec.len());
            vec.into_iter().for_each(|(d,g)|{hash.insert(d,g);});
            self.hash = hash;
        }
    }
    
}
impl TotalUserManagement{
    /// Should only be called once unless the thread somehow crashes, in which everything will be
    /// done for
    pub fn build_thread(database:Database, location_code:String, used_session_tokens:HashMap<String, chrono::DateTime<chrono::Utc>>)->(mpsc::Receiver<Box<dyn Response + Sync + Send>>, mpsc::Sender<Box<dyn Request + Sync + Send>>){
        let (client_sender, request_reciever) = mpsc::channel(50);
        let (response_sender, response_reciever) = mpsc::channel(25);
        let all_users = TotalUserManagement{
            users:vec![],
            sessions:SessionTokenStorage { location_code, used_session_tokens },
            database_access:database
        };
        tokio::spawn(Self::thread(all_users,request_reciever, response_sender));
        //start total user mngmnt
        (response_reciever, client_sender)
    }
    async fn thread(mut user_management:TotalUserManagement,mut request_reciever:mpsc::Receiver<Box<dyn Request + Sync + Send>>, response_sender:mpsc::Sender<Box<dyn Response + Sync + Send>>){
        //load balance to others
        while let Some(mut request) = request_reciever.recv().await{
            match request.get_type(){
                PayloadType::Login=>{
                    let payload = request.get_payload();
                    Self::login(&mut user_management, payload);
                },
                PayloadType::Register=>{

                },
                PayloadType::Note=>{

                },
                PayloadType::Game=>{

                }
            }
        }
    }
    fn login(user_management:&mut TotalUserManagement, payload:PayloadData)->login::LoginResponse{
        let result = payload.grab_login_request_data();
        if result.is_err(){
            LoginResponse{
                payload:LoginResponsePayload{
                    payload:PayloadData::LoginResponse("login failed because payload did not contain the correct login request data".to_string(), None)
                }
            }
        }else{
            let (username, password, ip) = result.unwrap();
            user_management.sessions.get_new_session_token(50);
            
            todo!()
        }
    }
    fn register(payload:PayloadData){
        todo!()
    }
    fn note(payload:PayloadData){
        todo!()
    }
    fn game(payload:PayloadData){
        todo!()
    }
}
