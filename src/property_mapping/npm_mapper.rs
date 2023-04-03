use crate::models::application::{Application, Dependency, read_applicaiton};
use crate::models::property_mapping;
use crate::models::config::Config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct NpmDependency {
    pub version: String,
}
#[derive(Serialize, Deserialize)]
struct PackageLock {
    pub name: String,
    pub dependencies: HashMap<String, NpmDependency>,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
struct NpmPackage {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub version: String,
}


/// Maps the application data from npm.
/// 
pub fn map_application(config: &Config) -> Result<Application, Box<dyn std::error::Error>> {
    // Process npm file
    let mut package_lock_file = File::open(format!("{}/package-lock.json", config.base_dir))?;
    let mut package_lock_json = String::new();
    package_lock_file.read_to_string(&mut package_lock_json)?;
    let package_lock: PackageLock = serde_json::from_str(&package_lock_json)?;

    let mut package_file = File::open(format!("{}/package.json", config.base_dir))?;
    let mut package_json = String::new();
    package_file.read_to_string(&mut package_json)?;
    let package: NpmPackage = serde_json::from_str(&package_json)?;
    // Set Dependencies 
    let inter_deps: Vec<Dependency> = package
        .dependencies
        .into_iter()
        .map(|dep| {
            let version = package_lock.dependencies.get(&dep.0).unwrap().version.clone();
            return Dependency {
                name: dep.0,
                version: version,
                port: None,
                property_mappings: None,
                protocol: None,
            };
        })
        .collect();
    // Get current app file;
    let current_app = read_applicaiton(config);
    // Process external dependencies
    let mut external_deps: Vec<Dependency> = Vec::new();
    for exp_dep in current_app.external_dependencies {
        log::info!("Processing {}", &exp_dep.name);
        external_deps.push(property_mapping::process_dependency(exp_dep));
    }
    Ok(Application {
        id: current_app.id,
        name: package_lock.name,
        description: None,
        parent: current_app.parent,
        subcomponents: current_app.subcomponents,
        internal_dependencies: inter_deps,
        external_dependencies: external_deps,
    })
}
