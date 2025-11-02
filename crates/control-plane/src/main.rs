use axum::{body::Body, extract::Path, routing::post, Router};
use futures::TryStreamExt;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_util::io::StreamReader;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/capture/{camera_id}", post(handle_capture));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on 0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn handle_capture(Path(camera_id): Path<String>, body: Body) -> String {
    println!("Receiving stream from camera: {}", camera_id);

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32); // bounded channel

    // Spawn a task to process frames
    let camera_id_clone = camera_id.clone();
    tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            // TODO: process each frame here
            println!(
                "Processing {} bytes from camera {}",
                frame.len(),
                camera_id_clone
            );
        }
    });

    let stream = body
        .into_data_stream()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
    let mut reader = StreamReader::new(stream);

    let mut buffer = [0u8; 4096];
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("Stream ended for camera {}", camera_id);
                break;
            }
            Ok(n) => {
                let frame = buffer[..n].to_vec();
                // Send frame to processing task
                if tx.send(frame).await.is_err() {
                    eprintln!("Processing task dropped for camera {}", camera_id);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading stream for {}: {:?}", camera_id, e);
                break;
            }
        }
    }

    format!("Stream from camera {} finished", camera_id)
}
