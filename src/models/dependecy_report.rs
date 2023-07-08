use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct DependencyReport {
    pub id: Option<String>,
    pub application_name: String,
    pub application_id: Option<String>,
    pub project: String,
    pub vulnerabilities: Vec<Vulnerability>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Vulnerability {
    pub name: String,
    pub version: String,
    pub fixed_version: Option<String>,
    pub paths: Vec<String>,
    pub severity: String,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    pub references: Vec<String>
}

