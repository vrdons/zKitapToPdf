use std::{env, fs, path::Path, thread, time::Duration};

pub mod paths;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::filesystem::setup();
    let files = utils::filesystem::check_input()
        .map_err(|e| anyhow::anyhow!("check_input hat verdi: {}", e))?;
    if files.is_empty() {
        return Err(anyhow::anyhow!("input klasörü boş"));
    }
    for item in files {
        #[cfg(target_os = "linux")]
        utils::wine::setup_wine()?;

        utils::filesystem::setup();
        #[cfg(target_os = "linux")]
        let temp_data = utils::wine::get_temp_path()?;
        #[cfg(target_os = "linux")]
        let child = &mut utils::wine::run_file(&item)?;
        //TODO: exec for windows
        break;
        //panic!("{:?}", output);
    }
    Ok(())
}
