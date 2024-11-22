mod qc_server;

use crate::qc_server::QcServer;
use anyhow::Result;
use log::{error, info};
use log4rs;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");
    // 输出日志 sample
    // debug!("This is an debug message.");
    // info!("This is an info message.");
    // warn!("This is a warning message.");
    // error!("This is an error message.");

    let addr = "127.0.0.1:8080";

    let result = QcServer::new(&addr).await;
    match result {
        Ok(server) => {
            if let Err(e) = server.start().await {
                error!("qc server start Error: {}", e);
            };
        }
        Err(e) => {
            error!("qc server new Error: {}", e);
        }
    }

    Ok(())
}
