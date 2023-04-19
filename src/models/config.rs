use serde::{Serialize, Deserialize};

pub const RESULT_DIR: &str = "./shield-ci";
/// Config File Mapping
/// # Properties
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Config{
    pub base_dir: String,
    pub shield_server: String,
    pub shield_user: String,
    pub shield_pass: String
}