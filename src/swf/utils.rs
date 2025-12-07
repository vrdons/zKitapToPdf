use image::Rgb;
use swf::FillStyle;

pub fn twips_to_px(twips: i32) -> u32 {
    (twips / 20) as u32
}

pub fn fill_style_to_rgb(fill: &FillStyle) -> Rgb<u8> {
    match fill {
        FillStyle::Color(color) => Rgb([color.r, color.g, color.b]),
        _ => Rgb([255, 255, 255]),
    }
}
