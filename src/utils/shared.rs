use chrono::prelude::{DateTime, Utc};
use lazy_static::lazy_static;
use log::LevelFilter;
use serde::Serialize;
use simple_logger::SimpleLogger;
use std::io::Write;
use std::{fs, sync::RwLock};
use crate::models::config::Config;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config {
        base_dir: String::from("./"),
        shield_server: String::new(),
        shield_user: String::new(),
        shield_pass: String::new()
    });
}

pub fn update_config(update: Config) {
    let mut settings = CONFIG.write().unwrap();
    *settings = update;
}

pub fn get_config() -> Config {
    return CONFIG.read().unwrap().clone();
}

pub fn iso_8601(time: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = time.clone().into();
    return dt.to_rfc3339();
}

pub fn write_json_file<T: ?Sized + Serialize>(
    path: &std::path::Path,
    object: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = path.parent();
    if root.is_none() {
        return Err(Box::from("Root of path does not exsist"));
    }
    let root = path.parent().unwrap();
    if root.exists() == false {
        log::debug!("Making dir {}", root.to_str().unwrap());
        fs::create_dir_all(root)?;
    }
    let json = serde_json::to_string_pretty(object)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn set_up_logger(verbose: bool) {
    let mut level = LevelFilter::Info;
    if verbose {
        level = LevelFilter::Debug;
    }
    SimpleLogger::new().env().with_level(level).init().unwrap();
}