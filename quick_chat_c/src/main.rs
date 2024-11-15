use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();

    let mut stdin_reader = BufReader::new(io::stdin()).lines();
    let mut server_reader = BufReader::new(reader).lines();

    loop {
        tokio::select! {
            Ok(line) = stdin_reader.next_line() => {
                if let Some(line) = line {
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
            Ok(line) = server_reader.next_line() => {
                if let Some(line) = line {
                    println!("Server: {}", line);
                }
            }
        }
    }
}
