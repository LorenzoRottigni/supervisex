use opencv::{core, highgui, prelude::*, videoio, Result};

use crate::frame::Frame;

/// Struct that owns a camera and allows iterating over frames
pub struct Capture {
    cam: videoio::VideoCapture,
}

impl Capture {
    /// Open a new camera (camera_index = 0 by default)
    pub fn new(camera_index: i32) -> Result<Self> {
        let cam = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)?;
        if !videoio::VideoCapture::is_opened(&cam)? {
            panic!("Camera not opened");
        }
        Ok(Self { cam })
    }
}

impl Iterator for Capture {
    type Item = Result<Frame>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut frame = Mat::default();

            match self.cam.read(&mut frame) {
                Ok(_) => {
                    if frame.empty() {
                        continue; // Skip empty frame, keep streaming
                    }
                    return Some(Ok(Frame::new(frame)));
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
