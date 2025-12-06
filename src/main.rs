use std::{
    fs::File, path::Path, sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    }, time::Duration
};

use crate::utils::{filesystem::check_files, sfw::extract_cws};

pub mod paths;
pub mod utils;

fn main() -> anyhow::Result<()> {
    utils::filesystem::setup();
    let files = utils::filesystem::check_files(paths::INPUT_PATH)
        .map_err(|e| anyhow::anyhow!("check_input hat verdi: {}", e))?;
    if files.is_empty() {
        return Err(anyhow::anyhow!("input klasörü boş"));
    }
    for item in files {
        let temp = Path::new(paths::TEMP_PATH).join("dlls");
        #[cfg(target_os = "linux")]
        utils::wine::setup_wine()?;

        utils::filesystem::setup();

        #[cfg(target_os = "linux")]
        let roaming = utils::wine::get_app_roaming(item.clone())?;
        println!("{}", roaming.to_string_lossy().to_string());
        let temp_clone = temp.clone();
        let roaming_clone = roaming.clone();

        let stop = Arc::new(AtomicBool::new(false));
        let stop2 = stop.clone();

        let _ = std::thread::spawn(move || {
            utils::filesystem::watch_and_copy(&roaming_clone, &temp_clone, "dll", stop2).unwrap()
        });
        #[cfg(target_os = "linux")]
        let child = &mut utils::wine::run_file(&item)?;
        //TODO: exec for windows
        child.wait()?;
        println!("Child finished");
        std::thread::sleep(Duration::from_millis(5000));
        stop.store(true, Ordering::Relaxed);
        
        let dlls = check_files(&temp.to_string_lossy().to_string())?;
        if dlls.is_empty() {
            return Err(anyhow::anyhow!("temp klasörü boş"));
        }
        for dll in dlls {
            let mut read = File::open(dll)?;
            extract_cws(&mut read)?;
        }
        break;
        //panic!("{:?}", output);
    }
    Ok(())
}
