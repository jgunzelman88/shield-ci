use clap::Parser;
use log;
use std::path;
use std::process::exit;
use tokio;

mod property_mapping;
use property_mapping::npm_mapper;

mod models;
use models::application;
use models::config::Config;

mod utils;
use utils::shared;
use utils::trivy_utils;

use crate::models::trivy::TrivyReport;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = String::from(""))]
    project_id: String,
    #[arg(long, default_value_t = String::from("."))]
    path: String,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[arg(long, default_value_t = String::from(""))]
    shield_url: String,
    #[arg(long, default_value_t = String::from(""))]
    shield_user: String,
    #[arg(long, default_value_t = String::from(""))]
    shield_pass: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let verbose = args.verbose;
    shared::set_up_logger(verbose);
    let config: Config;
    log::info!("🛡️ Shield CI Processing ...");
    match init_config(&args) {
        Ok(val) => {
            config = val.clone();
            shared::update_config(val)
        }
        Err(e) => {
            log::error!("Failed to configure: {}", e);
            exit(1)
        }
    }
    let tech = application::detect_technologies();
    let trivy: TrivyReport;
    match trivy_utils::run_fs_scan() {
        Ok(rpt) => trivy = rpt,
        Err(e) => {
            log::error!("Trivy failed! {}", e);
            exit(1);
        }
    }
    if tech.npm {
        npm_mapper::process_npm(&config, &trivy).await;
    } else {
        log::info!("No compatable technology found!")
    }
    log::info!("Finished!!");
}

fn init_config(args: &Args) -> Result<Config, Box<dyn std::error::Error>> {
    // Base Dir handling
    let base_dir: String;
    if args.path == "./" {
        let base_path = std::env::var_os("SHIELD_CI_SCAN_DIR");
        if base_path.is_some() {
            base_dir = String::from(base_path.unwrap().to_string_lossy());
        } else {
            base_dir = String::from(&args.path);
        }
    } else {
        base_dir = String::from(&args.path);
    }
    // Project id
    let mut project_id = String::from(&args.project_id);
    if args.project_id == "" {
        let project_id_env = std::env::var_os("PROJECT_ID");
        if project_id_env.is_some() {
            project_id = String::from(project_id_env.unwrap().to_string_lossy());
        }
    }
    // Shield URL
    let mut shield_url = String::from(&args.shield_url);
    if args.shield_url == "" {
        let shield_url_env = std::env::var_os("SHIELD_URL");
        if shield_url_env.is_some() {
            shield_url = String::from(shield_url_env.unwrap().to_string_lossy());
        }
    }
    // Shield User
    let mut shield_user = String::from(&args.shield_user);
    if args.shield_user == "" {
        let shield_user_env = std::env::var_os("SHIELD_USER");
        if shield_user_env.is_some() {
            shield_user = String::from(shield_user_env.unwrap().to_string_lossy());
        }
    }
    // Shield Password
    let mut shield_pass = String::from(&args.shield_pass);
    if args.shield_pass == "" {
        let shield_pass_env = std::env::var_os("SHIELD_PASS");
        if shield_pass_env.is_some() {
            shield_pass = String::from(shield_pass_env.unwrap().to_string_lossy());
        }
    }
    // If path does not exsist throw error
    let path = path::Path::new(&base_dir);

    if path.exists() {
        let config = Config {
            base_dir: base_dir,
            project_id: project_id.clone(),
            shield_server: shield_url.clone(),
            shield_user: shield_user.clone(),
            shield_pass: shield_pass.clone(),
        };
        let redact: String;
        if shield_pass != "" {
            redact = String::from("******");
        } else {
            redact = String::from("NOT SET!!!")
        }
        log::debug!(
            "Config:\n
            project_id: {},
            shield_URL: {},
            user: {},
            pass: {}",
            project_id,
            shield_url,
            shield_user,
            redact
        );
        Ok(config)
    } else {
        Err(Box::from("Path Provided does not exsist"))
    }
}
