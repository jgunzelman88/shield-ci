use std::process::Command;

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

pub fn get_branch() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}