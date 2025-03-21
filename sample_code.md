use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time,
};

const VALID_TOKEN: &str = "secret_token_2025";
const TIMEOUT_SECONDS: u64 = 60;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(socket: TcpStream) {
    let (read_stream, mut write_stream) = socket.into_split();
    let mut reader = BufReader::new(read_stream);
    let mut buffer = String::new();

    // 创建超时通道
    let (tx, mut rx) = mpsc::channel(1);

    // 任务1：等待客户端发送 Token
    let token_task = tokio::spawn(async move {
        if let Ok(bytes_read) = reader.read_line(&mut buffer).await {
            if bytes_read > 0 {
                let received_token = buffer.trim();
                if received_token == VALID_TOKEN {
                    let _ = tx.send(()).await; // 发送成功信号
                    return Ok(());
                }
            }
        }
        Err(())
    });

    // 任务2：超时检测
    let timeout_task = time::sleep(Duration::from_secs(TIMEOUT_SECONDS));

    tokio::select! {
        _ = token_task => {
            println!("Valid token received, connection maintained.");
        },
        _ = timeout_task => {
            println!("Timeout: No valid token received in {} seconds.", TIMEOUT_SECONDS);
            let _ = write_stream.shutdown().await; // 关闭连接
        },
        _ = rx.recv() => {} // 收到成功信号时取消超时
    }
}


let (abort_tx, mut abort_rx) = tokio::sync::oneshot::channel();
let handle = tokio::spawn(async move {
    tokio::select! {
        _ = time::sleep(Duration::from_secs(common::TIMEOUT_AUTH)) => {
            info!("Timeout triggered");
        },
        _ = abort_rx => {
            info!("Timeout canceled");
        }
    }
});

loop {
    tokio::select! {
        // 其他业务逻辑分支
        _ = some_condition() => {
            let _ = abort_tx.send(());
        }
    }
}
