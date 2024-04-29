use crate::{
    server::NotecardDBRequest,
    services::server::pogoots::{NotecardLibraryList, NotecardUploadResponse},
    AwsSecrets,
};
use libsql::Connection;
use serde::{Deserialize, Serialize};

use super::{
    database::{self, fetch_user_library, update_notecard_data},
    server::{
        pogoots::{Notecard, NotecardLibraryData, NotecardModifyRequest},
        LoginDBRequest,
    },
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
            NotecardDBRequest::List(request, callback) => {
                println!("Fetch request recieved: {:?}", request);
                //TODO: Implement permissions system
                let auth = request.auth_token;
                let username = request.username;

                let clonecon = conn.clone();
                let verifyerclone = verifyer.clone();
                // let secret_clone = secrets.clone();
                tokio::spawn(async move {
                    let list_result =
                        get_library_with_sql(clonecon, auth, verifyerclone, username).await;

                    let callback_result = if let Ok(result) = list_result {
                        callback.send(NotecardLibraryList {
                            library: result,
                            success: true,
                        })
                    } else {
                        callback.send(NotecardLibraryList {
                            library: Vec::with_capacity(0),
                            success: false,
                        })
                    };

                    if callback_result.is_err() {
                        println!("Callback failed");
                    }
                });
            }
            NotecardDBRequest::Modify(request, callback) => {
                println!("Modify request recieved: {:?}", request);
                // optional NotecardList notecards = 1;
                // optional string auth_token = 3;
                // optional string title = 4;
                // optional string description = 5;
                // optional string tags = 6;
                // optional string school = 7;
                // string username = 8;
                // string ogTitle = 9;
                let clonecon = conn.clone();
                let verifyerclone = verifyer.clone();
                let secrets_clone = secrets.clone();
                // let secret_clone = secrets.clone();
                tokio::spawn(async move {
                    modify_set(verifyerclone, clonecon, secrets_clone, request).await;
                });
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
    let verified = verify_credentials(
        verifyer,
        std::mem::take(&mut data.auth),
        std::mem::take(&mut data.username),
    )
    .await;
    if verified.is_err() {
        println!("Verification failed");
        return Err(());
    }
    if !verified.unwrap() {
        println!("Invalid Credentials");
        return Err(());
    }
    database::store_notecards(conn, list, &mut secrets, data).await
}

async fn get_library_with_sql(
    conn: Connection,
    auth: String,
    verifyer: tokio::sync::mpsc::Sender<LoginDBRequest>,
    username: String,
) -> Result<Vec<NotecardLibraryData>, ()> {
    let verified = verify_credentials(verifyer, auth, username.clone()).await;
    if verified.is_err() {
        println!("Verification failed");
        return Err(());
    }
    if !verified.unwrap() {
        println!("Invalid Credentials");
        return Err(());
    }
    fetch_user_library(&conn, &username).await
}

async fn verify_credentials(
    verifyer: tokio::sync::mpsc::Sender<LoginDBRequest>,
    auth: String,
    username: String,
) -> Result<bool, ()> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let result = verifyer
        .send(LoginDBRequest::VerifyToken(auth, username, tx))
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
        println!("Invalid Credentials");
        return Err(());
    }
    Ok(true)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReMapNotecard {
    front: Vec<String>,
    back: Vec<String>,
}
impl ReMapNotecard {
    pub fn remap(note: Notecard) -> Self {
        ReMapNotecard {
            front: note.front,
            back: note.back,
        }
    }
}

async fn modify_set(
    verifyer: tokio::sync::mpsc::Sender<LoginDBRequest>,
    conn: Connection,
    mut secrets: AwsSecrets,
    mut request: NotecardModifyRequest,
) -> Result<(), ()> {
    let auth_token = std::mem::take(&mut request.auth_token);
    let username = std::mem::take(&mut request.username);
    let verified = verify_credentials(verifyer, auth_token, username).await;

    if verified.is_err() {
        println!("Verification failed");
        return Err(());
    }
    if !verified.unwrap() {
        println!("Invalid Credentials");
        return Err(());
    }

    update_notecard_data(&conn, &mut secrets, request).await
}
