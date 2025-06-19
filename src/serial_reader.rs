use crate::{config::Config, model::PhotodiodeData};
use anyhow::Result;
use mongodb::{bson::{self, doc, Document}, Collection};
use tokio::{io::{AsyncBufReadExt, BufReader}, time::{sleep, Duration}};
use tokio_serial::{SerialPortBuilderExt, DataBits, FlowControl, Parity, StopBits};

pub async fn read_serial_data(cfg: &Config, collection: Collection<Document>) -> Result<()> {
    for port in tokio_serial::available_ports()? {
        println!("🔌 Terdeteksi port: {:?}", port.port_name);
    }

    println!("Menunggu Arduino siap...");
    sleep(Duration::from_secs(2)).await;

    let port = tokio_serial::new(&cfg.serial_port, cfg.baud_rate)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .open_native_async()?;

    println!("Membaca data dari photodioda melalui serial port...");

    let reader = BufReader::new(port);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("📥 Diterima: {}", line);
        match serde_json::from_str::<PhotodiodeData>(&line) {
            Ok(data) => {
                let doc = doc! {
                    "sensor_id": &cfg.sensor_id,
                    "intensity": i32::from(data.intensity),
                    "timestamp": bson::DateTime::now(),
                };
                collection.insert_one(doc, None).await?;
                println!("✅ Disimpan ke MongoDB: {:?}", data);
            }
            Err(e) => {
                eprintln!("❌ Gagal parse JSON: {e}");
            }
        }
    }

    Ok(())
}
