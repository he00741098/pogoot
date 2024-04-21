//A database system that needs to accomplish a few key tasks
//1. Store notecards in a Database
//2. Retreive notecards from a Database
//3. Store user data
//4. Retreive user data
use crate::AwsSecrets;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;
use libsql::{params, Connection};
use uuid::Uuid;

use super::notecard::{NotecardData, ReMapNotecard};
//NOTECARD STUFF----------------------------------------

pub async fn new_connection(secrets: AwsSecrets) -> Option<Connection> {
    let client_result = turso_init(&secrets).await;
    if client_result.is_err() {
        println!("Turso init failed");
        return None;
    }
    let client = client_result.unwrap();

    Some(client)
}

async fn turso_init(secrets: &AwsSecrets) -> Result<Connection, ()> {
    let dev_build_mode = true;
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
        println!("Client build failed: {:?}", client);
        return Err(());
    }
    let client = if dev_build_mode {
        libsql::Builder::new_local(":memory:").build().await
    } else {
        client
    };
    let client = client.unwrap();
    let client = client.connect().unwrap();
    //tracks the users username, password, most recently used ip, and stores more data as
    //rawJSON
    //TODO: Figure out the optimal database setup
    let create_table_result = client
        .execute(
            "CREATE TABLE IF NOT EXISTS USERS(
            USERNAME text,
            EMAIL text,
            PASSWORD text,
            RECENTIPS text
        );",
            (),
        )
        .await;

    if create_table_result.is_err() {
        println!(
            "Turso table creation failed for USERS: {:?}",
            create_table_result
        );
        return Err(());
    }

    // ID text,
    let create_table_result = client
        .execute(
            "CREATE TABLE IF NOT EXISTS NOTECARDS(
            OWNER text,
            NAME text,
            BODY BLOB,
            PERMISSIONS_JSON text,
            DESCRIPTION text,
            TAGS text,
            SCHOOL text,
            CFID text
        );",
            (),
        )
        .await;

    if create_table_result.is_err() {
        println!("Turso table creation failed for NOTECARDS");
        return Err(());
    }
    // client.clone();
    Ok(client)
}
///Notecard Storage Sequence
/// Assigns a unique ID to the Notecard Sequence
/// TODO: Redundancy
///
pub async fn store_notecards(
    conn: Connection,
    notes: Vec<ReMapNotecard>,
    secrets: &mut AwsSecrets,
    data: NotecardData,
) -> Result<String, ()> {
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
    let id = Uuid::new_v4();
    let id = format!("{}{}", data.username, id);
    let result = conn
        .execute(
            "INSERT INTO NOTECARDS VALUES (?,?,?,?,?,?,?,?);",
            //username, email, password, ips
            params![
                data.username,
                data.title,
                "".as_bytes(),
                "",
                data.desc,
                data.tags,
                data.school,
                id.clone()
            ],
        )
        .await;
    if result.is_err() {
        return Err(());
    }

    let result =
        crate::services::cfstorage::upload_notecard_to_cloudflare(secrets, compressed, &id).await;
    if result.is_err() {
        println!("Notecard Store in Cloudflare Failed");
        //TODO: Handle failure
        return Err(());
    }
    Ok(id)
}
pub async fn store_user_info(email: String, password: String, conn: &Connection) -> Result<(), ()> {
    //     CREATE TABLE IF NOT EXISTS USERS(
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
        println!("Password hashing failed: {:?}", password_hash);
        return Err(());
    }
    let result = conn
        .execute(
            "INSERT INTO USERS VALUES (?,?,?,?);",
            //username, email, password, ips
            params![
                email.as_str(),
                email.as_str(),
                password_hash.unwrap().to_string().as_str(),
                ""
            ],
        )
        .await;
    if result.is_err() {
        println!("Database Insertion failed: {:?}", result);
        return Err(());
    }
    let result = result.unwrap();
    println!("Inserted into Users: {}", result);
    Ok(())
}

///checks if an email exists in the database. If it does, it will return the password
pub async fn check_email_exists(conn: &Connection, email: &str) -> Result<Option<String>, ()> {
    let result = conn
        .query(
            "SELECT PASSWORD FROM USERS WHERE EMAIL = ?1 OR USERNAME = ?1;",
            params![email],
        )
        .await;
    if result.is_err() {
        println!("Database Query was error");
        return Err(());
    }
    let mut rows = result.unwrap();
    match rows.next().await {
        Ok(Some(row)) => {
            let password = row.get_str(0);
            if password.is_err() {
                println!("row index is not TEXT");
                return Err(());
            }
            Ok(Some(password.unwrap().to_string()))
        }
        Ok(None) => Ok(None),
        Err(_) => {
            println!("Rows errored");
            Err(())
        }
    }
    // Ok(None)
}
