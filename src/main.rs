#![allow(unused_imports)]
#![allow(unused_variables)]

mod editor;
mod ui;
mod input;
mod utils;
mod config;

use crate::config::{Config};
use crate::config::settings::{Args};
use std::error::Error;
use utils::file_ops::read_file;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    
    // Initialize editor components
    println!("Lumen Text Editor - Starting up...");
    
    // Parse command line arguments and create config
    // parse the command line arguments and create a config defined in settings.rs
    let args = Args::parse();
    let config = Config::new(args)?;
    let settings = config.settings.clone();
    // get the args from the config as the old args was moved and is now in the config
    let args = config.args.clone();
    // print the settings and args
    println!("Settings: {:?}", settings);
    println!("Args: {:?}", args);

    

    
    // Print to stdout for user feedback
    println!("Configuration loaded successfully");
    if let Some(file) = config.file_to_open() {
        println!("Opening file: {:?}", file);
        // open the file
        let file = read_file(&file)?;
    }
    Ok(())
} 
