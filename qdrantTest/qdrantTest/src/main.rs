use anyhow::anyhow;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::ScoredPoint;
use qdrant_client::qdrant::SearchPoints;
use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::SearchResponse;
#[derive(Deserialize)]
struct EmbeddingsResponse {
    pub outputs: Vec<Vec<f32>>,
}

pub async fn get_mighty_embedding(
    client: &Client,
    url: &str,
    text: &str
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
    embeddings.outputs.get(0).ok_or_else(|| anyhow!("mighty returned empty embedding")).cloned()
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let embedding = get_mighty_embedding(&Client::new(), "https://opulent-trout-747r46vp7pvcw646-5050.app.github.dev/sentence-transformers", "hello").await.unwrap();
    let client = QdrantClient::from_url("https://2f402eb8-e61c-4fb4-bf66-e2393decab9c.us-east4-0.gcp.cloud.qdrant.io")
    .with_api_key("DrhZCHYENQCqjNhJdSepbSkEoK2hWK2Job8oxgZH-cQuODgPmswWnA")
    .build()
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