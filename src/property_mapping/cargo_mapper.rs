use std::path::Path;

use crate::models::config::Config;

pub fn map_application(config: &Config)  -> Result<Application, Box<dyn std::error::Error>> {
    let cargo_file_name = format!("{}/Cargo.toml", config.base_dir);
    let cargo_file = Path::new(cargo_file_name);

    
    Ok(Application {
        
    })
}