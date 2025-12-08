use std::path::Path;

use crate::{cli::Args, utils::clear_dir};

use anyhow::Result;
use clap::Parser;

pub mod cli;
pub mod executable;
pub mod paths;
pub mod utils;

fn main() -> Result<()> {
    let arg = Args::parse();
    let (input, output, scale) = arg.validate()?;

    println!("{:?}, {:?}, {}", input, output, scale);
    clear_dir(Path::new(paths::TEMP_DIR))?;
    Ok(())
}
