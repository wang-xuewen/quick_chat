use anyhow::{Context, Result};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct QcClient {
    stream: TcpStream,
}

impl QcClient {
    pub async fn new(address: &str) -> Result<Self> {
        let stream = TcpStream::connect(address)
            .await
            .context("Failed to connect qc server")?;

        println!("connect to: {}", address.to_string());
        Ok(QcClient { stream })
    }

    pub async fn start(&self) -> result<()> {
        let (reader, mut writer) = self.stream.into_split();
        let mut reader_stdin = BuffReader::new(io::stdin()).lines;
        let mut reader_qc = BuffReader::new(reader).lines;
        loop {
tokio::select! {
    
}
        }
    }
}
