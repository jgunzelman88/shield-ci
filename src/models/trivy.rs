use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageConfig {
    pub architecture: String,
    pub created: String,
    pub os: String,
    pub rootfs: Rootfs,
    pub config: Config,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rootfs {
    #[serde(rename = "type")]
    pub fs_type: String,
    pub diff_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Metadata {
    pub ImageConfig: ImageConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CVSSDetails {
    pub V2Vector: Option<String>,
    pub V3Vector: String,
    pub V2Score: Option<f32>,
    pub V3Score: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DataSource {
    pub ID: String,
    pub Name: String,
    pub URL: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Vulnerability {
    pub VulnerabilityID: String,
    pub PkgID: String,
    pub PkgName: String,
    pub InstalledVersion: String,
    pub FixedVersion: String,
    pub SeveritySource: String,
    pub PrimaryURL: String,
    pub DataSource: DataSource,
    pub Title: String,
    pub Description: String,
    pub Severity: String,
    pub References: Vec<String>,
    pub PublishedDate: Option<String>,
    pub LastModifiedDate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Result {
    pub Target: String,
    pub Class: String,
    pub Type: String,
    pub Vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TrivyReport {
    pub SchemaVersion: u32,
    pub ArtifactName: String,
    pub ArtifactType: String,
    pub Metadata: Metadata,
    pub Results: Vec<Result>,
}
