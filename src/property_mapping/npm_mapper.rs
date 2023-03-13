use crate::models::application::{Application, Dependency, read_applicaiton};
use crate::models::config::Config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct PackageJson {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub version: String,
}

/// Maps the application data from npm.
pub fn map_application(config: &Config) -> Result<Application, Box<dyn std::error::Error>> {
    let mut npm_file = File::open(format!("{}/package.json", config.base_dir))?;
    let mut json = String::new();
    npm_file.read_to_string(&mut json)?;
    let npm_content: PackageJson = serde_json::from_str(&json)?;
    let inter_deps: Vec<Dependency> = npm_content
        .dependencies
        .into_iter()
        .map(|dep| {
            return Dependency {
                name: dep.0,
                version: dep.1,
                port: None,
                property_mappings: None,
                protocol: None,
            };
        })
        .collect();
    let current_app = read_applicaiton(config);
    Ok(Application {
        name: npm_content.name,
        parent: current_app.parent,
        subcomponents: current_app.subcomponents,
        internal_dependencies: inter_deps,
        external_dependencies: current_app.external_dependencies,
    })
}
