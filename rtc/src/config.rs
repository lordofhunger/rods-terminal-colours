use std::{fs, io, path::PathBuf};
use dirs;

pub fn find_kitty_config_path() -> Option<PathBuf> {
    if let Some(mut path) = dirs::config_dir() {
        path.push("kitty");
        path.push("kitty.conf");
        if path.exists() {
            return Some(path);
        }
    }

    if let Some(mut path) = dirs::home_dir() {
        path.push(".kitty.conf");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub fn get_rtc_config_dir() -> Result<PathBuf, io::Error> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;
    path.push("rtc");
    fs::create_dir_all(&path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to create RTC config directory: {}", e)))?;
    Ok(path)
}

pub fn get_colours_backup_path(backup_name: &Option<String>) -> Result<PathBuf, io::Error> {
    let mut path = get_rtc_config_dir()?;
    let filename = if let Some(name) = backup_name {
        format!("{}.rtc_colours", name)
    } else {
        "default.rtc_colours".to_string()
    };
    path.push(filename);
    Ok(path)
}
