use crate::paths;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;
use std::io::Read;

pub fn setup() {
    let _ = fs::create_dir_all(paths::INPUT_PATH);
    let _ = fs::create_dir_all(paths::OUTPUT_PATH);
    let _ = clear_temp();
}

pub fn check_input() -> anyhow::Result<Vec<String>> {
    let mut items = Vec::new();
    scan_dir(paths::INPUT_PATH, &mut items)?;
    Ok(items)
}

pub fn watch_and_copy(
    path: &PathBuf,
    out: &PathBuf,
    extension: &str,
    stop: Arc<AtomicBool>,
) -> anyhow::Result<()> 
{
    fs::create_dir_all(out)?;

    let mut child = Command::new("inotifywait")
        .args([
            "-m",
            "-r",
            &path.to_string_lossy().to_string(),
            "--format",
            "%w%f %e",
            "-e", "create",
            "-e", "modify",
            "--exclude", ".*\\.tmp",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = child.stdout.take().unwrap();

    println!("✔ inotifywait başlatıldı");

    let mut buf = [0u8; 4096];
    let mut text_buffer = String::new();

    loop {
        if stop.load(Ordering::Relaxed) {
            let _ = child.kill();
            break;
        }

        match stdout.read(&mut buf) {
            Ok(0) => {
                thread::sleep(Duration::from_millis(30));
                continue;
            }
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                text_buffer.push_str(&chunk);

                // newline'lara göre satır satır işle
                while let Some(pos) = text_buffer.find('\n') {
                    let line = text_buffer[..pos].trim().to_string();
                    text_buffer = text_buffer[pos + 1..].to_string();

                    if line.is_empty() {
                        continue;
                    }

                    println!("> {}", line);

                    if !line.contains(extension) {
                        continue;
                    }
                    if !(line.contains("CREATE") || line.contains("MODIFY")) {
                        continue;
                    }

                    if let Some((filepath, _event)) = line.rsplit_once(' ') {
                        let src = PathBuf::from(filepath);
                        let filename = src.file_name().unwrap();
                        let dest = out.join(filename);

                        if fs::copy(&src, &dest).is_ok() {
                            println!(
                                "✔ DLL yakalandı: {} → {}",
                                filename.to_string_lossy(),
                                dest.display()
                            );
                        }
                    }
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(30));
                continue;
            }
        }
    }

    Ok(())
}


pub fn find_closest_folder(base: &Path, target: &str) -> Option<PathBuf> {
    let mut best: Option<(PathBuf, f32)> = None;

    for entry in fs::read_dir(base).ok()? {
        let entry = entry.ok()?;
        let meta = entry.metadata().ok()?;
        if !meta.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let sim = simple_similarity(&name, target);

        match best {
            None => best = Some((entry.path(), sim)),
            Some((_, best_sim)) => {
                if sim > best_sim {
                    best = Some((entry.path(), sim));
                }
            }
        }
    }

    best.map(|(path, _)| path)
}

fn simple_similarity(a: &str, b: &str) -> f32 {
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    let len_a = ab.len();
    let len_b = bb.len();
    let min_len = len_a.min(len_b);

    let mut score = 0;

    for i in 0..min_len {
        if ab[len_a - 1 - i] == bb[len_b - 1 - i] {
            score += 1;
        }
    }

    score as f32 / min_len as f32
}

fn scan_dir<P: AsRef<std::path::Path>>(path: P, out: &mut Vec<String>) -> anyhow::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path_buf = entry.path();
        let path_str = path_buf.to_string_lossy().to_string();

        out.push(path_str.clone());

        if entry.metadata()?.is_dir() {
            scan_dir(path_buf, out)?;
        }
    }
    Ok(())
}

fn clear_temp() -> anyhow::Result<()> {
    let tmp_dir = Path::new(paths::TEMP_PATH);

    if tmp_dir.exists() {
        fs::remove_dir_all(tmp_dir)?;
    }

    fs::create_dir_all(tmp_dir)?;

    Ok(())
}
