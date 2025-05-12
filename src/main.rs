mod editor;
mod ui;
mod input;
mod utils;
mod config;

use crate::config::{Config, Args};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    
    // Initialize editor components
    println!("Lumen Text Editor - Starting up...");
    
    // Parse command line arguments and create config
    /// parse the command line arguments and create a config defined in settings.rs
    let args = Args::parse();
    let config = Config::new(args)?;
    
    // Log configuration information
    log::info!("Config: {:?}", config);
    log::info!("Args: {:?}", args);
    
    // Print to stdout for user feedback
    println!("Configuration loaded successfully");
    if let Some(file) = config.file_to_open() {
        println!("Opening file: {:?}", file);
    }
    
    Ok(())
} 
