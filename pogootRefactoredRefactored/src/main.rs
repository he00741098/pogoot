use std::{fs::File, io::Read, collections::HashMap};
use std::process::Command;

use serde::{Deserialize, Serialize};

mod services;

#[derive(Deserialize, Serialize)]
struct AwsSecrets{
    turso_url:String,
    auth_token:String,
    zone_id:String,
    auth_key:String,
    auth_email:String,
    cloudflare_cert:String,
    cloudflare_key:String
}

#[tokio::main]
async fn main() {
    //change cloudflare stuff ----
    //
    let aws_secrets = fetch_aws_secrets().await;
    if aws_secrets.is_err(){
        println!("Secrets read failed...");
        //TODO: Make fallback
    }
    let aws_secrets = aws_secrets.unwrap();
    if aws_secrets.is_none(){
        println!("No Secrets!");
    }
    let aws_secrets = aws_secrets.unwrap();

    println!("Secrets:\n {:?}", aws_secrets);
    let mut aws_secrets = serde_json::from_str::<AwsSecrets>(&aws_secrets).unwrap();

    //Currently no longer seting cloudflare because we are going to just have 3 servers
    //     let ip = if let Some(ip) = public_ip::addr_v6().await {
    //         println!("ipv6 address: {:?}", ip);
    //         ip.to_string()
    //     } else {
    //         println!("Couldn't get an IP address");
    //         panic!("Can't get ip");
    //     };

    // // pub struct CFSecrets{
    // //     zone_id:String,
    // //     auth_key:String,
    // //     auth_email:String
    // // }

    //     let cf_secrets = CFSecrets { zone_id:std::mem::take(&mut aws_secrets.zone_id), auth_key:std::mem::take(&mut aws_secrets.auth_key), auth_email:std::mem::take(&mut aws_secrets.auth_email) };
    //     let mut map = HashMap::new();
    //     map.insert("content", ip.clone());
    //     map.insert("name", format!("{}.sweep.rs", ip.replace(':',"").replace('.',"").to_string()));
    //     map.insert("proxied", "true".to_string());
    //     map.insert("type", "AAAA".to_string());
    //     map.insert("comment", "auto_dns_update".to_string());
    //     map.insert("ttl", "1".to_string());

    //     let client = reqwest::Client::new();
    //     let res = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", cf_secrets.zone_id))
    //         .header("Content-Type","application/json")
    //         .header("X-Auth-Email", cf_secrets.auth_email)
    //         .header("X-Auth-key", cf_secrets.auth_key)
    //         // .header("proxied", "true".to_string())
    //         .json(&map)
    //         .send()
    //     .await;
    //     if res.is_err(){
    //         println!("Res Is Err: {:?}", res);
    //         panic!("Ip not set up");
    //     }
    //     let res = res.unwrap();
    //     println!("Res Is Ok: {:?}", res);

    
    // if res.is
    services::corporate::Coordinator::start_all_services(aws_secrets).await;
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

#[tokio::test]
async fn test_cloudflare_test(){
    let aws_secrets = fetch_aws_secrets().await;
    if aws_secrets.is_err(){
        println!("Secrets read failed...");
        //TODO: Make fallback
    }
    let aws_secrets = aws_secrets.unwrap();
    if aws_secrets.is_none(){
        println!("No Secrets!");
    }
    let aws_secrets = aws_secrets.unwrap();

    println!("Secrets:\n {:?}", aws_secrets);
    let mut aws_secrets = serde_json::from_str::<AwsSecrets>(&aws_secrets).unwrap();

    let ip = "2600:1f14:1638:b401:2526:7fe1:1be6:c5ee".to_string();

    let cf_secrets = CFSecrets { zone_id:std::mem::take(&mut aws_secrets.zone_id), auth_key:std::mem::take(&mut aws_secrets.auth_key), auth_email:std::mem::take(&mut aws_secrets.auth_email) };
    let mut map = HashMap::new();
    map.insert("content", ip.clone());
    map.insert("name", format!("{}.sweep.rs", ip.replace(':',"").replace('.',"").to_string()));
    // map.insert("proxied", true.to_string());
    map.insert("type", "AAAA".to_string());
    map.insert("comment", "auto_dns_update".to_string());
    map.insert("ttl", "1".to_string());
    // let mut map1 = HashMap::new();
    // map1.insert("proxied", true);
    // let first = r#"{\n  \"content\": \""#;
    // let second = r#"\",\n  \"name\": \""#;
    // let raw_body = format!("{}{}{}{}{}",first, ip.clone(), second, format!("{}.sweep.rs", ip.replace(':',"").replace('.',"").to_string()), r#"\",\n  \"proxied\": true,\n  \"type\": \"AAAA\",\n  \"comment\": \"auto_dns_update\",\n  \"tags\": [\n    \"owner:dns-team\"\n  ],\n  \"ttl\": 1\n}"#);
    // println!("\n\n\n{}", raw_body);
    let client = reqwest::Client::new();
    let res = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", cf_secrets.zone_id))
        .header("Content-Type","application/json")
        .header("X-Auth-Email", cf_secrets.auth_email)
        .header("X-Auth-key", cf_secrets.auth_key)
        // .header("proxied", "true".to_string())
        .json(&map)
        // .body(raw_body)
        // .json(&map1)
        .send()
    .await;
    if res.is_err(){
        println!("Res Is Err: {:?}", res);
        panic!("Ip not set up");
    }
    let res = res.unwrap();
    println!("Res Is Ok: {:?}", res);
}
