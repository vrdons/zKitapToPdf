use std::io::Write;
use std::{fs::File, io::BufWriter};

use super::utils;
use image::{Rgb, RgbImage};
use swf::{Font, Shape, Tag};
struct ShapeClass {
    id: u16,
    canvas: RgbImage,
}

pub fn render_image(swf: &swf::Swf) -> anyhow::Result<()> {
    let file = File::create("test.txt")?;
    let mut writer = BufWriter::new(file);
    let w = utils::twips_to_px(11320); // 566 px
    let h = utils::twips_to_px(16140); // 807 px
    let mut image = RgbImage::from_pixel(w, h, Rgb([255, 255, 255])); // beyaz arkaplan

    let mut fonts: Vec<&Font> = Vec::new();
    let mut shapes: Vec<&ShapeClass> = Vec::new();

    for tag in &swf.tags {
        match tag {
            Tag::FileAttributes(_) | Tag::DoAbc(_) | Tag::DoAbc2(_) | Tag::SymbolClass(_) => {
                continue;
            }
            Tag::DefineFont2(font) => fonts.push(&**font),
            Tag::SetBackgroundColor(color) => {
                image = RgbImage::from_pixel(w, h, Rgb([color.r, color.g, color.b]));
            }
            Tag::DefineShape(shape) => {
                shapes.push(&handle_shape(shape));
            }
            _ => writeln!(writer, "{:#?}", tag)?,
        }
    }
    Ok(())
}

fn handle_shape(shape: &Shape) -> ShapeClass {
    let width =
        (shape.shape_bounds.x_max.to_pixels() - shape.shape_bounds.x_min.to_pixels()) as u32;
    let height =
        (shape.shape_bounds.y_max.to_pixels() - shape.shape_bounds.y_min.to_pixels()) as u32;

    let mut canvas = RgbImage::new(width, height);
    let fill_styles: Vec<Rgb<u8>> = shape
        .styles
        .fill_styles
        .iter()
        .map(utils::fill_style_to_rgb)
        .collect();

    let mut fill_style_0: Option<Rgb<u8>> = None;
    let mut fill_style_1: Option<Rgb<u8>> = None;
    let mut line_style: Option<Rgb<u8>> = None;
    for s in &shape.shape {
        match s {
            swf::ShapeRecord::StyleChange(data) => {
                if let Some(idx) = data.fill_style_0 {
                    fill_style_0 = fill_styles.get((idx - 1) as usize).cloned();
                }
                if let Some(idx) = data.fill_style_1 {
                    fill_style_1 = fill_styles.get((idx - 1) as usize).cloned();
                }
                if let Some(idx) = data.line_style {
                    line_style = fill_styles.get((idx - 1) as usize).cloned();
                }
            }
            //swf::ShapeRecord::StraightEdge(data) => {}
            _ => {}
        }
    }

    ShapeClass {
        id: shape.id,
        canvas,
    }
}
