pub mod cfstorage {

    use aws_config::{BehaviorVersion, Region};
    use aws_sdk_s3::types::{
        BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier,
    };
    use aws_sdk_s3::{
        config::{Credentials, ProvideCredentials},
        operation::{
            copy_object::{CopyObjectError, CopyObjectOutput},
            create_bucket::{CreateBucketError, CreateBucketOutput},
            get_object::{GetObjectError, GetObjectOutput},
            list_objects_v2::ListObjectsV2Output,
            put_object::{PutObjectError, PutObjectOutput},
        },
    };
    use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};
    use std::str;
    use std::{
        fs::{read_to_string, File},
        io::{BufRead, Write},
        path::Path,
    };

    use crate::services::notecard::ReMapNotecard;
    use crate::services::server::pogoots::NotecardList;
    use crate::AwsSecrets;

    async fn get_object(client: Client, key: &str) -> Result<Vec<u8>, ()> {
        let object = client
            .get_object()
            .bucket("pogootdata")
            .key(key)
            .send()
            .await;
        if object.is_err() {
            return Err(());
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
        // let mut byte_count = 0_usize;
        while let Ok(Some(bytes)) = object.body.try_next().await {
            // let bytes_len = bytes.len();
            let err = buffer.write_all(&bytes);
            if err.is_err() {
                println!("Buffer write erred");
                return Err(());
            }
            // byte_count += bytes_len;
        }

        // println!("{:?}", String::from_utf8(buffer));
        Ok(buffer)
        // unimplemented!()
    }
    async fn upload_object(
        client: &Client,
        body: Vec<u8>,
        key: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        let body = ByteStream::from(body);
        client
            .put_object()
            .bucket("pogootdata")
            .key(key)
            .body(body)
            .send()
            .await
    }

    //TODO: potentially figure out how to do bulk uploads
    pub async fn upload_notecard_to_cloudflare(
        secrets: &mut AwsSecrets,
        notes: Vec<u8>,
        id: &str,
    ) -> Result<(), ()> {
        let region = Region::new("auto");
        let credentials_provider = Credentials::new(
            std::mem::take(&mut secrets.r2accesskeyid),
            std::mem::take(&mut secrets.r2secretaccesskey),
            None,
            None,
            "cloudflare",
        );
        let endpointurl = format!(
            "https://{}.r2.cloudflarestorage.com/pogootdata",
            secrets.r2accountid
        );
        let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region(region)
            .endpoint_url(&endpointurl)
            .credentials_provider(credentials_provider)
            .load()
            .await;
        let client = aws_sdk_s3::Client::new(&config);
        let object_store_result = upload_object(&client, notes, id).await;
        if object_store_result.is_err() {
            Err(())
        } else {
            Ok(())
        }
    }

    #[tokio::test]
    async fn get_object_test() {
        let region = Region::new("auto");
        let credentials_provider = Credentials::new("", "", None, None, "cloudflare");
        let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region(region)
            .endpoint_url("")
            .credentials_provider(credentials_provider)
            .load()
            .await;
        let client = aws_sdk_s3::Client::new(&config);
        let object_store_result = upload_object(&client, "weeeweee".as_bytes().to_vec(), "test")
            .await
            .expect("Upload failed");
        let object_result = get_object(client, "test").await;

        println!("{:?}", String::from_utf8(object_result.unwrap()));
    }
}
