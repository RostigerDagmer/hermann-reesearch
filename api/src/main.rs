use futures::StreamExt;
use ndarray::Array1;
mod server;
mod handlers;
use handlers::{embedding, qdrant};
use tonic::transport::Server;

mod model;
mod rank;
use server::surface;
use handlers::insert;

// use qdrant_client::prelude::*;
// //use model::paper::Paper;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::Path;
// use triton_client::Client;
// use tonic::{transport::Server, Request, Response, Status};
// use std::collections::HashMap;
// use serde_json;

async fn init() -> anyhow::Result<(reqwest::Client, triton_client::Client, qdrant_client::client::QdrantClient)> {
    //let tokenizer = tokenizers::Tokenizer::from_pretrained("allenai/specter2", None).unwrap();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());

    let fatcat_client = reqwest::ClientBuilder::default().default_headers(headers).build().unwrap();
    let triton_client = triton_client::Client::new("http://172.24.165.10:9001/", None).await?;//"http://192.168.178.24:8001/", None).await?;

    let qdrant_config = qdrant_client::client::QdrantClientConfig::from_url("http://qdrant:6334");
    let qdrant_client = qdrant_client::client::QdrantClient::new(Some(qdrant_config)).await?;
    let collection_exists = insert::qdrant::create_collection_if_not_exists(&qdrant_client, "papers", None).await?;
    println!("paper collection exists: {}", collection_exists.result);

    Ok((fatcat_client, triton_client, qdrant_client))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let (fatcat_client, triton_client, qdrant_client) = init().await.unwrap();
    let tokenizer = tokio::task::spawn_blocking(move || { tokenizers::Tokenizer::from_pretrained("allenai/specter2", None).unwrap() }).await.unwrap();

    println!("Server listening on http://0.0.0.0:8000 -> 8005");
    Server::builder().add_service(surface::paper_search_server::PaperSearchServer::new(server::PaperSearchEngine {
        inference_client: triton_client,
        tokenizer,
        fatcat_client,
        qdrant_client,
    })).serve("0.0.0.0:8000".parse().unwrap()).await?;

    Ok(())
}