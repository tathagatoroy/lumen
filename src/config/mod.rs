mod constants;
mod settings;

use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use settings::{Args, Settings};


pub struct Config {
    pub settings: Settings,
    pub args: Args,
}

impl Config {
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let settings = Settings::new()?;
        Ok(Self { settings, args })
    }
    /// returns settings and args
}


// re export the settings and args
pub use constants::*;
pub use settings::*;















