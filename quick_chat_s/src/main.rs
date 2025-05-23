mod common;
mod qc_server;
mod qc_web;

use anyhow::Result;
use clap::Parser;
use log::{error, info};
use log4rs;
use qc_server::qc_server::start_qc_server;
use qc_web::web_server::start_web_server;
use std::error::Error;

#[derive(Parser)]
struct Args {
    /// Authentication key
    #[arg(short = 'a', long, default_value = "")]
    auth_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    // 初始化日志系统
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    let ip_web = "127.0.0.1".to_string();
    let port_web = 8080;

    let ip_qc = "127.0.0.1".to_string();
    let port_qc = 9080;

    // start_web_server("127.0.0.1", 8080).await?;
    // match start_web_server(ip, port).await {
    //     Ok(()) => {
    //         info!("Web server started successfully on {}:{}", ip, port)
    //     }
    //     Err(e) => {
    //         error!("Error starting web server: {}", e);
    //         std::process::exit(1);
    //     }
    // }

    let web_server = tokio::spawn(async move {
        if let Err(e) = start_web_server(ip_web, port_web).await {
            error!("web server error: {}", e);
        }
    });
    let qc_server = tokio::spawn(async move {
        if let Err(e) = start_qc_server(ip_qc, port_qc).await {
            error!("web server error: {}", e);
        }
    });

    tokio::select! {
        _ = web_server => {},
        _ = qc_server => {}
    }
    Ok(())
}
