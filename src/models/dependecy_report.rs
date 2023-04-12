use serde::{Serialize, Deserialize};
use super::owasp::Reference;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct DependencyReport {
    pub id: Option<String>,
    pub application_name: String,
    pub date: String,
    pub application_id: Option<String>,
    pub vulnerabilities: Vec<Vulnerability>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Vulnerability {
    pub name: String,
    pub path: Option<String>,
    pub severity: String,
    pub description: Option<String>,
    pub references: Vec<Reference>
}