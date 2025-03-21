use anyhow::{Context, Result};
use log::info;
use qc_lib::QcMessage;
use serde_json;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct QcClient {}

impl QcClient {
    pub async fn open(address: &str) -> Result<()> {
        let stream = TcpStream::connect(address)
            .await
            .context("Failed to connect qc server")?;

        info!("connect to: {}", address.to_string());
        let (reader, mut writer) = stream.into_split();

        let mut stdin_reader = BufReader::new(io::stdin()).lines();
        let mut server_reader = BufReader::new(reader).lines();

        loop {
            tokio::select! {
                Ok(line) = stdin_reader.next_line() => {
                    if let Some(line) = line {
                        let qc_message = QcMessage {
                            nick_name: "Alice".to_string(),
                            message: (&line).to_string(),
                            cmd: "".to_string(),
                        };
                        // 序列化为 Vec<u8>
                        // let serialized = bincode::serialize(&qc_message).expect("Failed to serialize");
                        let serialized = serde_json::to_string(&qc_message).expect("Failed to serialize");


                        // 转换为 &[u8]
                        // let bytes: &[u8] = &serialized;

                        writer.write_all(serialized.as_bytes()).await?;
                        writer.write_all(b"\n").await?;
                    }
                }
                Ok(line) = server_reader.next_line() => {
                    if let Some(line) = line {

                        // 反序列化
                        let qc_message: QcMessage = serde_json::from_str(&line).expect("Failed to deserialize");
                        // let deserialized: QcMessage = bincode::deserialize(&serialized).expect("Failed to deserialize");

                        info!("message from server: {}", qc_message.message);
                    }
                }
            }
        }
    }
}
