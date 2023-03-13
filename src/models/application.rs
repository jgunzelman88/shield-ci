use super::property_mapping::PropertyMapping;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::models::config::{Config, RESULT_DIR};
use std::io::{Write, Read};
use std::fs;
use std::path;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Application {
    pub name: String,
    pub parent: Option<String>,
    pub subcomponents: Option<Vec::<String>>,
    pub internal_dependencies: Vec::<Dependency>,
    pub external_dependencies: Vec::<Dependency>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub port: Option<String>,
    pub protocol: Option<String>,
    pub property_mappings: Option<Vec::<PropertyMapping>>
}
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Technologies {
    pub npm: bool,
    pub pip: bool,
    pub cargo: bool,
    pub docker: bool,
}

/// Wrties application json file to specified location
pub fn write_application(app: &Application, path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(app)?;
    if path.exists() == false {
      fs::create_dir_all(path)?;
    }
    let file_name = format!("{}/app.json", path.to_str().unwrap());
    let mut file = fs::File::create(file_name)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Reads the curent application json file to get human 
pub fn read_applicaiton(config: &Config) -> Application {
  let mut app_file: fs::File;
  let basic = Application {
      name: String::new(),
      parent: None,
      subcomponents: None,
      internal_dependencies: Vec::new(),
      external_dependencies: Vec::new(),
  };
  match fs::File::open(format!("{}/{}/app.json", config.base_dir, RESULT_DIR)) {
      Ok(file) => app_file = file,
      Err(_) => return basic,
  }
  let mut json = String::new();
  match app_file.read_to_string(&mut json) {
      Ok(_) => (),
      Err(_) => return basic,
  }
  let npm_content: Application = serde_json::from_str(&json).unwrap_or_else(|_| {return basic});
  return npm_content;
}

/// Detects standard files that are associated with certain technoloies to allow scraping of data.
pub fn detect_technologies(config: &Config) -> Technologies{
    let mut tech = Technologies {
      npm: false,
      pip: false,
      cargo: false,
      docker: false
    };
  
    // Python
    let pip_loc = format!("{}/requirements.txt", config.base_dir);
    let pip_path = path::Path::new(&pip_loc);
    if pip_path.exists() {
      tech.pip = true;
    }
    // JS/TS
    let npm_loc = format!("{}/package.json", config.base_dir);
    let npm_path = path::Path::new(&npm_loc);
    if npm_path.exists() {
      tech.npm = true;
    }
    // Rust
    let cargo_loc = format!("{}/cargo.toml", config.base_dir);
    let cargo_path = path::Path::new(&cargo_loc);
    if cargo_path.exists() {
      tech.cargo = true;
    }
    // Docker
    let docker_loc = format!("{}/Dockerfile", config.base_dir);
    let docker = path::Path::new(&docker_loc);
    if docker.exists() {
      tech.docker = true
    }
    return tech;
  }  