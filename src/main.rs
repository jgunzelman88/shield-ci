use clap::Parser;
use std::path::Path;
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod models;
use models::config::Config;

mod property_mapping;
use property_mapping::{project_detection, npm_mapper};


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
    let config = read_config(&args).expect("CONFIG FILE NOT FOUND");
    let tech = project_detection::detect_technologies(&config);
    if tech.npm {
        npm_mapper::map_application(&config);
    }
    
}

fn read_config(args: &Args) -> Result<Config, Box<dyn std::error::Error>>  {    
    let path = Path::new(&args.path);
    if path.exists() {
        Ok(Config { base_dir: args.path.clone()})
    }else{
        Err(Box::from("Config File Not Found"))
    }
}

fn set_up_logger(args: &Args){
    let mut level = LevelFilter::Info;
    if args.verbose { level = LevelFilter::Debug;}
    SimpleLogger::new().with_level(level).init().unwrap();
}