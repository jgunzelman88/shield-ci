use crate::models::trivy::TrivyReport;
use std::process::Command;

use super::shared::get_config;

pub fn run_fs_scan() -> Result<TrivyReport, Box<dyn std::error::Error>> {
    let config = get_config();
    log::debug!("scaning dir: {}", config.base_dir.clone());
    let output = Command::new("trivy")
        .arg("fs")
        .args(["--format","json"])
        //.args(["--scanners","vuln,config"])
        .arg(config.base_dir.clone())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    log::debug!("trivy output: {}",stdout);
    log::debug!("trivy err: {}",String::from_utf8(output.stderr)?);
    let trivy: TrivyReport = serde_json::from_str(&stdout)?;
    Ok(trivy)
}

#[cfg(test)]
mod trivy_tests {
    use super::*;
    use crate::models::config::Config;
    use crate::shared;

    fn set_up(){
        shared::set_up_logger(true);
        let config = Config {
            base_dir: String::from("./test-data/npm/"),
            project_id: String::from("12345"),
            shield_server: String::new(),
            shield_user: String::new(),
            shield_pass: String::new(),
        };
        shared::update_config(config);
    }

    #[test]
    fn run_scan() {
        set_up();
        let trivy: TrivyReport;
        match run_fs_scan() {
            Ok(scan) => trivy = scan,
            Err(e) => {log::error!("{}",e); return},
        }
        let trivy_string = serde_json::to_string_pretty(&trivy).unwrap();
        log::info!("result :\n{}", trivy_string);
    }
}
