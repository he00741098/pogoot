use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use async_std::sync::Arc;
use std::sync::Mutex;
///Manage the allocated tokens, attempt to request more if neccessary
///This struct is intended to be created once.
pub struct GameTokenStorage{
    allocated:Arc<RwLock<Allocation>>,
    game_tokens:OptimalDataStorage<String, Box<dyn Game>>
}
pub struct Allocation{
    ///a list of the valid characters that are allocated
    valid_chars:Vec<ValidChar>,
    ///A list of ranges that are allocated - Note that the usizes represent a range of indexes in
    ///the valid_chars vec
    ///Eg. allocating 1234-1235 with a valid_char of [1,2,3,4,5] would look like
    ///[(0,1),(1,2),(2,3),(3,5)]
    allocated:Vec<(usize, usize)>,
    ///A list of all values that are occupied
    ///represents them as usizes
    ///Eg. 1234 in 1234-1235 would be 0, 1235 would be 1
    occupied:Vec<usize>
}
enum ValidChar{
    Number(usize),
    Char(char),
}
///Manage a logged in user
struct UserLoggedInTracker{
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
pub struct SessionTokenStorage{
    
}
///Manage all the logged in users
pub struct TotalUserManagement{

}
///state for axum
pub struct CrossThreadState{

}
///Attempt to optimize data storage
pub struct OptimalDataStorage<D, G>{
    vec:Vec<(D, G)>,
    hash:HashMap<D, G>,
    mode:bool
}
pub struct OptimalSingleDataStore<D>{
    vec:Vec<D>,
    hash:HashSet<D>,
    mode:bool
    
}
///the game trait that games should implement
trait Game{

}
