

pub mod qdrant {
    use qdrant_client::prelude::*;
    use qdrant_client::qdrant::vectors_config::Config;
    use qdrant_client::qdrant::{CreateCollection, SearchPoints, VectorParams, VectorsConfig, CollectionOperationResponse};


    pub async fn create_collection_if_not_exists(client: &QdrantClient, collection_name: &str, configuration: Option<CreateCollection>) -> Result<CollectionOperationResponse, anyhow::Error> {
        let collection_exists = client.has_collection(collection_name).await?;
        println!("collection exists: {:?}", collection_exists);
        if !collection_exists {
            println!("creating collection: {:?}", collection_name);
            let res = match configuration {
                Some(config) => {
                    client.create_collection(&config).await
                }
                None => {
                    client.create_collection(&CreateCollection {
                        collection_name: collection_name.to_string(),
                        vectors_config: Some(VectorsConfig {
                            config: Some(Config::Params(VectorParams {
                                size: 768,
                                distance: 3, // Distance::Cosine.into(), <- this is not implemented as const
                            })),
                        }),
                        ..Default::default()
                        
                    }).await
                }
            };
            res
        }
        else {
            Ok(CollectionOperationResponse {
                result: true,
                time: 0.0,
            })
        }
    }
}
