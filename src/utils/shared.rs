use lazy_static::lazy_static;
use log::LevelFilter;
use serde::Serialize;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::{fs, sync::RwLock};
use crate::models::config::Config;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config {
        base_dir: String::from("./"),
        project_id: String::new(),
        shield_server: String::new(),
        shield_user: String::new(),
        shield_pass: String::new(),
        image_path: String::new(),
        image_tag: String::new(),
    });
}

pub fn update_config(update: Config) {
    let mut settings = CONFIG.write().unwrap();
    *settings = update;
}

pub fn get_config() -> Config {
    return CONFIG.read().unwrap().clone();
}

/// Writes object to a JSON file
/// Arguments: 
///    * path: path to wirte the file.
///    * object: object to write.
pub fn write_json_file<T: ?Sized + Serialize>(
    path: &std::path::Path,
    object: &T,
) -> (){
    let root = path.parent();
    if root.is_none() {
        log::error!("Root of path does not exsist");
    }
    let root = path.parent().unwrap();
    if root.exists() == false {
        log::debug!("Making dir {}", root.to_str().unwrap());
        match fs::create_dir_all(root) {
            Ok(()) => (),
            Err(e) => {
                log::error!("Failed to make create directory to store json file: {}", e);
                return;
            }
        }
    }
    let json: String;
    match serde_json::to_string_pretty(object) {
        Ok(val) => json = val,
        Err(e) => {
            log::error!("Failed to convert object to JSON: {}",e);
            return;
        }
    }
    let mut file: File;
    match fs::File::create(path) {
        Ok(val) => file = val,
        Err(e) => {
            log::error!("Failed to create file for JSON: {}",e);
            return;
        }
    }
    match file.write_all(json.as_bytes()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Failed to write file for JSON: {}",e);
            return;
        }
    }
    log::debug!("Wrote File {}",path.as_os_str().to_string_lossy());
}

pub fn set_up_logger(verbose: bool) {
    let verbose_env_str = std::env::var_os("SHIELD_VERBOSE")
    .unwrap_or_default()
    .to_string_lossy()
    .to_ascii_lowercase();
    let verbose_env: bool;
    match FromStr::from_str(&verbose_env_str) {
        Ok(true) => verbose_env = true ,
        Ok(false) => verbose_env = false,
        Err(_) => verbose_env = false 
    }
    let mut level = LevelFilter::Info;
    if verbose || verbose_env {
        level = LevelFilter::Debug;
    }
    SimpleLogger::new().env().with_level(level).init().unwrap();
}