use anyhow::{Context, Error, Result};
use log::{debug, error, info};
use qc_lib::QcMessage;
// use serde::Deserialize;

use crate::common::get_global_map;
use serde_json; // 使用 serde_json 来反序列化

use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tokio::time;

pub struct QcServer {
    listener: TcpListener,
    tx: Arc<Mutex<broadcast::Sender<String>>>,
}
impl QcServer {
    // 创建一个新的 QcServer 实例
    pub async fn new(address: &str) -> Result<Self> {
        let listener = TcpListener::bind(address)
            .await
            .context("Failed to bind TCP listener")?;

        info!("bind: {}", address.to_string());

        let (tx, _) = broadcast::channel(10000);
        Ok(QcServer {
            listener,
            tx: Arc::new(Mutex::new(tx)),
        })
    }

    // 启动服务器并开始接收客户端连接
    pub async fn start(&self) -> Result<()> {
        loop {
            let (socket, _) = self.listener.accept().await?;
            let tx = self.tx.clone(); // 克隆发送端

            // 启动一个新的异步任务来处理每个连接
            tokio::spawn(async move {
                if let Err(e) = handle_client(socket, tx)
                    .await
                    .context("handle client error")
                {
                    error!("Client handler failed: {}", e);
                }
            });
        }
    }
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    tx: Arc<Mutex<broadcast::Sender<String>>>,
) -> Result<()> {
    // task::block_in_place() 会在一个专门的线程池中执行同步代码，而不会阻塞当前的异步任务所在的线程。
    // 也就是说，socket.peer_addr() 的执行会被阻塞到一个独立的线程中，不会占用 Tokio 的异步线程。
    let peer_addr = tokio::task::block_in_place(|| socket.peer_addr());

    if let Ok(peer_addr) = peer_addr {
        info!("Client connected from: {}", peer_addr);
    }

    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    let mut rx = tx.lock().await.subscribe(); // 订阅广播

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(value) if value == 0 => break,// 客户端关闭连接
                    Err(e) => {
                        error!("Read Error: {}", e);
                        continue; // 跳过本次循环
                    },
                    Ok(_) => (),
                }

                debug!("recieve: {}",line);

                // 反序列化
                // let qc_message: QcMessage = serde_json::from_str(&line).expect("Failed to deserialize");

                let qc_message = match serde_json::from_str::<QcMessage>(&line) {
                    Ok(msg) => {
                        info!("recieve obj: {:?}",msg);
                        msg
                    }
                    Err(e) => {
                        // 记录错误并保存原始数据
                        error!("unmarshal Error: {}", e);
                        continue;
                    }
                };

                let map = get_global_map();
                if !map.is_exist(&qc_message.token) {
                    error!("token is valid: {}" ,qc_message.token);
                    continue;
                }
                if let Err(e)=tx.lock().await.send(line.clone()) {
                    error!("tx send Error: {}", e);
                    continue;
                }
                line.clear(); // 清空行缓存
            },
            result = rx.recv() => {

                match result {
                    Ok(value) => {

                        writer.write_all(value.as_bytes()).await?
                    },
                    Err(e) => {
                        error!("rx recv Error: {}", e);
                        break;
                    },
                }

            },
        }
    }

    Ok(())
}

pub async fn start_qc_server(ip: String, port: u16) -> Result<()> {
    // let addr = "127.0.0.1:8080";
    let addr = format!("{}:{}", ip, port);

    let qc_server = QcServer::new(&addr)
        .await
        .context("create qc server failed")?;

    qc_server.start().await.context("start qc server failed")?;

    Ok(())
}
