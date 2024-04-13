use crate::{
    server::NotecardDBRequest, services::server::pogoots::NotecardUploadResponse, AwsSecrets,
};
use libsql::Connection;
use serde::{Deserialize, Serialize};

use super::{
    database,
    server::{pogoots::Notecard, LoginDBRequest},
};

pub async fn upload_proccessor(
    conn: Connection,
    mut reciever: tokio::sync::mpsc::Receiver<NotecardDBRequest>,
    verifyer: tokio::sync::mpsc::Sender<LoginDBRequest>,
    secrets: AwsSecrets,
) {
    while let Some(request) = reciever.recv().await {
        match request {
            NotecardDBRequest::Store(request, callback) => {
                println!("Store Request Recieved: {:?}", request);

                let auth = request.auth_token;
                let title = request.title;
                let school = request.school;
                let tags = request.tags;
                let description = request.description;
                let data = NotecardData {
                    auth,
                    title,
                    school,
                    tags,
                    desc: description,
                    username: request.username,
                };

                if let Some(set) = request.notecards {
                    let notes = set.notecards;
                    let notes = notes
                        .into_iter()
                        .map(ReMapNotecard::remap)
                        .collect::<Vec<ReMapNotecard>>();

                    println!("{:?}", notes);
                    let clonecon = conn.clone();
                    let verifyerclone = verifyer.clone();
                    let secret_clone = secrets.clone();
                    tokio::spawn(async move {
                        let store_result =
                            store_with_sql(clonecon, notes, data, verifyerclone, secret_clone)
                                .await;

                        let callback_result = if let Ok(result) = store_result {
                            callback.send(NotecardUploadResponse {
                                success: true,
                                id: result,
                            })
                        } else {
                            callback.send(NotecardUploadResponse {
                                success: false,
                                id: "Upload failed".to_string(),
                            })
                        };

                        if callback_result.is_err() {
                            println!("Callback failed");
                        }
                    });
                } else {
                    let response = NotecardUploadResponse {
                        success: false,
                        id: "Upload Failed, No Notecards Provided".to_string(),
                    };
                    let callback_result = callback.send(response);
                    if callback_result.is_err() {
                        println!("Callback errored when sending response.");
                    }
                }
            }
            NotecardDBRequest::Fetch(id, callback) => {
                todo!()
            }
            NotecardDBRequest::Modify(request, callback) => {
                todo!()
            }
        }
    }
}
pub struct NotecardData {
    pub auth: String,
    pub title: String,
    pub school: String,
    pub tags: String,
    pub desc: String,
    pub username: String,
}

async fn store_with_sql(
    conn: Connection,
    list: Vec<ReMapNotecard>,
    mut data: NotecardData,
    verifyer: tokio::sync::mpsc::Sender<LoginDBRequest>,
    mut secrets: AwsSecrets,
) -> Result<String, ()> {
    //TODO:Verify login
    let (tx, rx) = tokio::sync::oneshot::channel();
    let result = verifyer
        .send(LoginDBRequest::VerifyToken(
            std::mem::take(&mut data.auth),
            data.username.clone(),
            tx,
        ))
        .await;
    if result.is_err() {
        println!("Verifyer channel failed somehow!!!");
        return Err(());
    }
    let result = rx.await;
    if result.is_err() {
        println!("Callback Channel Failed to recieve, verifier dropped the channel");
        return Err(());
    }
    if !result.unwrap() {
        println!("Not Logged In");
        return Err(());
    }

    database::store_notecards(conn, list, &mut secrets, data).await
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
