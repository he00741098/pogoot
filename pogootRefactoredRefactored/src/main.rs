use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod services;
#[derive(Deserialize, Serialize, Default)]
struct AwsSecrets {
    turso_url: String,
    auth_token: String,
    zone_id: String,
    auth_key: String,
    auth_email: String,
    cloudflare_cert: String,
    cloudflare_key: String,
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "pogoot=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // let aws_secrets = fetch_aws_secrets().await;
    // if aws_secrets.is_err(){
    //     println!("Secrets read failed...");
    //     //TODO: Make fallback
    // }
    // let aws_secrets = aws_secrets.unwrap();
    // if aws_secrets.is_none(){
    //     println!("No Secrets!");
    // }
    // let aws_secrets = aws_secrets.unwrap();
    //
    // println!("Secrets:\n {:?}", aws_secrets);
    // let aws_secrets = serde_json::from_str::<AwsSecrets>(&aws_secrets).unwrap();
    // let AwsSecrets { turso_url, auth_token, zone_id, auth_key, auth_email, cloudflare_cert, cloudflare_key }
    let aws_secrets = AwsSecrets::default();
    server::start_serving(aws_secrets).await;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    auto_added: bool,
    source: String,
}

use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager;
use services::server;

async fn fetch_aws_secrets() -> Result<Option<String>, aws_sdk_secretsmanager::Error> {
    let secret_name = "pogootSecrets";
    let region = Region::new("us-west-2");

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm.get_secret_value().secret_id(secret_name).send().await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    let secret_string = response.secret_string();
    match secret_string {
        Some(s) => Ok(Some(s.to_string())),
        _ => Ok(None),
    }

    // Your code goes here
}

#[tokio::test]
async fn fetch_aws_secrets_test() {
    let secrets = fetch_aws_secrets().await;
    println!("Secrets: {:?}", secrets);
}
