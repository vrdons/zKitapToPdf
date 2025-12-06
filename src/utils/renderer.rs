use std::collections::HashMap;

use image::{Rgb, RgbImage};
use swf::{CharacterId, Color, Font};
struct FontData<'a> {
    font: &'a swf::Font<'a>,
}

pub struct Renderer<'a> {
    fonts: HashMap<CharacterId, FontData<'a>>,
    width: u32,
    height: u32,
    background_color: Rgb<u8>,
    canvas: RgbImage,
}

impl<'a> Renderer<'a> {
    pub fn new(width: u32, height: u32, background_color: Rgb<u8>) -> Self {
        let canvas = RgbImage::from_pixel(width, height, background_color);
        Self {
            fonts: HashMap::new(),
            width,
            height,
            background_color,
            canvas,
        }
    }
    pub fn save_font(&mut self, font: &'a Font<'a>) {
        self.fonts.insert(font.id, FontData { font });
    }
    pub fn set_background(&mut self, color: Rgb<u8>) {
        self.background_color = color;
    }
    pub fn render(&self, path: &str) -> anyhow::Result<()> {
        self.canvas.save(path)?;
        Ok(())
    }
}
