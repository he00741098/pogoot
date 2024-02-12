use std::{fs::File, io::Read, collections::HashMap};
use std::process::Command;

use serde::{Deserialize, Serialize};

mod services;

#[tokio::main]
async fn main() {
    //change cloudflare stuff ----
    //
    let ip = if let Some(ip) = public_ip::addr_v6().await {
        println!("ipv6 address: {:?}", ip);
        ip.to_string()
    } else {
        println!("Couldn't get an IP address");
        panic!("Can't get ip");
    };

    //run script to setup certbot

    // Command::new("sudo certbot certonly")
    //     .arg("--non-interactive")
    //     .arg("--agree-tos")
    //     .arg("--no-eff-email")
    //     .arg("--no-redirect")
    //     .arg("--email 'admin@sweep.rs'")
    //     .arg("--certname pogootCert")
    //     .arg("--domains '*.sweep.rs'")
    //     .spawn()
    //     .expect("Certbot failed to start");

    // Command::new("sudo certbot install")
    //     .arg("--nginx")
    //     .arg("--no-redirect")
    //     .arg("--certname pogootCert")
    //     .arg(format!("--domains '{}.sweep.rs'", ip.clone().replace(".", "")))
    //     .spawn()
    //     .expect("Certbot failed to start");


    let mut contents = String::new();
    let mut file = File::open("CloudflareSecrets.toml");
    if file.is_err(){
        // return DBSecrets{
        //     turso_url:std::env!("turso").to_string(),
        //     auth_token:std::env!("auth").to_string(),

        // }
        panic!("File read failed");
        // let std::env!()
    }
    let mut file = file.unwrap();
    let cf_secrets = if file.read_to_string(&mut contents).is_ok(){
        let cf_secrets:CFSecrets = toml::from_str(&contents).unwrap(); 
        cf_secrets
    }else{
        panic!("No Secrets!");
    };

    let mut map = HashMap::new();
    map.insert("content", ip.clone());
    map.insert("name", format!("{}.sweep.rs", ip.replace(':',"").replace('.',"").to_string()));
    map.insert("type", "AAAA".to_string());
    map.insert("comment", "auto_dns_update".to_string());
    map.insert("ttl", "1".to_string());

    let client = reqwest::Client::new();
    let res = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", cf_secrets.zone_id))
        .header("Content-Type","application/json")
        .header("X-Auth-Email", cf_secrets.auth_email)
        .header("X-Auth-key", cf_secrets.auth_key)
        .json(&map)
        .send()
    .await;
    if res.is_err(){
        println!("Res Is Err: {:?}", res);
        panic!("Ip not set up");
    }else{
        println!("Res Is Ok: {:?}", res.unwrap().json::<CfResponse>().await);
    }
    
    // if res.is
    services::corporate::Coordinator::start_all_services().await;
}


#[derive(Serialize, Deserialize, Clone)]
pub struct CFSecrets{
    zone_id:String,
    auth_key:String,
    auth_email:String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CfResponse{
    result {
        content:String,
        name:String,
        proxied:bool,
        r#type:String,
        comment:String,
        created_on:String,
        id:String,
        locked:bool,
        meta:meta,
    modified_on:String,
    proxiable:bool,
    tags:Vec<String>,
    ttl:i32,
    zone_id:String,
    zone_name:String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct meta{
        auto_added:bool,
        source:String,
}



use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager;

async fn fetch_aws_secrets() -> Result<Option<String>, aws_sdk_secretsmanager::Error> {
    let secret_name = "pogootSecrets";
    let region = Region::new("us-west-2");

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    let secret_string = response.secret_string();
    match secret_string{
        Some(s)=>{
            Ok(Some(s.to_string()))
        },
        _=>{
            Ok(None)
        }
    }
    
    // Your code goes here
}

#[tokio::test]
async fn fetch_aws_secrets_test(){
    let secrets = fetch_aws_secrets().await;
    println!("Secrets: {:?}", secrets);
}
