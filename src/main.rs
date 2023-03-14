use clap::Parser;
use log;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::path;
use std::process::exit;

mod property_mapping;
use property_mapping::npm_mapper;

mod models;
use models::application;
use models::config::{Config, RESULT_DIR};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(default_value_t = String::from("./"))]
    path: String,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    set_up_logger(&args);
    let config: Config;
    log::info!("Phoenix🛡️  Processing ...");
    match read_config(&args) {
        Ok(val) => config = val,
        Err(e) => {
            log::error!("Failed to configure: {}", e);
            exit(1)
        }
    }
    let tech = application::detect_technologies(&config);
    if tech.npm {
        match npm_mapper::map_application(&config) {
            Ok(app) => {
                application::write_application(&app, path::Path::new(RESULT_DIR))
                    .expect("Failed to write app.json");
            }
            Err(e) => {
                log::error!("Failed to build Application definition: \n{}", e);
            }
        }
    }
}

fn read_config(args: &Args) -> Result<Config, Box<dyn std::error::Error>> {
    let path = path::Path::new(&args.path);
    if path.exists() {
        Ok(Config {
            base_dir: args.path.clone(),
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
