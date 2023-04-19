use clap::Parser;
use log;
use std::path;
use std::process::exit;
use tokio;

mod property_mapping;
use property_mapping::npm_mapper;

mod models;
use models::application;
use models::config::{Config, RESULT_DIR};


mod utils;
use utils::shield;
use utils::shared;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = String::from("./"))]
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let verbose = args.verbose;
    shared::set_up_logger(verbose);
    let config: Config;
    log::info!("🛡️ Shield CI Processing ...");
    match read_config(&args) {
        Ok(val) => {config = val.clone(); shared::update_config(val)},
        Err(e) => {
            log::error!("Failed to configure: {}", e);
            exit(1)
        }
    }
    let tech = application::detect_technologies();
    if tech.npm {
        let app = npm_mapper::map_application()?;
        let app_path_name = format!("{}/{}/app.json", config.base_dir, RESULT_DIR);
        shared::write_json_file(path::Path::new(&app_path_name), &app)?;
        if config.shield_server != "" {
            shield::submit_results(&app, &config).await?;
        }
    }else {
        log::info!("No compatable technology found!")
    }
    log::info!("Finished!!");
    Ok(())
}

fn read_config(args: &Args) -> Result<Config, Box<dyn std::error::Error>> {
    let path = path::Path::new(&args.path);
    if path.exists() {
        Ok(Config {
            base_dir: args.path.clone(),
            shield_server: args.shield_url.clone(),
            shield_user: args.shield_user.clone(),
            shield_pass: args.shield_pass.clone(),
        })
    } else {
        Err(Box::from("Path Provided does not exsist"))
    }
}