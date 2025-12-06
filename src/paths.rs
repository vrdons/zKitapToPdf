pub const TEMP_PATH: &str = "tmp";
pub const INPUT_PATH: &str = "in";
pub const OUTPUT_PATH: &str = "out";

#[cfg(target_os = "linux")]
pub const WINE_PATH: &str = "wine";

#[cfg(target_os = "linux")]
pub const SEVENZ: &str = "bin/7z/7zz";

#[cfg(target_os = "windows")]
pub const SEVENZ: &str = "bin/7z/7za.exe";
