use tokio::{io::AsyncWriteExt, net::TcpStream, sync::mpsc};

pub async fn stream_frames(mut tcp: TcpStream, mut rx: mpsc::Receiver<Vec<u8>>) {
    while let Some(data) = rx.recv().await {
        let chunk_size = format!("{:X}\r\n", data.len());
        tcp.write_all(chunk_size.as_bytes()).await.unwrap();
        tcp.write_all(&data).await.unwrap();
        tcp.write_all(b"\r\n").await.unwrap();
    }
}
