use opencv::{core, highgui, imgproc, prelude::*, videoio, Result};
use tokio::sync::mpsc;

/// Capture frames from camera, show preview, and send frames to channel.
/// Must run on main thread (for highgui on macOS)
pub fn run_camera(tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
    // Open camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Camera not opened");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.empty() {
            continue;
        }

        // Convert to RGB
        let mut rgb = Mat::default();
        imgproc::cvt_color(
            &frame,
            &mut rgb,
            imgproc::COLOR_BGR2RGB,
            0,
            core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;

        // Show preview
        highgui::imshow("Camera", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 27 {
            break; // ESC
        }

        // Send frame
        let buf = rgb.data_bytes()?.to_vec();
        futures::executor::block_on(tx.send(buf)).unwrap();
    }

    Ok(())
}
