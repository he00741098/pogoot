use argon2::password_hash::SaltString;
use argon2::{PasswordHash, Argon2, PasswordVerifier, PasswordHasher};
use async_std::sync::Mutex;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use std::sync::Arc;
use std::{fs::File, sync::atomic::AtomicI64};
use std::io::*;
use libsql_client::{Config, Client, Value, Statement, args, ResultSet};
use uuid::Uuid;
use std::result::Result as stdResult;
use crate::services::user_manage::User;
use crate::AwsSecrets;
//the database abstraction
///Database is purely intended to be an abstraction for the actual database
#[derive(Debug)]
pub struct Database{
    ///The client is the connection to the database
    ///It is not intended to be accessed directly
    ///Methods will be provided for every neccessary function
    client:Client,
}


#[derive(Debug)]
pub enum CoreDatatypeError {
    DatabaseDisconnect,
    UserDoesNotExist,
    DuplicateUsername,
    ValueNotText,
    PasswordHashIsError,
    IdNotFound,
    DuplicateEntries,
    SomethingWentWrong,
    DoesNotExist,
    ToStringFailed,
    NotInteger,
    ArrayLenIsZero,
    ParseFailed,
    UpdateFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicPermissionManager{
    owners:Vec<String>,
    ///Setting editors to None means that everyone can edit - Probably not a good idea
    editors:Option<Vec<String>>,
    ///Setting viewers to None means that everyone can view
    viewers:Option<Vec<String>>,
    blacklist:Vec<String>
    

}
impl BasicPermissionManager{
    ///The default new creates a permission set that will have a owner, editors, and blacklist.
    ///viewers is set to None so everyone can view.
    pub fn new()->Self{
        BasicPermissionManager { owners: vec![], editors: Some(vec![]), viewers: None, blacklist: vec![] }
    }
    ///Creates a new permission set with a owner. Has the same default settings as new()
    pub fn new_with_owner(username:&str)->Self{
        let mut basic = Self::new();
        basic.owners.push(username.to_owned());
        basic
    }
    ///checks if a user has permissions for this permission manager
    ///The ordering of priority starts with the explicitly named before going into general
    ///permissions
    ///Blacklist trumps general permissions but specific permissions trump blacklist
    pub fn has_permission(&mut self, username:&String)->bool{
        let viewers = std::mem::take(&mut self.viewers).unwrap();
       if viewers.contains(username){
            self.viewers=Some(viewers);
            return true;
        }else{
            self.viewers=Some(viewers);
        }
        let editors = std::mem::take(&mut self.editors).unwrap();
        if editors.contains(username){
            self.editors=Some(editors);
            return true
        }else{
            self.editors=Some(editors);
        }
        if self.owners.contains(username){
            return true;
        }
        if self.blacklist.contains(username){
            return false;
        }else if self.viewers.is_none(){
            return true;
        }else if self.editors.is_none(){
            return true;
        }
        false

    }
}

impl Database{
    //example sql interaction
    // let id_res = client.execute("SELECT ID FROM infoDb").await;
    //         if id_res.is_ok(){
    //         let id_res=id_res.unwrap();
    //         id_res.rows.into_iter().for_each(|x|{
    //                 if let Value::Text { value } = x.values[0].clone(){
    //                     ids.push(value);
    //                 }
    //             });
    //         }

    
    ///This method will take in a username and password. It will retrieve the user information from
    ///the database (no cache) and will compare the retrieved, hashed, password with the user
    ///inputted password. If anything goes wrong with the retrieval, the method will return an
    ///error. If the password was retrieved successfully, it will either return true or false.
    ///False will be if the password does not match the stored one and true will mean that the
    ///password is correct
    pub async fn verify_password(&self, username:String, password:String)->stdResult<(bool, String), CoreDatatypeError>{
        let statement = Statement::with_args(r"Select * from POGOOT where USERNAME=?;", args!(username));
        let retrieve_password_hash_result = self.client.execute(statement).await;
        if retrieve_password_hash_result.is_err(){
            println!("Database Disconnected");
            return Err(CoreDatatypeError::DatabaseDisconnect);
        }
        let password_hashes = retrieve_password_hash_result.unwrap();
        if password_hashes.rows.len()==1{
            let saved_password = password_hashes.rows[0].values[1].clone();
            let grab_value = match saved_password {
                libsql_client::Value::Text { value } => {
                    Some(value)
                }
                _ => None,
            };
            if grab_value.is_none(){return Err(CoreDatatypeError::ValueNotText)}
            let grab_value = grab_value.unwrap();
            let password_hash = PasswordHash::new(&grab_value);
            if password_hash.is_err(){
                return Err(CoreDatatypeError::PasswordHashIsError)
            }
            let password_hash =  password_hash.unwrap();
            if Argon2::default().verify_password(password.as_bytes(), &password_hash).is_ok(){
                let raw_json = password_hashes.rows[0].values[3].clone();
                if let Value::Text { value } = raw_json{

                
                // let parse_raw_json = serde_json::from_str::<User>(&raw_json);
                return Ok((true, value));
                }else{
                    return Ok((true, "".to_string()));
                }
            }
            //The password was verified but was incorrect
            return Ok((false, "".to_string()))
        }else if password_hashes.rows.len()<1{
            Err(CoreDatatypeError::UserDoesNotExist)
        }else if password_hashes.rows.len()>1{
            Err(CoreDatatypeError::DuplicateUsername)
        }else{
            Err(CoreDatatypeError::SomethingWentWrong)
        }

    }

    ///Fetches the raw json of the notecard based on the notecard id
    pub async fn fetch_note_card(&self, notecard_id:String)->stdResult<String, CoreDatatypeError>{
        let stmt = Statement::with_args(r"Select * from NOTECARDS where ID=?;", args!(notecard_id));
        let fetched = self.client.execute(stmt).await;
        if fetched.is_err(){
            return Err(CoreDatatypeError::IdNotFound);
        }
        let fetched = fetched.unwrap();
        if fetched.rows.len()==1{
            return match fetched.rows[0].values[2].clone(){
                Value::Text { value } =>{Ok(value)},
                _=>{Err(CoreDatatypeError::ValueNotText)}
            }
        }else if fetched.rows.len()==0{
            return Err(CoreDatatypeError::DoesNotExist)
        }else{
            return Err(CoreDatatypeError::DuplicateEntries)
        }

    }
    

    
    ///Generates a supposedly totally random notecard_id. NOTE THAT THIS IS NOT CHECKED WITH THE
    ///DATABASE AND COLLISION COULD POSSIBLY OCCUR ALTHOUGH PROBABLY NOT
    pub fn generate_random_id()->String{
        serde_json::to_string(&Uuid::now_v7()).unwrap_or(nanoid::nanoid!(16))
    }

    pub async fn store_note_card(&self, notecard_rawjson:String, username:String, name:String)->stdResult<String,CoreDatatypeError>{
            // USERNAME text,
            // ID text,
        //      Name text
            // PERMISSIONS_JSON text,
            // RAWJSON text, VERSION INT
        let notecard_id = Self::generate_random_id();
        let parsed_result = serde_json::to_string(&BasicPermissionManager::new_with_owner(&username));
        if parsed_result.is_err(){
            return Err(CoreDatatypeError::ToStringFailed);
        }
        let stmt = Statement::with_args(r"INSERT INTO NOTECARDS VALUES (?, ?, ?, ?, ?, ?);", args!(&username, &notecard_id, name, parsed_result.unwrap(), notecard_rawjson, 1));
        let fetched = self.client.execute(stmt).await;
        if fetched.is_err(){
            return Err(CoreDatatypeError::DatabaseDisconnect);
        }
        Ok(notecard_id)
    }
    ///id, username, token
    ///Grabs the permissions of a specific notecard set
    pub async fn get_notecard_permissions(&self, id:&str)->stdResult<BasicPermissionManager, CoreDatatypeError>{
        let stmt = Statement::with_args(r"SELECT * FROM NOTECARDS VALUES WHERE ID=?;", args!(id));
        let fetched = self.client.execute(stmt).await;
        if fetched.is_err(){
            return Err(CoreDatatypeError::DatabaseDisconnect);
        }
        let fetched = fetched.unwrap();
        println!("Notecard fetched permission JSON: {:?}", fetched);
        if fetched.rows.len()==1{
            return match fetched.rows[0].values[1].clone(){
                Value::Text { value } =>{
                    if let Ok(value) = serde_json::from_str(&value){
                        Ok(value)
                    }else{
                        Err(CoreDatatypeError::ParseFailed)
                    }
                },
                _=>{Err(CoreDatatypeError::ValueNotText)}
            }
        }else if fetched.rows.len()==0{
            return Err(CoreDatatypeError::DoesNotExist)
        }else{
            return Err(CoreDatatypeError::DuplicateEntries)
        }
    }
    pub async fn edit_note_card_permissions(){
        todo!()
    }
    pub async fn edit_note_card_contents(){
        todo!()
    }

    // USERNAME text,
    // PASSWORD text,
    // RECENTIP text,
    // RAWJSON text,
    // VERSION INT
    /// Purely registers the player. This does not log them in or give them a token
    pub async fn register_user(&self, username:String, password:String, ip:String)->stdResult<User,CoreDatatypeError>{
        // test if the username is available

        let stmt1 = Statement::with_args(r"SELECT * FROM POGOOT WHERE USERNAME=?;", args!(&username));
        let stmt1_result = self.client.execute(stmt1).await;
        if stmt1_result.is_err(){
            return Err(CoreDatatypeError::SomethingWentWrong)
        }
        let stmt1_final = stmt1_result.unwrap();
        if stmt1_final.rows.len()>0{
            return Err(CoreDatatypeError::DuplicateUsername)
        }
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt);
        if password_hash.is_err(){
            return Err(CoreDatatypeError::PasswordHashIsError)
        }
        let password_hash = password_hash.unwrap();
        let user = User::new(username.clone(), password_hash.clone().to_string(), ip.clone());
        let stringy = serde_json::to_string(&user);
        if stringy.is_err(){
            return Err(CoreDatatypeError::ToStringFailed)
        }

        let stmt = Statement::with_args(r"INSERT INTO POGOOT VALUES (?, ?, ?, ?, ?);", args!(&username, &password_hash.to_string(), &ip, stringy.unwrap(), 1));
        let fetched = self.client.execute(stmt).await;
        if fetched.is_err(){
            return Err(CoreDatatypeError::DatabaseDisconnect);
        }
        Ok(user)
    }
    pub async fn delete_user(&self, username:String)->stdResult<(), CoreDatatypeError>{
        
        let stmt = Statement::with_args(r"Select * FROM POGOOT WHERE USERNAME=?;", args!(&username));
        let fetched = self.client.execute(stmt).await;
        if fetched.is_err(){
            Err(CoreDatatypeError::DatabaseDisconnect)
        }else{
            let fetched = fetched.unwrap();
            if fetched.rows.len()>0&&fetched.rows[0].values.len()>0{
               let parsing = fetched.rows[0].values[0].clone();
                match parsing{
                    Value::Text { value }=>{
                        let parsed_result = serde_json::from_str::<User>(&value);
                        if parsed_result.is_err(){return Err(CoreDatatypeError::ParseFailed)}
                        let mut parsed_result = parsed_result.unwrap();
                        parsed_result.deleted = true;
                        let to_string = serde_json::to_string(&parsed_result);
                        if to_string.is_err(){
                            return Err(CoreDatatypeError::ToStringFailed)
                        }
                        let to_string = to_string.unwrap();
                        let stmt = Statement::with_args(r"UPDATE POGOOT SET RAWJSON = ? WHERE USERNAME = ?;", args!(to_string, &username));
                        let fetched = self.client.execute(stmt).await;
                        if fetched.is_err(){return Err(CoreDatatypeError::UpdateFailed)}
                        Ok(())
                    },
                        _=>{
                        return Err(CoreDatatypeError::ValueNotText)
                    }
                }
            }else{
                Err(CoreDatatypeError::ArrayLenIsZero)
            }

        }
    }
    ///Updates the users raw json. Do not use to update usernames, passwords, etc
    pub async fn update_user_json(&self, user:Arc<Mutex<User>>)->stdResult<(), CoreDatatypeError>{
        let user = user.lock().await;
        let stringy = to_string(&*user);
        if stringy.is_err(){
            return Err(CoreDatatypeError::ToStringFailed);
        }
        let stmt = Statement::with_args(r"UPDATE POGOOT SET RAWJSON=? WHERE USERNAME=?;", args!(&user.username));
        let execute_result = self.client.execute(stmt).await;
        if execute_result.is_err(){
            return Err(CoreDatatypeError::UpdateFailed);
        }else{
            return Ok(())
        }
    }
    pub async fn change_password(){
        todo!()
    }
    ///CHANGING USERNAME IS NOT ALLOWED> USERNAME WILL BE USED FOR PERMISSIONS STORAGE AS WELL AS
    ///OTHER STUFF
    pub async fn change_username(){
        todo!()
    }
    pub async fn update_recent_ip(){
        todo!()
    }

    ///Tries to get secrets
    ///Note that DBSecrets.toml MUST exist or it will panic
    ///This is only intended to be called once at startup
    pub fn try_to_get_secrets(secrets:AwsSecrets)->DBSecrets{
        DBSecrets{
            turso_url: secrets.turso_url,
            auth_token: secrets.auth_token,
        }
    }
    pub async fn new(credentials:DBSecrets)->Option<Self>{
        let url = credentials.turso_url.as_str().try_into();
        if url.is_err(){return None;}
        let url = url.unwrap();
        let config = Config{
            url,
            auth_token: Some(credentials.auth_token),
        };
        let client = if let Ok(c) = Client::from_config(config).await{
            c
        }else{
            return None
        };
        //tracks the users username, password, most recently used ip, and stores more data as
        //rawJSON
        //TODO: Figure out the optimal database setup
        let create_table_result = client.execute("CREATE TABLE IF NOT EXISTS POGOOT(
            USERNAME text,
            PASSWORD text,
            RECENTIP text,
            RAWJSON text,
            VERSION INT
        );").await;
        if create_table_result.is_err(){
            return None;
        }
        let create_table_result = client.execute("CREATE TABLE IF NOT EXISTS NOTECARDS(
            USERNAME text,
            ID text,
            NAME text,
            PERMISSIONS_JSON text,
            RAWJSON text,
            VERSION INT
        );").await;
        if create_table_result.is_err(){
            return None;
        }
        Some(Database{
            client,
        })
    }
    
}
// #[test]
// fn init_database_test(){
//     let db = Database::new(Database::try_to_get_secrets());

// }

#[derive(Serialize, Deserialize, Clone)]
pub struct DBSecrets{
    turso_url:String,
    auth_token:String,
}
