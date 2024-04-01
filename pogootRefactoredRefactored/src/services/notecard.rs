use crate::{
    server::NotecardDBRequest, services::server::pogoots::NotecardUploadResponse, AwsSecrets,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};

use super::server::pogoots::Notecard;

pub async fn upload_proccessor(
    mut reciever: tokio::sync::mpsc::Receiver<NotecardDBRequest>,
    secrets: AwsSecrets,
) {
    while let Some(request) = reciever.recv().await {
        match request {
            NotecardDBRequest::Store(Request, callback) => {
                println!("Store Request Recieved: {:?}", Request);
                let auth = Request.auth_token;
                if let Some(set) = Request.notecards {
                    let notes = set.notecards;
                    let notes = notes
                        .into_iter()
                        .map(ReMapNotecard::remap)
                        .collect::<Vec<ReMapNotecard>>();
                    store_with_sql(notes, auth).await;
                } else {
                    let response = NotecardUploadResponse {
                        success: false,
                        id: "placeholder".to_string(),
                    };
                    let callback_result = callback.send(response);
                    if callback_result.is_err() {
                        println!("Callback errored when sending response.");
                    }
                }
            }
            NotecardDBRequest::Fetch(ID, callback) => {
                todo!()
            }
            NotecardDBRequest::Modify(Request, callback) => {
                todo!()
            }
        }
    }
}

async fn store_with_sql(list: Vec<ReMapNotecard>, auth: String) {
    // let json = serde_json::to_string(&list);
    todo!()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReMapNotecard {
    front: Vec<String>,
    back: Vec<String>,
}
impl ReMapNotecard {
    fn remap(note: Notecard) -> Self {
        ReMapNotecard {
            front: note.front,
            back: note.back,
        }
    }
}
