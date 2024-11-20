mod qc_server;

use crate::qc_server::QcServer;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = QcServer::new("127.0.0.1:8080").await?;
    server.start().await?;

    Ok(())
}
