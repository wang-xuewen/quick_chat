mod common;
mod qc_web;

use anyhow::Result;
use log::{error, info};
use log4rs;
use qc_web::web_server::start_web_server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    let ip = "127.0.0.1";
    let port = 8080;

    // start_web_server("127.0.0.1", 8080).await?;
    match start_web_server(ip, port).await {
        Ok(()) => {
            info!("Web server started successfully on {}:{}", ip, port)
        }
        Err(e) => {
            error!("Error starting web server: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
