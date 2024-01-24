use serde::{Serialize, Deserialize};

use crate::services::notecard::NoteCardVariants;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCardUploadRequest{
    pub set_name: String,
    pub notecard_varient:NoteCardVariants,
    pub session_token:String,
    pub username:String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCardGetRequest{
    pub set_name: String,
    pub session_token:String,
    pub username:String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCardGetRequestPart2{
    pub notecard_id: String,
    pub session_token:String,
    pub username:String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCardGetResponse{
    pub set_name: String,
    pub notecard_varient:NoteCardVariants
}
