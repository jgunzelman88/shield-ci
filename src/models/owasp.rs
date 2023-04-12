use serde::{Serialize, Deserialize};
use std::io::{Write};
use std::fs;
use std::path;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct OwsapReport {
    pub scan_info: ScanInfo,
    pub dependencies: Vec<Dependency>
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScanInfo {
    pub engine_version: String
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub file_name: String,
    pub description: Option<String>,
    pub vulnerability_ids: Option<Vec<VulnerabilityId>>,
    pub vulnerabilities: Option<Vec<Vulnerability>>
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct VulnerabilityId {
    pub id: String,
    pub url: String,
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Vulnerability {
    pub source: String,
    pub name: String,
    pub severity: String,
    pub description: Option<String>,
    pub references: Vec<Reference>
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Reference {
    pub source: String,
    pub url: String,
    pub name: String
}

pub fn write_dep_check(owsap: &OwsapReport, path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(owsap)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}