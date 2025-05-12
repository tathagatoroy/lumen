#![allow(unused_imports)]
#![allow(unused_variables)]

use std::path::PathBuf;
use clap::{Subcommand};
use serde::{Deserialize, Serialize};
use crate::config::constants::*;
use clap::Parser;

/// Command line arguments for the editor
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Optional file to open
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Log level
    #[arg(short, long, default_value = "info")]
    pub log_level: LogLevel,

    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available editor commands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Open a new file
    Open { path: PathBuf },
    /// Save current file
    Save { path: Option<PathBuf> },
    /// Quit the editor
    Quit,
}

/// Editor settings that can be configured at runtime
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    /// Tab width in spaces
    pub tab_width: usize,
    /// Whether to use spaces instead of tabs
    pub use_spaces: bool,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether to highlight current line
    pub highlight_current_line: bool,
    /// Custom key bindings
    pub key_bindings: Vec<(String, String)>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tab_width: DEFAULT_TAB_WIDTH,
            use_spaces: DEFAULT_USE_SPACES,
            show_line_numbers: DEFAULT_SHOW_LINE_NUMBERS,
            highlight_current_line: DEFAULT_HIGHLIGHT_CURRENT_LINE,
            key_bindings: DEFAULT_KEYBINDINGS.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }
}

impl Settings {
    /// Load settings from a configuration file
    /// ? means that the function can return an error
    /// else it returns the settings
    /// Ok means that the function returns a value
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&contents)?;
        Ok(settings)
    }

    /// Save settings to a configuration file
    /// ? means that the function can return an error
    /// Box<dyn std::error::Error> means that the function can return any error that implements the std::error::Error trait
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_tab_width(&self) -> usize {
        self.tab_width
    }
    pub fn get_use_spaces(&self) -> bool {
        self.use_spaces
    }
    pub fn get_show_line_numbers(&self) -> bool {
        self.show_line_numbers
    }
    pub fn get_highlight_current_line(&self) -> bool {
        self.highlight_current_line
    }
    pub fn get_key_bindings(&self) -> &Vec<(String, String)> {
        &self.key_bindings
    }

    fn set_tab_width(&mut self, tab_width: usize) {
        self.tab_width = tab_width;
    }
    fn set_use_spaces(&mut self, use_spaces: bool) {
        self.use_spaces = use_spaces;
    }   
    fn set_show_line_numbers(&mut self, show_line_numbers: bool) {
        self.show_line_numbers = show_line_numbers;
    }
    fn set_highlight_current_line(&mut self, highlight_current_line: bool) {
        self.highlight_current_line = highlight_current_line;
    }
    fn set_key_bindings(&mut self, key_bindings: Vec<(String, String)>) {
        self.key_bindings = key_bindings;
    }
} 