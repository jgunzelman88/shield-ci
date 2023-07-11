use super::property_mapping::PropertyMapping;
use crate::models::config::RESULT_DIR;
use crate::utils::shared::get_config;

use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Read;
use std::fs;
use std::path;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Application {
    pub id: Option<String>,
    pub name: String,
    pub project: String,
    pub maintainer: Option<String>,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub subcomponents: Option<Vec::<String>>,
    pub internal_dependencies: Vec::<Dependency>,
    pub external_dependencies: Vec::<Dependency>,
    pub dependency_sets: Vec::<DependencySet>,
    pub branches: Vec::<String>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub port: Option<String>,
    pub protocol: Option<String>,
    pub property_mappings: Option<Vec::<PropertyMapping>>
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct DependencySet {
  pub name: Option<String>,
  pub source: String,
  pub dependencies: Vec<Dependency>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct SubDependency {
    pub version: String,
    pub path: String,
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Technologies {
    pub npm: bool,
    pub pip: bool,
    pub cargo: bool,
    pub docker: bool,
}

/// Reads the curent application json file
pub fn read_applicaiton() -> Application {
  let config = get_config();
  let mut app_file: fs::File;
  let basic = Application {
      id: None,
      name: String::new(),
      project: String::new(),
      parent: None,
      description: None,
      maintainer: None,
      subcomponents: None,
      internal_dependencies: Vec::new(),
      external_dependencies: Vec::new(),
      dependency_sets: Vec::new(),
      branches: Vec::new()
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
  let content: Application = serde_json::from_str(&json).unwrap_or_else(|_| {return basic});
  return content;
}

/// Detects standard files that are associated with certain technoloies to allow scraping of data.
pub fn detect_technologies() -> Technologies{
    let config = get_config();
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