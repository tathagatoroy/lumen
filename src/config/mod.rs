mod constants;
pub mod settings;

use std::path::PathBuf;
use std::error::Error;
use crate::config::settings::{Args, Settings};

#[derive(Debug, Clone)]
pub struct Config {
    pub settings: Settings,
    pub args: Args,
}

impl Config {
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let settings = Settings::new();
        Ok(Self { settings, args })
    }
    // add default file to open
    pub fn file_to_open(&self) -> Option<PathBuf> {
        self.args.file.clone()

    }
}


















