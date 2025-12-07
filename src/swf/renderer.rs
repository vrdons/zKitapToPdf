use std::io::Write;
use std::{fs::File, io::BufWriter};

use image::{Rgb, RgbImage};
use swf::{Font, Tag};

pub fn render_image(swf: &swf::Swf) -> anyhow::Result<()> {
    let mut fonts: Vec<&Font> = Vec::new();
    let file = File::create("test.txt")?;
    let mut writer = BufWriter::new(file);
    let w = super::utils::twips_to_px(11320); // 566 px
    let h = super::utils::twips_to_px(16140); // 807 px
    let mut image = RgbImage::from_pixel(w, h, Rgb([255, 255, 255])); // beyaz arkaplan

    for tag in &swf.tags {
        match tag {
            Tag::FileAttributes(_) | Tag::DoAbc(_) | Tag::DoAbc2(_) | Tag::SymbolClass(_) => {
                continue;
            }
            Tag::DefineFont2(font) => fonts.push(&**font),
            Tag::SetBackgroundColor(color) => {
                image = RgbImage::from_pixel(w, h, Rgb([color.r, color.g, color.b]));
            }
            _ => writeln!(writer, "{:#?}", tag)?,
        }
    }
    Ok(())
}
