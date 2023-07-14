use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct DependencyReport {
    pub id: Option<String>,
    pub application_name: String,
    pub application_id: Option<String>,
    pub project: String,
    pub branch: Option<String>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub image_reports: Vec<ImageReport>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Vulnerability {
    pub name: String,
    pub version: String,
    pub fixed_version: Option<String>,
    pub paths: Vec<String>,
    pub top_level_dependency: Option<String>,
    pub severity: String,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    pub references: Vec<String>
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct ImageReport {
  pub dockerfile: String,
  pub vulnerabilities: Vec<Vulnerability>
}