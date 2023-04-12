use crate::models::application::{read_applicaiton, Application, Dependency, SubDependency};
use crate::models::config::Config;
use crate::models::dependecy_report::{DependencyReport, Vulnerability};
use crate::models::property_mapping;
use crate::utils::owasp_dep_check;
use crate::utils::shared;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

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

/// Maps the application data from npm.
pub fn map_application(config: &Config) -> Result<Application, Box<dyn std::error::Error>> {
    log::debug!("Reading npm package lock file");
    // Process npm file
    let mut package_lock_file = File::open(format!("{}/package-lock.json", config.base_dir))?;
    let mut package_lock_json = String::new();
    package_lock_file.read_to_string(&mut package_lock_json)?;
    let package_lock: PackageLock = serde_json::from_str(&package_lock_json)?;
    log::debug!("Reading npm package file");
    let mut package_file = File::open(format!("{}/package.json", config.base_dir))?;
    let mut package_json = String::new();
    package_file.read_to_string(&mut package_json)?;
    let package: NpmPackage = serde_json::from_str(&package_json)?;
    log::debug!("Processing Internal Dependencies ");
    // Set Dependencies
    let mut inter_deps: Vec<Dependency> = Vec::new();
    for (name, version) in package.dependencies {
        let version = version;
        inter_deps.push(Dependency {
            name: name,
            version: version,
            port: None,
            property_mappings: None,
            protocol: None,
        });
    }
    log::debug!("Looking for exsisting app file");
    let current_app = read_applicaiton(config);
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

pub fn run_vulnerability_scan(config: &Config, app: &Application, package_lock: &PackageLock) -> Result<DependencyReport, Box<dyn std::error::Error>> {
    let path = format!("{}/package-lock.json", config.base_dir);
    let owasp = owasp_dep_check::run_owasp_dep_check(config, Some(&path))?;
    let now = std::time::SystemTime::now();
    let mut vulnerabilities: Vec<Vulnerability> = Vec::new();
    let sub_deps = get_sub_dependencies(&app.internal_dependencies, package_lock);
    for dep in owasp.dependencies {
        if dep.vulnerabilities.is_none() {continue;}
        for vul in dep.vulnerabilities.unwrap() {
            let sub_dep = sub_deps.get(&vul.name);
            if sub_dep.is_some() {
                let sub_dep = sub_dep.unwrap();
                let issue = Vulnerability {
                    name: vul.name.to_string(),
                    path: Some(sub_dep.path.to_string()),
                    severity: vul.severity.to_string(),
                    description: vul.description.to_owned(),
                    references: vul.references.to_owned(),
                };
                vulnerabilities.push(issue)
            }
        }
    }
    Ok( DependencyReport {
        id: None,
        application_id: app.id.clone(),
        application_name: app.name.clone(),
        date: shared::iso_8601(&now),
        vulnerabilities: vulnerabilities
    })
}

fn get_sub_dependencies(
    dependecies: &Vec<Dependency>,
    package_lock: &PackageLock,
) -> HashMap<String, SubDependency> {
    let mut sub_dep_map: HashMap<String, SubDependency> = HashMap::new();
    for dependency in dependecies {
        sub_dependencies_recursive_search(
            &dependency.name,
            &dependency.name,
            &mut sub_dep_map,
            package_lock,
        );
    }
    return sub_dep_map;
}

fn sub_dependencies_recursive_search(
    dependency: &str,
    path: &str,
    dep_list: &mut HashMap<String, SubDependency>,
    package_lock: &PackageLock,
) {
    if package_lock.dependencies.is_none() {
        return;
    }
    let all_deps = package_lock.dependencies.to_owned().unwrap();
    if all_deps.is_empty() {
        return;
    }

    //Get dependecies of Depency
    let package = &all_deps.get(dependency);
    if package.is_none() {
        return;
    }
    let package_deps = package.unwrap().to_owned();

    if package_deps.dependencies.is_some() {
        let deps_of_dep = package_deps.dependencies.unwrap();

        //Interate though the dependecies of the dependency
        for (name, dep) in deps_of_dep {
            let new_path = format!("{}::{}", path, name);
            log::debug!("{}", new_path);
            let version: String;
            match dep {
                DependencyType::Obj(obj) => version = obj.version,
                DependencyType::String(str) => version = str,
            }
            let sub_dependency = SubDependency {
                version: version.to_owned(),
                path: path.to_owned(),
            };
            dep_list.insert(name.clone(), sub_dependency);
            sub_dependencies_recursive_search(&name, &new_path, dep_list, package_lock)
        }
    }

    if package_deps.requires.is_some() {
        let reqs_of_dep = package_deps.requires.unwrap();
        //Interate though the requiers of the dependency
        for (name, _) in reqs_of_dep {
            let new_path = format!("{}::{}", path, name);
            let package = &all_deps.get(&name);
            log::debug!("{}", &new_path);
            if package.is_none() {
                log::warn!("failed to find {} in root package lock", name);
                continue;
            }
            let version = package.unwrap().version.to_owned();
            let sub_dependency = SubDependency {
                version: version.to_owned(),
                path: path.to_owned(),
            };
            dep_list.insert(name.clone(), sub_dependency);
            sub_dependencies_recursive_search(&name, &new_path, dep_list, package_lock)
        }
    }
}

#[cfg(test)]
mod npm_mapper_tests {
    use super::map_application;
    use crate::models::config::Config;
    use crate::shared;
    use std::path::Path;

    #[test]
    fn test_meta_data() {
        shared::set_up_logger(true);
        let config = Config {
            base_dir: String::from("./test-data/npm/"),
            pb_server: String::new(),
            pb_user: String::new(),
            pb_pass: String::new(),
        };
        match map_application(&config) {
            Ok(app) => {
                shared::write_json_file(Path::new("./test-data/npm/p-shield/app.json"), &app)
                    .unwrap()
            }
            Err(e) => assert!(false, "Failed to map application {}", e),
        }
    }
}
