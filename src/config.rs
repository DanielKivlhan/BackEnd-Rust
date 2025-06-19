use std::env;
use anyhow::Result;

pub struct Config {
    pub mongo_uri: String,
    pub db_name: String,
    pub collection_name: String,
    pub serial_port: String,
    pub sensor_id: String,
    pub baud_rate: u32,
}

pub fn load_config() -> Result<Config> {
    dotenvy::dotenv().ok();

    Ok(Config {
        mongo_uri: env::var("MONGO_URI")?,
        db_name: env::var("DB_NAME")?,
        collection_name: env::var("COLLECTION_NAME")?,
        serial_port: env::var("SERIAL_PORT")?,
        sensor_id: env::var("SENSOR_ID").unwrap_or("UNKNOWN".into()),
        baud_rate: env::var("BAUD_RATE")?.parse::<u32>()?,
    })
}
