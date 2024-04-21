use std::collections::HashMap;

use anyhow::anyhow;
use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::PointStruct;
use qdrant_client::qdrant::SearchPoints;
use qdrant_client::qdrant::SearchResponse;
use qdrant_client::qdrant::{vectors_config::Config, VectorParams, VectorsConfig};
use reqwest::Client;
use serde::Deserialize;
#[derive(Deserialize)]
struct EmbeddingsResponse {
    pub outputs: Vec<Vec<f32>>,
}

pub async fn qdrant_insert_embeddings(
    qdrant_client: &QdrantClient,
    text: Vec<(HashMap<String, Value>, Vec<f32>)>,
) {
    let mut payload = Vec::new();
    let mut id = 1;
    for ele in text {
        payload.push(PointStruct::new(
            id,
            ele.1,
            Payload::new_from_hashmap(ele.0),
        ));
        id += 1;
    }
    // let point = PointStruct {
    //     id: Some(PointId::from(42)), // unique u64 or String
    //     vectors: Some(vec![0.0_f32; 512].into()),
    //     payload: std::collections::HashMap::from([
    //         ("great".into(), Value::from(true)),
    //         ("level".into(), Value::from(9000)),
    //         ("text".into(), Value::from("Hi Qdrant!")),
    //         ("list".into(), Value::from(vec![1.234, 0.815])),
    //     ]),
    // };

    let response = qdrant_client
        .upsert_points(COLLECTION_NAME, None, payload, None)
        .await
        .unwrap();
}

pub async fn get_mighty_embedding(
    client: &Client,
    url: &str,
    text: &str,
) -> anyhow::Result<Vec<f32>> {
    let response = client.get(url).query(&[("text", text)]).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Mighty API returned status code {}",
            response.status()
        ));
    }

    let embeddings: EmbeddingsResponse = response.json().await?;
    // ignore multiple embeddings at the moment
    embeddings
        .outputs
        .get(0)
        .ok_or_else(|| anyhow!("mighty returned empty embedding"))
        .cloned()
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let text_bank = [
        ("spanish", "los dos"),
        ("dog", "collie"),
        ("dog", "chunky muybyebo"),
    ];
    let mut embeddings = vec![];
    for (x, y) in text_bank {
        let mut map = HashMap::new();
        map.insert(x.to_string(), Value::from(y));
        embeddings.push((
            map,
            get_mighty_embedding(
                &Client::new(),
                "http://localhost:5050/sentence-transformers",
                y,
            )
            .await
            .unwrap(),
        ))
    }

    let client = QdrantClient::from_url(
        "https://2f402eb8-e61c-4fb4-bf66-e2393decab9c.us-east4-0.gcp.cloud.qdrant.io:6334",
    )
    .with_api_key("DrhZCHYENQCqjNhJdSepbSkEoK2hWK2Job8oxgZH-cQuODgPmswWnA")
    .build()
    .unwrap();
    let collections_list = client.list_collections().await.unwrap();
    client.delete_collection(COLLECTION_NAME).await.unwrap();
    let result1 = client
        .create_collection(&CreateCollection {
            collection_name: COLLECTION_NAME.to_string(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 384,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    qdrant_insert_embeddings(&client, embeddings).await;
    let embedding = get_mighty_embedding(
        &Client::new(),
        "http://localhost:5050/sentence-transformers",
        "the two",
    )
    .await
    .unwrap();
    let search_response = qdrant_search_embeddings(&client, embedding).await.unwrap();
    println!("{:?}", search_response);
}

pub const SEARCH_LIMIT: u64 = 5;
const COLLECTION_NAME: &str = "mighty";

pub async fn qdrant_search_embeddings(
    qdrant_client: &QdrantClient,
    vector: Vec<f32>,
) -> anyhow::Result<SearchResponse> {
    qdrant_client
        .search_points(&SearchPoints {
            collection_name: COLLECTION_NAME.to_string(),
            vector,
            limit: SEARCH_LIMIT,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await
        .map_err(|err| anyhow!("Failed to search Qdrant: {}", err))
}
