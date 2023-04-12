use std::io::Read;
use std::process::Command;
use std::fs;

use crate::models::application::Application;
use crate::models::config::Config;
use crate::models::dependecy_report::DependencyReport;
use crate::models::owasp::{OwsapReport, Dependency};

use super::shared::iso_8601;

pub const DEP_CHECK_SH: &str = "/home/shieldci/dependency-check/bin/dependency-check.sh";
pub const OUTPUT_FILE: &str = "/home/shieldci/dep-report.json";
pub const FINAL_OUTPUT_FILE: &str = "/home/shieldci/final-dep-report.json";

pub fn run_owasp_dep_check(config: &Config, scan: Option<&str>) -> Result<OwsapReport, Box<dyn std::error::Error>> {
    let scan_path : String;
    if scan.is_none() {
        scan_path = config.base_dir.to_owned();
    } else {
        scan_path = scan.unwrap().to_string();
    }
    let mut child = Command::new(DEP_CHECK_SH)
        .arg("-f=JSON")
        .arg(format!("-o={}", OUTPUT_FILE))
        .arg(format!("-s={}", scan_path))
        .spawn()?;
    child.wait()?;
    //Open file
    let mut result_file = fs::File::open(OUTPUT_FILE)?;
    let mut json_str = String::new();
    result_file.read_to_string(&mut json_str)?;
    //Filter out only problematci deps
    let owasp: OwsapReport = serde_json::from_str(&json_str)?;
    let mut filtered_deps : Vec<Dependency> = Vec::new();
    for depedency in owasp.dependencies {
        let copy = depedency.clone();
        if depedency.vulnerabilities.is_none() { continue; }
        let vulnerabiliies = depedency.vulnerabilities.unwrap();
        if vulnerabiliies.len() == 0 { continue; }
        filtered_deps.push(copy);
    }
    let final_owasp = OwsapReport {
        scan_info: owasp.scan_info,
        dependencies: filtered_deps
    };
    Ok(final_owasp)
}