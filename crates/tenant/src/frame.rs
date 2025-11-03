use opencv::{core::Mat, highgui};

pub struct Frame {
    pub pixels: Mat,
}

impl Frame {
    pub fn new(pixels: Mat) -> Self {
        Self { pixels }
    }

    pub fn to_rgb(&mut self) -> opencv::Result<()> {
        let mut rgb = Mat::default();
        opencv::imgproc::cvt_color(
            &self.pixels,
            &mut rgb,
            opencv::imgproc::COLOR_BGR2RGB,
            0,
            opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )?;
        self.pixels = rgb;
        Ok(())
    }

    pub fn preview(&self) -> opencv::Result<()> {
        highgui::imshow("Camera", &self.pixels)?;
        let key = highgui::wait_key(1)?;
        if key == 27 {
            std::process::exit(0); // Exit on ESC
        }
        Ok(())
    }
}
