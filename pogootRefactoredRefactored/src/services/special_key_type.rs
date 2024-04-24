//Use case
//This is a special key type in order to have less hashmaps in user management
//The idea is a token, a username, or an email will be able to get the same user in the same hashmap

use std::{collections::HashMap, hash::Hash, sync::Arc};
use tokio::sync::Mutex;

use super::user_manage::User;

///The usermap will allow lookups with any type
pub struct UserManageMap {
    usermap: HashMap<String, Arc<Mutex<User>>>,
    tokenmap: HashMap<String, Arc<Mutex<User>>>,
    emailmap: HashMap<String, Arc<Mutex<User>>>,
}

impl UserManageMap {
    ///Triple the cost of a regular lookup
    pub fn get(&self, key: String) -> Option<Arc<Mutex<User>>> {
        if let Some(user) = self.usermap.get(&key) {
            Some(user.clone())
        } else if let Some(user) = self.tokenmap.get(&key) {
            Some(user.clone())
        } else {
            self.emailmap.get(&key).cloned()
        }
    }
    ///Double cost lookup - gets from usermap or emailmap
    pub fn get_with_user_or_email(&self, email: &String) -> Option<Arc<Mutex<User>>> {
        if let Some(user) = self.usermap.get(email) {
            Some(user.clone())
        } else {
            self.emailmap.get(email).cloned()
        }
    }

    ///Regular lookup - only tokens
    pub fn get_with_token(&self, token: &String) -> Option<Arc<Mutex<User>>> {
        self.tokenmap.get(token).cloned()
    }
    ///Regular lookup - only tokens
    pub fn get_with_username(&self, token: &String) -> Option<Arc<Mutex<User>>> {
        self.usermap.get(token).cloned()
    }
    ///Regular lookup - only tokens
    pub fn get_with_email(&self, token: &String) -> Option<Arc<Mutex<User>>> {
        self.emailmap.get(token).cloned()
    }
    pub fn new() -> Self {
        UserManageMap {
            usermap: HashMap::with_capacity(100),
            tokenmap: HashMap::with_capacity(100),
            emailmap: HashMap::with_capacity(100),
        }
    }
    pub fn insert(
        &mut self,
        email: String,
        token: String,
        username: String,
        user: Arc<Mutex<User>>,
    ) {
        // let user = Arc::new(Mutex::new(user));
        self.usermap.insert(username, user.clone());
        self.tokenmap.insert(token, user.clone());
        self.emailmap.insert(email, user);
    }
}
