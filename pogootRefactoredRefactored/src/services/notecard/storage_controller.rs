use std::sync::Arc;

use crate::services::database::{Database, CoreDatatypeError};

use super::NoteCardVariants;
pub struct NotecardStorageManager{
    
}

impl NotecardStorageManager{
    pub async fn notecard_store(username:String, set_name:String, notecard:NoteCardVariants, database_access:Arc<Database>)->Result<String,()>{
        let notecard_rawjson = serde_json::to_string(&notecard);
        if notecard_rawjson.is_err(){return Err(())}
        let notecard_rawjson = notecard_rawjson.unwrap();
        let database_action_result = database_access.store_note_card(notecard_rawjson, username, set_name).await;
        if database_action_result.is_err(){
            Err(())
        }else{
            Ok(database_action_result.unwrap())
        }
    }
    pub async fn retrieve_from_storage(set_id:String, database_access:Database)->Result<NoteCardVariants, CoreDatatypeError>{
        let notecard = database_access.fetch_note_card(set_id).await;
        if let Err(error) = notecard{
            return Err(error)
        }else{
            let parse = serde_json::from_str::<NoteCardVariants>(&notecard.unwrap());
            if parse.is_err(){
                return Err(CoreDatatypeError::ParseFailed)
            }else{
                let parsed = parse.unwrap();
                return Ok(parsed)
            }
        }
    }
}
