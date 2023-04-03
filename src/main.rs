use clap::Parser;
use log;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::path;
use std::process::exit;
use tokio;

mod property_mapping;
use property_mapping::npm_mapper;

mod models;
use models::application;
use models::config::{Config, RESULT_DIR};

mod utils;
use utils::pocketbase;

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
    set_up_logger(&args);
    let config: Config;
    log::info!("🛡️ Shield CI Processing ...");
    match read_config(&args) {
        Ok(val) => config = val,
        Err(e) => {
            log::error!("Failed to configure: {}", e);
            exit(1)
        }
    }
    let tech = application::detect_technologies(&config);
    if tech.npm {
        let app = npm_mapper::map_application(&config)?;
        application::write_application(&app, path::Path::new(RESULT_DIR))
            .expect("Failed to write app.json");
        if config.pb_server != "" {
            pocketbase::submit_results(&app, &config).await?;
        }
    }else {
        log::info!("No compatable technology found!")
    }
    Ok(())
}

fn read_config(args: &Args) -> Result<Config, Box<dyn std::error::Error>> {
    let path = path::Path::new(&args.path);
    if path.exists() {
        Ok(Config {
            base_dir: args.path.clone(),
            pb_server: args.shield_url.clone(),
            pb_user: args.shield_user.clone(),
            pb_pass: args.shield_pass.clone(),
        })
    } else {
        Err(Box::from("Path Provided does not exsist"))
    }
}

fn set_up_logger(args: &Args) {
    let mut level = LevelFilter::Info;
    if args.verbose {
        level = LevelFilter::Debug;
    }
    SimpleLogger::new().env().with_level(level).init().unwrap();
}
