use crate::camera::Capture;
use opencv::core::MatTraitConstManual;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Streamer {
    pub capture: Capture,
    tcp: Option<TcpStream>,
}

impl Streamer {
    pub const HEADERS: &'static [u8; 104] = b"POST /capture/tenant_camera HTTP/1.1\r\n\
    Content-Type: application/octet-stream\r\n\
    Connection: keep-alive\r\n\
    \r\n";

    pub fn new(capture: Capture) -> Self {
        Self { capture, tcp: None }
    }

    /// Establish TCP connection and store it
    pub async fn connect(&mut self) {
        let mut tcp = TcpStream::connect("127.0.0.1:3000")
            .await
            .expect("Failed to connect to server");

        if let Err(e) = tcp.write_all(Self::HEADERS).await {
            eprintln!("Server closed during header write: {}", e);
            return;
        }

        self.tcp = Some(tcp);
    }

    /// Stream frames over the existing connection
    pub async fn stream(&mut self) {
        let tcp = self.tcp.as_mut().expect("TCP not connected");

        for frame in &mut self.capture {
            match frame {
                Ok(f) => {
                    f.preview();
                    let buf = f.pixels.data_bytes().unwrap().to_vec();
                    let size = (buf.len() as u32).to_be_bytes();

                    if tcp.write_all(&size).await.is_err() {
                        println!("Server closed connection (size)");
                        break;
                    }

                    if tcp.write_all(&buf).await.is_err() {
                        println!("Server closed connection (frame)");
                        break;
                    }
                }
                Err(e) => eprintln!("Frame stream error: {}", e),
            }
        }
        println!("Stream closed.");
    }
}
