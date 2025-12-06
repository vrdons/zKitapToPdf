use image::{Rgb, RgbImage};
use swf::Color;

pub struct Renderer {
    width: u32,
    height: u32,
    background_color: Rgb<u8>,
    canvas: RgbImage,
}

impl Renderer {
    pub fn new(width: u32, height: u32, background_color: Rgb<u8>) -> Self {
        let canvas = RgbImage::from_pixel(width, height, background_color);
        Self {
            width,
            height,
            background_color,
            canvas,
        }
    }
    pub fn set_background(&mut self, color: Rgb<u8>) {
        self.background_color = color;
    }
    pub fn render(&self, path: &str) -> anyhow::Result<()> {
        self.canvas.save(path)?;
        Ok(())
    }
}
