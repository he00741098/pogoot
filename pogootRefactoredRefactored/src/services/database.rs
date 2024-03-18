//A database system that needs to accomplish a few key tasks
//1. Store notecards in a Database
//2. Retreive notecards from a Database
//3. Store user data
//4. Retreive user data

use crate::AwsSecrets;
use crate::services::server::NotecardDBRequest;
use libsql_client::{Config, Client, Value, Statement, args, ResultSet};
use prost::Message;


//NOTECARD STUFF----------------------------------------
pub struct NotecardStorageControllerService{
    access_credentials:AwsSecrets,
    client:Client
}

impl NotecardStorageControllerService{
    async fn start_service(secrets:AwsSecrets)->Option<tokio::sync::mpsc::Sender<NotecardDBRequest>>{
        let client_result = Self::turso_init(&secrets).await;
        if client_result.is_err(){
            return None;
        }
        let client = client_result.unwrap();

        let dbobj = NotecardStorageControllerService{
            access_credentials:secrets,
            client
        };

        let (tx, rx) = tokio::sync::mpsc::channel(216);
        
        Some(tx)

    }

    async fn turso_init(secrets:&AwsSecrets)->Result<Client, ()>{
        let url = secrets.turso_url.as_str().try_into();
        if url.is_err(){return Err(());}
        let url = url.unwrap();
        let config = Config{
            url,
            auth_token: Some(secrets.auth_token.clone()),
        };
        let client = if let Ok(c) = Client::from_config(config).await{
            c
        }else{
            return Err(())
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
            return Err(());
        }
        let create_table_result = client.execute("CREATE TABLE IF NOT EXISTS NOTECARDS(
            USERNAME text,
            ID text,
            NAME text,
            PERMISSIONS_JSON text,
            VERSION INT
        );").await;
        if create_table_result.is_err(){
            return Err(());
        }
        Ok(client)
    }
    
    async fn begin_serving(self, mut rx:tokio::sync::mpsc::Receiver<NotecardDBRequest>){
        // let mut buffer = vec![];
        while let Some(db_req) = rx.recv().await{
            match db_req{
                NotecardDBRequest::Store(notecards, callback) => todo!(),
                NotecardDBRequest::Fetch(id, callback) => todo!(),
            }
        }

        
    }





}


mod CFStorage{

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::{Credentials, ProvideCredentials}, operation::{
    copy_object::{CopyObjectError, CopyObjectOutput},
    create_bucket::{CreateBucketError, CreateBucketOutput},
    get_object::{GetObjectError, GetObjectOutput},
    list_objects_v2::ListObjectsV2Output,
    put_object::{PutObjectError, PutObjectOutput},
}};
use aws_sdk_s3::types::{
    BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier,
};
use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};
use std::{fs::{read_to_string, File}, io::{BufRead, Write}, path::Path};
use std::str;


    async fn fetch_aws_secrets() -> Result<Option<String>, aws_sdk_s3::Error> {
        let region = Region::new("us-west-2");

        let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region(region)
            .endpoint_url("https://60140a99f7c7c4232fe54ee74112198b.r2.cloudflarestorage.com/pogootdata")
            .load()
        .await;

        let asm = aws_sdk_secretsmanager::Client::new(&config);

        // let response = asm
        // .await?;
        // For a list of exceptions thrown, see
        // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

        // let secret_string = response.secret_string();
        // match secret_string{
        //     Some(s)=>{
        //         Ok(Some(s.to_string()))
        //     },
        //     _=>{
        //         Ok(None)
        //     }
        // }

        // Your code goes here
        unimplemented!()
    }

    async fn get_object(client: Client, key: &str) -> Result<Vec<u8>, ()> {
        // trace!("bucket:      {}", opt.bucket);
        // trace!("object:      {}", opt.object);
        // trace!("destination: {}", opt.destination.display());


        // let destination = format!("./temp/{}", input);
        let object = client
            .get_object()
            .bucket("pogootdata")
            .key(key)
            .send()
        .await;
        if object.is_err(){
            return Err(())
        }
        let mut object = object.unwrap();
        // let body = object.body;
        

        // let file = File::create(destination);
        // if file.is_err(){
        //     println!("couldn't create file");
        //     return Err(())
        // }
        // let mut file = file.unwrap();
        let mut buffer = vec![];
        let mut byte_count = 0_usize;
        while let Ok(Some(bytes)) = object.body.try_next().await {
            let bytes_len = bytes.len();
            let err = buffer.write_all(&bytes);
            if err.is_err(){
                println!("Buffer write erred");
                return Err(())
            }
            byte_count += bytes_len;
        }

        // println!("{:?}", String::from_utf8(buffer));
        Ok(buffer)
        // unimplemented!()
    }
    pub async fn upload_object(
        client: &Client,
        bucket_name: &str,
        text:&str,
        key: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from(text.as_bytes().to_vec());
        client
            .put_object()
            .bucket(bucket_name)
            .key(key)
            .body(body)
            .send()
        .await
    }
    #[tokio::test]
    async fn get_object_test(){

        let region = Region::new("auto");
        let credentials_provider = Credentials::new("e41ea0be751835c45af3bc71b15bb336", "5f453dce6234a0443733aa79cb5f21b576654d4259a5ca71da0534e9140089ba", None, None, "cloudflare");
        let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region(region)
            .endpoint_url("https://60140a99f7c7c4232fe54ee74112198b.r2.cloudflarestorage.com/pogootdata")
            .credentials_provider(credentials_provider)
            .load()
        .await;
        let client = aws_sdk_s3::Client::new(&config);
        let object_store_result = upload_object(&client, "pogootdata", "weeeweee", "test").await.expect("Upload failed");
        let object_result = get_object(client, "test").await;

        println!("{:?}", String::from_utf8(object_result.unwrap()));
    }


}
