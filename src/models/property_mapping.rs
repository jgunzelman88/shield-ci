use super::application::Dependency;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use jsonpath;
use log;
#[derive(Serialize, Deserialize, Clone)]
pub struct PropertyMapping {
    pub key: String,
    pub file: String,
    pub mapping_path: String,
}

enum FileFormat {
    Json,
    Yaml,
}

/// Process and replace a dependency with property values
pub fn process_dependency(dependency: Dependency) -> Dependency {
    if dependency.property_mappings.is_none() {
        return dependency.clone();
    };
    let mut name = dependency.name;
    let mut version = dependency.version;
    let mut port = dependency.port;
    let mut protocol = dependency.protocol;

    let props = dependency.property_mappings.clone().unwrap();
    //Map Properties
    for prop in props {
        match prop.key.as_str() {
            "name" => {
                let result = read_value_from_file(&prop.file, &prop.mapping_path);
                match result {
                    Ok(name_val) => name = name_val,
                    Err(e) => log::warn!("Failed to map name on prop mapping: {}", e)
                }
            }
            "version" => {
                let result = read_value_from_file(&prop.file, &prop.mapping_path);
                match result {
                    Ok(version_val) => version = version_val,
                    Err(e) => log::warn!("Failed to map version on prop mapping: {}", e)
                }
            }
            "port" => {
                let result =read_value_from_file(&prop.file, &prop.mapping_path);
                match result {
                    Ok(port_val) => port = Some(port_val),
                    Err(e) => log::warn!("Failed to map port on prop mapping: {}", e)
                }
            }
            "protocol" => {
                let result =read_value_from_file(&prop.file, &prop.mapping_path);
                match result {
                    Ok(protcol_val) => protocol = Some(protcol_val),
                    Err(e) => log::warn!("Failed to map protocol on prop mapping: {}", e)
                }
            }
            _ => {
                continue;
            }
        }
    }
    return Dependency {
        name: name,
        version: version,
        port: port,
        protocol: protocol,
        property_mappings: dependency.property_mappings,
    };
}

/// Read a value from a json or yaml file.
/// # Arguments
///    * file_path: Path and name of the file ex: ./deploy/chart.yml
///    * json_path: JSON path to the value you want to retrieve. 
fn read_value_from_file(file_path: &str, json_path: &str) -> Result<String, Box::<dyn std::error::Error>> {
    // Determine the file format based on the file extension
    let file_format = match file_path.split('.').last() {
        Some("json") => FileFormat::Json,
        Some("yaml") | Some("yml") => FileFormat::Yaml,
        _ => return Err(Box::from("Invalid file extension")),
    };

    // Read the file contents into a string
    let file_contents =
        fs::read_to_string(file_path).map_err(|_| String::from("Failed to read file"))?;

    // Parse the file contents into a JSON or YAML value
    let value = match file_format {
        FileFormat::Json => {
            let json_result = serde_json::from_str::<JsonValue>(&file_contents);
            match json_result {
                Ok(val) =>  val,
                Err(_) => return Err(Box::from("Failed to Read json file"))
            }
        },
        FileFormat::Yaml => {
            let yaml_result = serde_yaml::from_str::<YamlValue>(&file_contents);
            match yaml_result {
                Ok(val) => yaml_to_json(&val),
                Err(_) => return Err(Box::from("Failed to parse YAML "))
            }
        }?,
    };

    // Traverse the value using the JSON path
    let selector = jsonpath::Selector::new(json_path)?;
    let values : Vec<String> = selector.find(&value).map(
        |val| {
            if val.is_string() {
                val.as_str().unwrap().to_string()
            }else if val.is_boolean() {
                val.as_bool().unwrap().to_string()
             }else if val.is_f64() {
                val.as_f64().unwrap().to_string()
             }else if val.is_i64() {
                val.as_i64().unwrap().to_string()
             }else if val.is_u64() {
                val.as_u64().unwrap().to_string()
             }else {
                String::new()
             }
        }
    ).collect();
    if values.len() == 0 {
        Err(Box::from("Failed to Find value"))
    }else {
        Ok(values[0].clone())
    }
}

/// Convert Yaml value to Json value
fn yaml_to_json(yaml_value: &YamlValue) -> Result<JsonValue, String> {
    serde_json::to_value(yaml_value)
        .map_err(|err| format!("Failed to convert YAML to JSON: {}", err))
}
