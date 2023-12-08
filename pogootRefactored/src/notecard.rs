

//notecard endpoints
//Store notecard
//require login
//
//allow conversion to pogoot game

use std::sync::Arc;

use crate::dataTypes::{state_storage::state_storage, questionList};

///Takes the user's token and stores the questions list in the database
///Token must be valid. Returns the ID of the stored questionlist.
/// Adds set to userdata
pub async fn upload_set(token:String, questions:questionList, state:Arc<state_storage>)->String{
    todo!()
}

///Returns the set that has the id
pub async fn get_set(id:String, state:Arc<state_storage>)->questionList{
    todo!()
}

///Returns all the ids of a user's sets
pub async fn get_users_sets(username:String, state:Arc<state_storage>)->Vec<String>{
todo!()
}

