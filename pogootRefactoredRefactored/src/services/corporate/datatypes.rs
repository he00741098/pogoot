use serde::{Serialize, Deserialize};

use crate::services::notecard::NoteCardVariants;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCardUploadRequest{

    notecard_varient:NoteCardVariants,
    session_token:String,
}