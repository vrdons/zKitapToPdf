use std::fs::File;

pub mod renderer;
pub mod utils;
pub fn handle_swf(file: &mut File) -> anyhow::Result<()> {
    let swf_buf = swf::decompress_swf(file)?;
    let swf = swf::parse_swf(&swf_buf)?;
    renderer::render_image(&swf);
    Ok(())
}
