use std::{env, fs, path::Path, process::{Child, Command}};

use anyhow::Ok;

use crate::paths;

pub fn setup_wine() -> anyhow::Result<()> {
    let wp = Path::new(paths::WINE_PATH).canonicalize()?.to_string_lossy().to_string();
    Command::new("wine")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("Wine bulunamadı: {}", e));
    clear_temp()?;
    let mut child = Command::new("wineboot")
        .env("WINEPREFIX", wp)
        .spawn()?;

    let status = child.wait()?;

    if status.success() {
        println!("Winecfg başarıyla çalıştı.");
    } else {
        anyhow::bail!("Winecfg çalıştırılamadı! Çıkış kodu: {:?}", status.code());
    }

    Ok(())
}
pub fn run_file(path: &str) -> anyhow::Result<Child> {
    let wp = Path::new(paths::WINE_PATH).canonicalize()?.to_string_lossy().to_string();
    println!("{}",wp);
    let child = Command::new("wine")
        .env("WINEPREFIX", wp)
        .arg(path)
        .spawn()?;
    Ok(child)
}
fn clear_temp() -> anyhow::Result<()> {
          let username = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "unknown".to_string());
      let tmp_dir = Path::new(paths::WINE_PATH)
            .join("drive_c")
            .join("users")
            .join(username)
            .join("AppData")
            .join("Local")
            .join("Temp");

    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir)?;
    }

    fs::create_dir_all(&tmp_dir)?;
    Ok(())
}