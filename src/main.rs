use std::fs::FileType;

use chrono::Utc;
use models::Config;

pub mod models;

fn main() {
    if let Err(e) = process() {
        std::fs::write("error_log.txt", e).unwrap();
    }
}

fn process() -> Result<(), String> {
    let mut log = String::new();
    let config = if let Some(config) = open_or_create_config() {
        config
    } else {
        return Ok(());
    };
    let backup_path = std::path::Path::new(&config.backup_path);
    if config.backup_path.is_empty() || !backup_path.exists() {
        if config.backup_path.is_empty() {
            log.push_str("backup path is empty");
        } else {
            log.push_str(&format!("backup path {:#?} doesn't exist", backup_path));
        }
        return Err(log);
    }
    let dt = Utc::now();
    let folder_name = dt.format("%Y.%m.%d_%H:%M:%S").to_string();
    if std::fs::create_dir(&folder_name).is_err() {
        log.push_str("failed to create the backup target folder {folder_name}");
        return Err(log);
    }
    for dir_entry_result in std::fs::read_dir(config.backup_path).unwrap() {
        if let Ok(dir_entry) = dir_entry_result {
            if dir_entry.file_type().unwrap().is_dir() {
                let path = std::path::Path::new(&folder_name); //.join(dir_entry.file_name());
                                                               // std::fs::create_dir(&path).unwrap();
                let mut options = fs_extra::dir::CopyOptions::new();
                options.copy_inside = false;
                if let Err(e) = fs_extra::dir::copy(&dir_entry.path(), &path, &options) {
                    let err = format!(
                        "Failed to copy a folder {} over: {:#?}",
                        dir_entry.path().to_string_lossy(),
                        e
                    );
                    log.push_str(&err);
                    eprintln!("{err}");
                }
            } else if let Err(e) = std::fs::copy(
                &dir_entry.path(),
                &std::path::Path::new(&folder_name).join(dir_entry.file_name()),
            ) {
                let err = format!(
                    "Failed to copy a file {} over: {:#?}",
                    dir_entry.path().to_string_lossy(),
                    e
                );
                log.push_str(&err);
                eprintln!("{err}");
            }
        } else {
            eprintln!("Error reading a path!");
        }
    }
    if log.is_empty() {
        Ok(())
    } else {
        Err(log)
    }
}

fn open_or_create_config() -> Option<Config> {
    if let Ok(file) = std::fs::read_to_string("config.toml") {
        toml::from_str(&file).ok()
    } else {
        let tmp = Config::default();
        std::fs::write("config.toml", toml::to_string_pretty(&tmp).ok().unwrap()).unwrap();
        None
    }
}
