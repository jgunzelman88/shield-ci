use std::path::Path;
use crate::models::config::Config;
use crate::models::application::Technologies;

pub fn detect_technologies(config: &Config) -> Technologies{
  let mut tech = Technologies {
    npm: false,
    pip: false,
    cargo: false,
    docker: false
  } ;

  // Python
  let pip_loc = format!("{}/requirements.txt", config.base_dir);
  let pip_path = Path::new(&pip_loc);
  if pip_path.exists() {
    tech.pip = true;
  }
  // JS/TS
  let npm_loc = format!("{}/package.json", config.base_dir);
  let npm_path = Path::new(&npm_loc);
  if npm_path.exists() {
    tech.npm = true;
  }
  // Rust
  let cargo_loc = format!("{}/cargo.toml", config.base_dir);
  let cargo_path = Path::new(&cargo_loc);
  if cargo_path.exists() {
    tech.cargo = true;
  }
  // Docker
  let docker_loc = format!("{}/Dockerfile", config.base_dir);
  let docker = Path::new(&docker_loc);
  if docker.exists() {
    tech.docker = true
  }
  return tech;
}   