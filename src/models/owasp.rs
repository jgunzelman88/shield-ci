use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwsapReport {
    scan_info: ScanInfo
}`
#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanInfo {
    engine_version: String
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    file_name: String,
    description: String,
    vulnerability_ids: Vec<VulnerabilityId>,
}
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct VulnerabilityId {
    id: String,
    url: String,
}
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Vulnerability {
    source: String,
    name: String,
    severity: String,
    description: String,
    references: Vec<Reference>
}
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Reference {
    source: String,
    url: String,
    name: String
}