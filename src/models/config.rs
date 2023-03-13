use serde::{Serialize, Deserialize};

/// Config File Mapping
/// # Properties
///    * build_tool : Build tool used, valid values below
///       * cargo
///       * npm
///       * pip
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Config{
    pub base_dir: String
}