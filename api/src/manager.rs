type PooledConnection = r2d2::PooledConnection<r2d2::ConnectionManager<qdrant_client::SqliteConnection>>;


pub struct ContextConfig {
    pub client: QdrantClientConfig,
    pub tokenizer: String,
}


pub struct Context {
    pub client: QdrantClient,
    pub tokenizer: Tokenizer,
}

impl Context {
    pub fn new(config: ContextConfig) -> Self {
        let client = QdrantClient::new(Some(config.client)).await?;
        let tokenizer = Tokenizer::from_pretrained(config.tokenizer, None).unwrap();

        Self {
            client,
            tokenizer,
        }
    }

    async fn list_collections(&self) -> Result<Vec<String>, Error> {
        let collections = self.client.list_collections().await?;
        Ok(collections)
    }

    async fn create_collection(&self, collection_name: &str) -> Result<(), Error> {
        let collection = Collection::new(collection_name);
        self.client.create_collection(collection).await?;
        Ok(())
    }

}