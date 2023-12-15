use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use async_std::sync::Arc;
use std::sync::Mutex;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
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
/// First 4 characters indicate server location, maybe 50 characters total
pub struct SessionTokenStorage{
    used_session_tokens:Vec<String>,
}
///Manage all the logged in users, Intended to be kept in a single thread
pub struct TotalUserManagement{
    pub users:Vec<UserLoggedInTracker>,
    pub sessions:Vec<SessionTokenStorage>,
}
///Long term user data, intended to be stored as json in a database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTermUserDataEntry{
    username:String,
    ips:Vec<String>,
    past_login_dates:Vec<chrono::DateTime<chrono::Utc>>,
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
    pub fn get_new_session_token(&mut self)->String{
        todo!()
    }
    pub fn token_expiration(&mut self)->String{
        todo!()
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