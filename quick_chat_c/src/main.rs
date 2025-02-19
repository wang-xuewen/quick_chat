mod common;

use anyhow::Result;
use clap::Parser;
use log::info;
use log4rs;

#[derive(Parser)]
struct Args {
    /// Authentication key
    #[arg(short = 'a', long, default_value = "default_key")]
    auth_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 取得命令行参数
    let args = Args::parse();
    // 使用match模式匹配
    match args.auth_key {
        Some(auth_key) => {
            println!("Auth key provided: {}", auth_key);
            // 调用 config 模块的接口设置全局变量
            if let Err(e) = common::set_auth_key(auth_key) {
                eprintln!("Failed to set auth key: {}", e);
                std::process::exit(2);
            }
        }
        None => {
            eprintln!("No auth key provided");
            std::process::exit(3);
        }
    }

    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    Ok(())
}
