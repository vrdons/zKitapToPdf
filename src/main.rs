use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub mod paths;
pub mod utils;

fn main() -> anyhow::Result<()> {
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
        let roaming = utils::wine::get_app_roaming(item.clone())?;
        println!("{}", roaming.to_string_lossy().to_string());
        let temp_clone = Path::new(paths::TEMP_PATH).join("dlls").clone();
        let roaming_clone = roaming.clone();

        let stop = Arc::new(AtomicBool::new(false));
        let stop2 = stop.clone();

        let handle_copy = std::thread::spawn(move || {
            utils::filesystem::watch_and_copy(&roaming_clone, &temp_clone, "dll", stop2).unwrap()
        });
        #[cfg(target_os = "linux")]
        let child = &mut utils::wine::run_file(&item)?;
        //TODO: exec for windows
        child.wait()?;
        println!("Child finished");
        std::thread::sleep(Duration::from_millis(1500));
        stop.store(true, Ordering::Relaxed);
        break;
        //panic!("{:?}", output);
    }
    Ok(())
}
