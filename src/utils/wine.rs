use std::{path::Path, process::{Child, Command}};

use anyhow::Ok;

use crate::paths;

pub fn check_wine() -> anyhow::Result<()> {
    Command::new("wine")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("Wine bulunamadı: {}", e));
    Ok(())
}

pub fn setup_wine() -> anyhow::Result<()> {
    let mut child = Command::new("winecfg")
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