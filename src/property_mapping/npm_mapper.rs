use crate::models::application::{read_applicaiton, Application, Dependency};
use crate::models::dependecy_report::{DependencyReport, Vulnerability};
use crate::models::property_mapping;
use crate::models::trivy::TrivyReport;
use crate::utils::shared::{get_config, iso_8601};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct LeafDependency {
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DependencyType {
    Obj(LeafDependency),
    String(String),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct RootDependency {
    pub version: String,
    pub dependencies: Option<HashMap<String, DependencyType>>,
    pub requires: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PackageLock {
    pub name: String,
    pub dependencies: Option<HashMap<String, RootDependency>>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NpmPackage {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub version: String,
}

lazy_static! {
    static ref PACKAGE_LOCK: RwLock<PackageLock> = RwLock::new(read_package_lock());
}
/// Maps the application data from npm.
/// # Arguments
///    * config: &Config - Config reference from main. Used to get desired scan path.
pub fn map_application() -> Result<Application, Box<dyn std::error::Error>> {
    log::debug!("Reading npm package lock file");
    let config = get_config();
    let package_lock = get_package_lock();
    log::debug!("Reading npm package file");
    let mut package_file = File::open(format!("{}/package.json", config.base_dir))?;
    let mut package_json = String::new();
    package_file.read_to_string(&mut package_json)?;
    let package: NpmPackage = serde_json::from_str(&package_json)?;

    log::debug!("Processing Internal Dependencies ");
    let mut inter_deps: Vec<Dependency> = Vec::new();
    for (name, version) in package.dependencies {
        let mut version = version;
        let lock_verison = get_package_lock_version(&name);
        if lock_verison.is_some() {
            version = lock_verison.unwrap().to_string()
        }
        inter_deps.push(Dependency {
            name: name,
            version: version,
            port: None,
            property_mappings: None,
            protocol: None,
        });
    }

    log::debug!("Looking for exsisting app file");
    let current_app = read_applicaiton();

    log::debug!("Processing External Dependencies");
    let mut external_deps: Vec<Dependency> = Vec::new();
    for exp_dep in current_app.external_dependencies {
        log::info!("Processing {}", &exp_dep.name);
        external_deps.push(property_mapping::process_dependency(&config, exp_dep));
    }
    Ok(Application {
        id: current_app.id,
        name: package_lock.name,
        description: package.description,
        maintainer: None,
        parent: current_app.parent,
        subcomponents: current_app.subcomponents,
        internal_dependencies: inter_deps,
        external_dependencies: external_deps,
    })
}

pub fn get_dependency_report(
    trivy: &TrivyReport,
    app: &Application,
) -> Result<DependencyReport, Box<dyn std::error::Error>> {
    let now = std::time::SystemTime::now();
    let mut vulnerabilities: Vec<Vulnerability> = Vec::new();
    for result in trivy.Results.to_owned() {
        if result.Type == "npm" {
            for vul in result.Vulnerabilities {
                let paths = get_parent_dependencies(&vul.PkgName, app)?;
                vulnerabilities.push(Vulnerability {
                    name: vul.PkgName,
                    version: vul.InstalledVersion,
                    fixed_version: vul.FixedVersion,
                    paths: paths,
                    severity: vul.Severity,
                    published: vul.PublishedDate,
                    updated: vul.LastModifiedDate,
                    description: Some(vul.Description.to_owned()),
                    references: Vec::new(),
                })
            }
        }
    }
    Ok(DependencyReport {
        id: app.id.to_owned(),
        application_name: app.name.to_owned(),
        date: iso_8601(&now),
        application_id: None,
        vulnerabilities: vulnerabilities,
    })
}

/// Get Package lock version from package name
///    * Used to get the acutual version rather than the semantic version pattern
fn get_package_lock_version(dependency_name: &str) -> Option<String> {
    let package_lock = get_package_lock();
    if package_lock.dependencies.is_none() {
        return None;
    };
    let dependencies = package_lock.dependencies.to_owned().unwrap();
    let dep_option = dependencies.get(dependency_name);
    if dep_option.is_none() {
        return None;
    };
    return Some(dep_option.unwrap().version.to_owned());
}

fn get_parent_dependencies(
    dependency: &str,
    app: &Application,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut parents: HashSet<String> = HashSet::new();
    let mut dep_set: HashSet<String> = HashSet::new();
    for dep in app.internal_dependencies.to_owned() {
        dep_set.insert(dep.name.clone());
    }
    let package_lock = get_package_lock();
    if package_lock.dependencies.is_none() {
        return Ok(parents.into_iter().collect());
    }
    let all_deps = package_lock.dependencies.to_owned().unwrap();
    if all_deps.is_empty() {
        return Ok(parents.into_iter().collect());
    }
    //Get dependecies of Depency
    for (_, package_info) in all_deps {
        if package_info.requires.is_none() {
            continue;
        };
        for (dep, _) in package_info.requires.unwrap() {
            //log::debug!("dep: {}",&dep);
            if dep == dependency {
                //log::debug!("Found dep: {}, searching ....", &dep);
                let path = dfs_dependecies(&dep, &dep, &dep_set)?;
                log::debug!("Path: {}", &path);
                parents.insert(path);
            }
        }
    }

    return Ok(parents.into_iter().collect());
}

/// Depth first search for dependencies.  This funciton will search the depenedcy tree and return the path required to get to a root dependency.
/// # Arguments
///    * dependency: Leaf dependecy you wish to find.
///    * path: Path used to contain the recursive chain. To start a search this should be the same as dependency.
///    * dependencies: list of root dependecies.
fn dfs_dependecies(
    dependency: &str,
    path: &str,
    dependencies: &HashSet<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let package_lock = get_package_lock();
    // return path id there are no dependencies
    if package_lock.dependencies.is_none() {
        return Ok(path.to_string());
    }
    let all_deps = package_lock.dependencies.to_owned().unwrap();
    //return if deps are empty
    if all_deps.is_empty() {
        return Ok(path.to_string());
    }
    //found the root dependecy exit
    if dependencies.contains(dependency) {
        return Ok(format!("{}::{}", dependency, path));
    } else {
        for (root_name, deps) in all_deps {
            if deps.requires.is_none() {
                continue;
            }
            for (dep_name, _) in deps.requires.unwrap() {
                if dep_name == dependency.to_string() {
                    let new_path = format!("{}::{}", &root_name, path);
                    log::debug!("{}", &new_path);
                    return Ok(dfs_dependecies(&root_name, &new_path, dependencies)?);
                }
            }
        }
    }
    return Ok(path.to_string());
}

fn get_package_lock() -> PackageLock {
    return PACKAGE_LOCK.read().unwrap().clone();
}

///Reads package-lock.json file and returns the Package lock.
///   Note: ensure config is configured or default config will be used.
fn read_package_lock() -> PackageLock {
    let pl = PackageLock {
        dependencies: None,
        name: String::new(),
        version: String::new(),
    };
    let config = get_config();
    let mut package_lock_file: File;
    let file_open_result = File::open(format!("{}/package-lock.json", &config.base_dir));
    match file_open_result {
        Ok(file) => package_lock_file = file,
        Err(e) => {
            log::error!("Failed to find package lock file: {}", e);
            return pl;
        }
    }
    let mut package_lock_json = String::new();
    match package_lock_file.read_to_string(&mut package_lock_json) {
        Ok(_) => (),
        Err(e) => log::error!("failed to read data from package-lock.json: {}", e),
    }
    match serde_json::from_str(&package_lock_json) {
        Ok(pl_json) => return pl_json,
        Err(e) => {
            log::error!(
                "Failed to read package lock file. JSON binding failed: {}",
                e
            );
            return pl;
        }
    }
}

#[cfg(test)]
mod npm_mapper_tests {
    use super::*;
    use crate::models::config::Config;
    use crate::shared;
    use crate::utils::trivy_utils;
    use std::path::Path;

    fn set_up(){
        shared::set_up_logger(true);
        let config = Config {
            base_dir: String::from("./test-data/npm/"),
            shield_server: String::new(),
            shield_user: String::new(),
            shield_pass: String::new(),
        };
        shared::update_config(config);
    }

    #[test]
    fn test_meta_data() {
        set_up();
        match map_application()  {
            Ok(app) => {
                shared::write_json_file(Path::new("./test-data/npm/p-shield/app.json"), &app)
                    .unwrap()
            }
            Err(e) => assert!(false, "Failed to map application {}", e),
        }
    }

    #[test]
    fn test_find_root_dep() {
        set_up();
        let app = map_application().unwrap();
        let paths = get_parent_dependencies("ansi-html", &app).unwrap();
        let path_string = serde_json::to_string_pretty(&paths).unwrap();
        log::info!("result :\n{}", path_string);
        let paths = get_parent_dependencies("node-forge", &app).unwrap();
        let path_string = serde_json::to_string_pretty(&paths).unwrap();
        log::info!("result :\n{}", path_string)
    }

    #[test]
    fn test_get_dep_report() {
        set_up();
        let app = map_application().unwrap();
        let trivy = trivy_utils::run_fs_scan().unwrap();
        let dep_report = get_dependency_report(&trivy, &app).unwrap();
        let path_string = serde_json::to_string_pretty(&dep_report).unwrap();
        log::info!("result :\n{}", path_string)
    }
}
