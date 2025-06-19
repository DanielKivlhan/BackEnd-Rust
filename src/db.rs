use crate::config::Config;
use anyhow::Result;
use mongodb::{Client, Collection, bson::Document};

pub async fn connect_to_mongodb(cfg: &Config) -> Result<Collection<Document>> {
    let client = Client::with_uri_str(&cfg.mongo_uri).await?;
    let collection = client
        .database(&cfg.db_name)
        .collection::<Document>(&cfg.collection_name);
    Ok(collection)
}
