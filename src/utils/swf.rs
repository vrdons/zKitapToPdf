use image::Rgb;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use swf::Tag;

use super::renderer::Renderer;

use crate::paths;

pub fn handle_swf(file: &mut File) -> anyhow::Result<()> {
    let swf_buf = swf::decompress_swf(file)?;
    let swf = swf::parse_swf(&swf_buf)?;
    println!("Num frames: {}", swf.header.num_frames());
    let width = 794;
    let height = 1123;
    print!("W: {},H: {}", width, height);
    let mut frame_index = 0;
    let mut renderer = Renderer::new(width, height, Rgb([255, 255, 255]));

    for tag in &swf.tags {
        match tag {
            Tag::SetBackgroundColor(color) => {
                renderer.set_background(Rgb([color.r, color.g, color.b]));
            }

            Tag::ShowFrame => {
                let path =
                    Path::new(paths::TEMP_PATH).join(format!("frame_{:04}.png", frame_index));
                renderer.render(path.to_str().unwrap())?;
                panic!("");
                frame_index += 1;
                renderer = Renderer::new(width, height, Rgb([255, 255, 255]));
            }

            _ => {}
        }
    }

    Ok(())
}
