use anyhow::Ok;

use std::path::PathBuf;

pub async fn extract_zip(input_path: PathBuf, output_path: PathBuf) -> anyhow::Result<()> {
    use tokio::process::Command;
    Command::new(crate::paths::SEVENZ)
        .arg("x")
        .arg(input_path)
        .arg(format!("-o{}", output_path.to_string_lossy().to_string()))
        .output()
        .await?;
    Ok(())
}
