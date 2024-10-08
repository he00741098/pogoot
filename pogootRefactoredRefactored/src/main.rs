use serde::{Deserialize, Serialize};
mod services;
#[derive(Deserialize, Serialize, Default, Clone)]
struct AwsSecrets {
    turso_url: String,
    auth_token: String,
    zone_id: String,
    auth_key: String,
    auth_email: String,
    cloudflare_cert: String,
    cloudflare_key: String,
    r2accountid: String,
    r2accesskeyid: String,
    r2secretaccesskey: String,
    turnstileSecret: String,
}

#[tokio::main]
async fn main() {
    let aws_secrets = fetch_aws_secrets().await;
    if aws_secrets.is_err() {
        println!("Secrets read failed...");
        //TODO: Make fallback
    }
    let aws_secrets = aws_secrets.unwrap();
    if aws_secrets.is_none() {
        println!("No Secrets!");
    }
    let aws_secrets = aws_secrets.unwrap();

    println!("Secrets Read!");
    let aws_secrets = serde_json::from_str::<AwsSecrets>(&aws_secrets).unwrap();

    // let aws_secrets = AwsSecrets::default();
    server::start_serving(aws_secrets).await;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    auto_added: bool,
    source: String,
}

use aws_config::{self, BehaviorVersion, Region};
use services::server;

async fn fetch_aws_secrets() -> Result<Option<String>, aws_sdk_secretsmanager::Error> {
    let secret_name = "pogootSecrets";
    let region = Region::new("us-west-2");

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm.get_secret_value().secret_id(secret_name).send().await?;

    let secret_string = response.secret_string();
    match secret_string {
        Some(s) => Ok(Some(s.to_string())),
        _ => Ok(None),
    }
}

#[tokio::test]
async fn fetch_aws_secrets_test() {
    let secrets = fetch_aws_secrets().await;
    println!("Secrets: {:?}", secrets);
}
