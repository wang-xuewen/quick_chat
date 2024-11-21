mod qc_client;

use crate::qc_client::QcClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let result = QcClient::open("127.0.0.1:8080").await;
    if let Err(e) = result {
        eprintln!("qc client open Error: {}", e);
    }

    Ok(())
}
