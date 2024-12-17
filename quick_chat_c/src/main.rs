use anyhow::Result;
use log::info;
use log4rs;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    Ok(())
}
