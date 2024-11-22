mod qc_client;

use crate::qc_client::QcClient;
use anyhow::Result;
use log::{error, info};
use log4rs;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    let result = QcClient::open("127.0.0.1:8080").await;
    if let Err(e) = result {
        error!("qc client open Error: {}", e);
    }

    Ok(())
}
