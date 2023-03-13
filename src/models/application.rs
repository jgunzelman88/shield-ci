use super::property_mapping::PropertyMapping;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Application {
    pub name: String,
    pub parent: Option<String>,
    pub subcomponents: Option<Vec::<String>>,
    pub internal_dependencies: Vec::<Dependency>,
    pub external_dependencies: Vec::<Dependency>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub port: Option<String>,
    pub protocol: Option<String>,
    pub property_mappings: Option<Vec::<PropertyMapping>>
}
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Technologies {
    pub npm: bool,
    pub pip: bool,
    pub cargo: bool,
    pub docker: bool,
}