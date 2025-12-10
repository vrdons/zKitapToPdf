use std::{
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
    },
};
use walkdir::WalkDir;

use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

pub fn clear_dir(dir: &PathBuf) -> Result<()> {
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    fs::create_dir_all(dir)?;
    Ok(())
}

pub fn find_files(path: &Path, extension: &str) -> anyhow::Result<Vec<String>> {
    let mut dlls = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case(extension))
                .unwrap_or(false)
        {
            dlls.push(entry.path().to_string_lossy().to_string());
        }
    }

    if dlls.is_empty() {
        anyhow::bail!("temp folder is empty or dll not found");
    }

    dlls.sort_by_key(|path| {
        let stem = Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let last_digit = stem
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(999);
        (last_digit, stem)
    });

    Ok(dlls)
}
pub fn watch_and_copy_swf(path: &PathBuf, out: &Path, stop: Arc<AtomicBool>) -> Result<()> {
    if path.is_file() {
        anyhow::bail!("watch path must be a DIRECTORY, but file given: {:?}", path);
    }

    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    println!("Watching: {:?}", path.display());

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
    watcher.watch(&path.canonicalize()?, RecursiveMode::Recursive)?;

    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        match rx.recv() {
            Ok(event) => {
                let event = event?;
                if let notify::EventKind::Modify(_) = event.kind {
                    let path = event
                        .paths
                        .first()
                        .ok_or_else(|| anyhow::anyhow!("No path found"))?;

                    if path.file_stem().and_then(OsStr::to_str) == Some("p") {
                        continue;
                    }

                    if let Ok(bytes) = std::fs::read(path) {
                        if bytes.len() >= 3 {
                            let header = &bytes[..3];

                            let is_swf = header == b"FWS" || header == b"CWS" || header == b"ZWS";

                            if !is_swf {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    let out_path = out.join(
                        path.file_name()
                            .ok_or_else(|| anyhow::anyhow!("Failed to get file name"))?,
                    );

                    println!("copy: {:?}", path);

                    if let Err(e) = std::fs::copy(path.canonicalize()?, &out_path) {
                        println!("Failed to copy {:?} -> {:?}: {:?}", path, out_path, e);
                    }
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
    Ok(())
}
