mod camera;
mod tcp_streamer;

use camera::run_camera;
use tcp_streamer::stream_frames;

use opencv::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to server
    let tcp = TcpStream::connect("127.0.0.1:3000")
        .await
        .expect("Failed to connect to server");

    // Channel for frames
    let (tx, rx) = mpsc::channel::<Vec<u8>>(8);

    // Spawn async TCP streaming task (send headers here)
    let tcp_task = tokio::spawn(async move {
        let mut tcp = tcp; // take ownership
        let headers = b"POST /capture/tenant_camera HTTP/1.1\r\n\
Content-Type: application/octet-stream\r\n\
Transfer-Encoding: chunked\r\n\
\r\n";
        tcp.write_all(headers).await.unwrap();

        stream_frames(tcp, rx).await;
    });

    // Run camera loop on main thread
    run_camera(tx)?; // blocks main thread (needed for highgui)

    tcp_task.await.unwrap();

    Ok(())
}
