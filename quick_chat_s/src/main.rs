mod qc_server;

use crate::qc_server::QcServer;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let result = QcServer::new("127.0.0.1:8080").await;
    match result {
        Ok(server) => {
            if let Err(e) = server.start().await {
                eprintln!("qc server start Error: {}", e);
            };
        }
        Err(e) => {
            eprintln!("qc server new Error: {}", e);
        }
    }

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     let server = QcServer::new("127.0.0.1:8080").await?;
//     server.start().await?;

//     Ok(())
// }
