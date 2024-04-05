//A database system that needs to accomplish a few key tasks
//1. Store notecards in a Database
//2. Retreive notecards from a Database
//3. Store user data
//4. Retreive user data
use crate::services::server::NotecardDBRequest;
use crate::AwsSecrets;
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;
use libsql::{params, Connection};
//use libsql_client::{args, Client, Config, ResultSet, Statement, Value};
// use prost::Message;
// use rusqlite::Statement;
// use uuid::Uuid;

use super::notecard::ReMapNotecard;
//NOTECARD STUFF----------------------------------------
pub struct NotecardStorageControllerService {
    access_credentials: AwsSecrets,
    client: Connection,
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

    async fn turso_init(secrets: &AwsSecrets) -> Result<Connection, ()> {
        let url = secrets.turso_url.as_str();
        let url = url.to_string();
        // let config = Config {
        //     url,
        //     auth_token: Some(secrets.auth_token.clone()),
        // };
        // let client = if let Ok(c) = Client::from_config(config).await {
        //     c
        // } else {
        //     return Err(());
        // };
        let client = libsql::Builder::new_remote(url, secrets.auth_token.clone())
            .build()
            .await;
        if client.is_err() {
            return Err(());
        }
        let client = client.unwrap();
        let client = client.connect().unwrap();
        //tracks the users username, password, most recently used ip, and stores more data as
        //rawJSON
        //TODO: Figure out the optimal database setup
        let create_table_result = client
            .execute(
                "CREATE TABLE IF NOT EXISTS USERS(
            USERNAME text,
            PASSWORD text,
            RECENTIPS text
        );",
                (),
            )
            .await;

        if create_table_result.is_err() {
            return Err(());
        }

        // ID text,
        let create_table_result = client
            .execute(
                "CREATE TABLE IF NOT EXISTS NOTECARDS(
            OWNER text,
            NAME text,
            BODY BLOB,
            PERMISSIONS_JSON text
        );",
                (),
            )
            .await;

        if create_table_result.is_err() {
            return Err(());
        }
        // client.clone();
        Ok(client)
    }
    ///Notecard Storage Sequence
    /// Assigns a unique ID to the Notecard Sequence
    /// TODO: Redundancy
    ///
    async fn store_string_notecard(&self, notes: Vec<ReMapNotecard>) -> Result<(), ()> {
        // let id = Uuid::new_v4();
        let json = serde_json::to_string(&notes);
        if json.is_err() {
            println!("Serde To String Error");
            return Err(());
        }
        let json = json.unwrap();
        let compressed = zstd::stream::encode_all(json.as_bytes(), 0);
        if compressed.is_err() {
            return Err(());
        }
        let compressed = compressed.unwrap();
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
    pub async fn store_user_info(
        email: String,
        password: String,
        conn: Connection,
    ) -> Result<(), ()> {
        //         CREATE TABLE IF NOT EXISTS USERS(
        //     USERNAME text,
        //     PASSWORD text,
        //     RECENTIPS text
        //TODO: Verify email
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password.as_bytes(), &salt);
        if password_hash.is_err() {
            return Err(());
        }
        let password_hash = password_hash.unwrap().to_string();

        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = PasswordHash::new(&password_hash);
        if parsed_hash.is_err() {
            return Err(());
        }
        let parsed_hash = parsed_hash.unwrap();
        let result = conn
            .execute(
                "INSERT INTO USERS VALUES (?,?,?)",
                params![email.as_str(), password.as_str(), ""],
            )
            .await;
        if result.is_err() {
            return Err(());
        }
        let result = result.unwrap();
        println!("Inserted into Users: {}", result);
        Ok(())
    }
}
