//A database system that needs to accomplish a few key tasks
//1. Store notecards in a Database
//2. Retreive notecards from a Database
//3. Store user data
//4. Retreive user data

use crate::services::server::NotecardDBRequest;
use crate::AwsSecrets;
use libsql_client::{args, Client, Config, ResultSet, Statement, Value};
use prost::Message;
// use rusqlite::Statement;
use uuid::Uuid;

use super::notecard::ReMapNotecard;
//NOTECARD STUFF----------------------------------------
pub struct NotecardStorageControllerService {
    access_credentials: AwsSecrets,
    client: Client,
}

impl NotecardStorageControllerService {
    async fn new(secrets: AwsSecrets) -> Option<Self> {
        let client_result = Self::turso_init(&secrets).await;
        if client_result.is_err() {
            return None;
        }
        let client = client_result.unwrap();

        let dbobj = NotecardStorageControllerService {
            access_credentials: secrets,
            client,
        };
        Some(dbobj)
    }

    async fn turso_init(secrets: &AwsSecrets) -> Result<Client, ()> {
        let url = secrets.turso_url.as_str().try_into();
        if url.is_err() {
            return Err(());
        }
        let url = url.unwrap();
        let config = Config {
            url,
            auth_token: Some(secrets.auth_token.clone()),
        };
        let client = if let Ok(c) = Client::from_config(config).await {
            c
        } else {
            return Err(());
        };
        //tracks the users username, password, most recently used ip, and stores more data as
        //rawJSON
        //TODO: Figure out the optimal database setup
        let create_table_result = client
            .execute(
                "CREATE TABLE IF NOT EXISTS POGOOT(
            USERNAME text,
            PASSWORD text,
            RECENTIP text,
            RAWJSON text,
            VERSION INT
        );",
            )
            .await;
        if create_table_result.is_err() {
            return Err(());
        }
        let create_table_result = client
            .execute(
                "CREATE TABLE IF NOT EXISTS NOTECARDS(
            USERNAME text,
            ID text,
            NAME text,
            PERMISSIONS_JSON text,
            VERSION INT
        );",
            )
            .await;
        if create_table_result.is_err() {
            return Err(());
        }
        Ok(client)
    }
    ///Notecard Storage Sequence
    /// Assigns a unique ID to the Notecard Sequence
    /// TODO: Redundancy
    ///
    async fn store_string_notecard(&self, notes: Vec<ReMapNotecard>) -> Result<(), ()> {
        let id = Uuid::new_v4();
        let json = serde_json::to_string(&notes);
        if json.is_err() {
            println!("Serde To String Error");
            return Err(());
        }
        // let stmt = Statement::with_args(
        //     r"INSERT INTO NOTECARDS VALUES (?, ?, ?, ?, ?, ?);",
        //     args!(
        //         // &username,
        //         // &notecard_id,
        //         // name,
        //         // parsed_result.unwrap(),
        //         // notecard_rawjson,
        //         // 1
        //     ),
        // );

        todo!()
    }
    // async fn get_user_from_auth_token() {}
}
