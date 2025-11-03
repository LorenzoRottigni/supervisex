mod camera;
mod frame;
mod tcp_streamer;

use crate::tcp_streamer::Streamer;
use camera::Capture;
use opencv::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let capture = Capture::new(0)?;
    let mut streamer = Streamer::new(capture);

    // streamer.connect().await;

    streamer.stream().await;

    Ok(())
}
