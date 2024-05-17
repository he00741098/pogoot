use std::sync::Arc;

//A database system that needs to accomplish a few key tasks
//1. Store notecards in a Database
//2. Retreive notecards from a Database
//3. Store user data
//4. Retreive user data
use crate::AwsSecrets;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;
use libsql::{params, params_from_iter, Connection, Database};
use uuid::Uuid;

use super::notecard::{NotecardData, ReMapNotecard};
use super::server::pogoots::{NotecardLibraryData, NotecardModifyRequest};
//NOTECARD STUFF----------------------------------------

pub async fn new_connection(secrets: AwsSecrets) -> Option<Database> {
    let client_result = turso_init(&secrets).await;
    if client_result.is_err() {
        println!("Turso init failed");
        return None;
    }
    let client = client_result.unwrap();

    Some(client)
}

async fn turso_init(secrets: &AwsSecrets) -> Result<Database, ()> {
    let dev_build_mode = false;
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
    let build = libsql::Builder::new_remote(url, secrets.auth_token.clone())
        .build()
        .await;
    if build.is_err() {
        println!("Client build failed: {:?}", build);
        return Err(());
    }
    let build = if dev_build_mode {
        libsql::Builder::new_local(":memory:").build().await
    } else {
        build
    };
    let db = build.unwrap();
    let client = db.connect().unwrap();
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
            CFID text,
            CREATION_DATE text
        );",
            (),
        )
        .await;

    if create_table_result.is_err() {
        println!("Turso table creation failed for NOTECARDS");
        return Err(());
    }
    // client.clone();
    Ok(db)
}
///Notecard Storage Sequence
/// Assigns a unique ID to the Notecard Sequence
/// TODO: Redundancy
///
pub async fn store_notecards(
    conn: Arc<Database>,
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
    let id = format!("{}", id);
    let now = chrono::Utc::now();
    let now = now.to_string();
    let conn = conn.connect();
    if conn.is_err() {
        println!("Connection error!");
        return Err(());
    }
    let conn = conn.unwrap();

    let result = conn
        .execute(
            "INSERT INTO NOTECARDS VALUES (?,?,?,?,?,?,?,?,?);",
            //username, email, password, ips
            params![
                data.username,
                data.title,
                "".as_bytes(),
                "",
                data.desc,
                data.tags,
                data.school,
                id.clone(),
                now
            ],
        )
        .await;
    if result.is_err() {
        println!("Notecard Store Into SQLITE FAILED");
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
pub async fn store_user_info(
    username: &str,
    email: &str,
    password: String,
    conn: &Connection,
) -> Result<(), ()> {
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
                username,
                email,
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
///Returns a tuple (Password, Username, email)
pub async fn check_email_or_username_exists(
    conn: &Connection,
    email: &str,
) -> Result<Option<(String, String, String)>, ()> {
    let result = conn
        .query(
            "SELECT PASSWORD, USERNAME, EMAIL FROM USERS WHERE EMAIL = ?1 OR USERNAME = ?1 LIMIT 1;",
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
            let username = row.get_str(1);
            let email = row.get_str(2);
            if password.is_err() {
                println!("row index is not TEXT");
                return Err(());
            }
            Ok(Some((
                password.unwrap().to_string(),
                username.unwrap().to_string(),
                email.unwrap().to_string(),
            )))
        }
        Ok(None) => Ok(None),
        Err(_) => {
            println!("Rows errored");
            Err(())
        }
    }
    // Ok(None)
}

pub async fn check_email_exists(
    conn: &Connection,
    email: &str,
    username: &str,
) -> Result<Option<String>, ()> {
    let result = conn
        .query(
            "SELECT PASSWORD FROM USERS WHERE EMAIL = ?1 OR USERNAME = ?2;",
            params![email, username],
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

pub async fn fetch_user_library(
    conn: &Connection,
    username: &str,
) -> Result<Vec<NotecardLibraryData>, ()> {
    // OWNER text,
    // NAME text,
    // BODY BLOB,
    // PERMISSIONS_JSON text,
    // DESCRIPTION text,
    // TAGS text,
    // SCHOOL text,
    // CFID text
    //Grabbing ALL notecards that fit the criteria...
    let result = conn
        .query(
            "SELECT NAME, DESCRIPTION, TAGS, SCHOOL, CFID, CREATION_DATE FROM NOTECARDS WHERE OWNER = ?1;",
            params![username],
        )
        .await;
    if result.is_err() {
        println!("Database Query was error: {:?}", result.err());
        return Err(());
    }
    let mut rows = result.unwrap();
    let mut accumulate = vec![];
    //loops until empty and returns accumulate
    //If an error occurs, everything is over
    while let Ok(rows) = rows.next().await {
        match rows {
            Some(row) => {
                println!("Row: {:?}", row);
                let name = row.get_str(0);
                let desc = row.get_str(1);
                let tags = row.get_str(2);
                let school = row.get_str(3);
                let cfid = row.get_str(4);
                let date = row.get_str(5);
                if name.is_err()
                    || desc.is_err()
                    || tags.is_err()
                    || school.is_err()
                    || cfid.is_err()
                    || date.is_err()
                {
                    println!("row index is not TEXT");
                    return Err(());
                }
                accumulate.push(NotecardLibraryData {
                    title: name.unwrap().to_string(),
                    school: school.unwrap().to_string(),
                    tags: tags.unwrap().to_string(),
                    desc: desc.unwrap().to_string(),
                    cfid: cfid.unwrap().to_string(),
                    date: date.unwrap().to_string(),
                })
            }
            None => {
                println!("Rows Is None");
                return Ok(accumulate);
            }
        }
    }
    println!("Rows errored");
    Err(())
}

// let notecards = request.notecards;
// let title = request.title;
// let description = request.description;
// let tags = request.tags;
// let school = request.school;
pub async fn update_notecard_data(
    conn: &Connection,
    secrets: &mut AwsSecrets,
    request: NotecardModifyRequest,
) -> Result<(), ()> {
    let notecards = request.notecards;
    let title = request.title;
    let description = request.description;
    let tags = request.tags;
    let school = request.school;
    let cfid = request.cfid;
    let query = "UPDATE NOTECARDS SET".to_string();
    let ending = "WHERE CFID=?;";
    // let now = chrono::Utc::now();
    let conversion = &["NAME", "DESCRIPTION", "TAGS", "SCHOOL"];
    let strings = vec![title, description, tags, school];
    let strings = strings
        .into_iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|x| (x.0, x.1.unwrap()))
        .collect::<Vec<(usize, String)>>();
    let query = if strings.is_empty() {
        format!("{} CREATION_DATE=? {}", query, ending)
    } else if strings.len() == 1 {
        format!(
            "{} CREATION_DATE=?,{}=? {}",
            query, conversion[strings[0].0], ending
        )
    } else if strings.len() == 2 {
        format!(
            "{} {}=?, {}=? {}",
            query, conversion[strings[0].0], conversion[strings[1].0], ending
        )
    } else {
        let mut temp_formatter =
            format!("{} CREATION_DATE=?,{}=?,", query, conversion[strings[0].0]);
        for b in 1..strings.len() - 1 {
            temp_formatter = format!("{}{}=?,", temp_formatter, conversion[strings[b].0]);
        }
        temp_formatter = format!(
            "{}{}=? {}",
            temp_formatter,
            conversion[strings[strings.len() - 1].0],
            ending
        );
        temp_formatter
    };
    let strings = strings.into_iter().map(|x| x.1).collect::<Vec<String>>();
    let result = conn.query(query.as_str(), params_from_iter(strings)).await;
    if result.is_err() {
        return Err(());
    }

    if notecards.is_none() {
        return Ok(());
    }
    let notecards = notecards.unwrap();
    let notes = notecards
        .notecards
        .into_iter()
        .map(ReMapNotecard::remap)
        .collect::<Vec<ReMapNotecard>>();
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
    let result =
        crate::services::cfstorage::upload_notecard_to_cloudflare(secrets, compressed, &cfid).await;
    if result.is_err() {
        println!("Notecard Modify in Cloudflare Failed");
        //TODO: Handle failure
        return Err(());
    }
    Ok(())
}
