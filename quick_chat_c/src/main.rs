mod common;
mod qc_web;

use anyhow::Result;
use clap::Parser;
use log::info;
use log4rs;
use qc_web::call_auth;

#[derive(Parser)]
struct Args {
    /// host ip ,服务端ip
    #[arg(short = 's', long, default_value = "127.0.0.1")]
    host_ip: Option<String>,

    /// Authentication key
    #[arg(short = 'a', long, default_value = "")]
    auth_key: Option<String>,

    /// nick name
    #[arg(short = 'n', long)]
    nick_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 取得命令行参数
    get_para()?;

    // 初始化日志系统
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    // 调用认证接口
    match call_auth::call_auth(common::get_nick_name(), common::get_auth_key()).await {
        Ok(token) => {
            println!("Received token: {}", token);
            if let Err(e) = common::set_auth_token(token) {
                eprintln!("Failed to set auth token: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn get_para() -> Result<()> {
    let args = Args::parse();

    match args.host_ip {
        Some(host_ip) => {
            println!("host ip provided: {}", host_ip);
            // 调用 config 模块的接口设置全局变量
            if let Err(e) = common::set_host_ip(host_ip) {
                eprintln!("Failed to set host ip: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No host ip provided");
            std::process::exit(1);
        }
    }

    // 使用match模式匹配
    match args.auth_key {
        Some(auth_key) => {
            println!("Auth key provided: {}", auth_key);
            // 调用 config 模块的接口设置全局变量
            if let Err(e) = common::set_auth_key(auth_key) {
                eprintln!("Failed to set auth key: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No auth key provided");
            std::process::exit(1);
        }
    }

    match args.nick_name {
        Some(nick_name) => {
            println!("nick name provided: {}", nick_name);
            // 调用 config 模块的接口设置全局变量
            if let Err(e) = common::set_nick_name(nick_name) {
                eprintln!("Failed to set nick name: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No nick name provided");
            std::process::exit(1);
        }
    }
    Ok(())
}
