mod config;
mod db;
mod serial_reader;
mod model;

use anyhow::Result;
use config::load_config;
use db::connect_to_mongodb;
use serial_reader::read_serial_data;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = load_config()?;
    let collection = connect_to_mongodb(&cfg).await?;
    read_serial_data(&cfg, collection).await?;
    Ok(())
}
