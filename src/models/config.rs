use serde::{Serialize, Deserialize};

pub const RESULT_DIR: &str = "./shield-ci";
/// Config File Mapping
/// # Properties
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Config{
    pub base_dir: String,
    pub pb_server: String,
    pub pb_user: String,
    pub pb_pass: String
}