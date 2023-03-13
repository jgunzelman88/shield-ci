use serde::{Serialize, Deserialize};

pub const RESULT_DIR: &str = "./p-shield";
/// Config File Mapping
/// # Properties
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Config{
    pub base_dir: String
}