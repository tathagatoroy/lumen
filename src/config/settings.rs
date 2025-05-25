#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::PathBuf;
use clap::{Subcommand};
use serde::{Deserialize, Serialize};
use crate::{config::constants::*, utils::error::editorError};
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
    pub logLevel: LogLevel,

    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// bol to indicate if debug mode is enabled
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
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
    pub tabWidth: u32,
    /// Whether to use spaces instead of tabs
    pub useSpaces: bool,
    /// Whether to show line numbers
    pub showLineNumbers: bool,
    /// Whether to highlight current line
    pub highlightCurrentLine: bool,
    /// Custom key bindings
    pub keyBindings: Vec<(String, String)>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tabWidth: DEFAULT_TAB_WIDTH,
            useSpaces: DEFAULT_USE_SPACES,
            showLineNumbers: DEFAULT_SHOW_LINE_NUMBERS,
            highlightCurrentLine: DEFAULT_HIGHLIGHT_CURRENT_LINE,
            keyBindings: DEFAULT_KEYBINDINGS.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }
}

impl Settings {
    /// Load settings from a configuration file
    /// ? means that the function can return an error
    /// else it returns the settings
    /// Ok means that the function returns a value
    pub fn loadFromFile(path: &PathBuf) -> Result<Self, editorError> {
        let contents = std::fs::read_to_string(path).map_err(editorError::IOError)?;
        let settings: Settings = toml::from_str(&contents).map_err(editorError::TomlDeserializeError)?;
        Ok(settings)
    }

    /// Save settings to a configuration file
    /// ? means that the function can return an error
    /// Box<dyn std::error::Error> means that the function can return any error that implements the std::error::Error trait
    pub fn saveToFile(&self, path: &PathBuf) -> Result<(), editorError> {
        let contents = toml::to_string_pretty(self).map_err(editorError::TomlError)?;
        std::fs::write(path, contents).map_err(editorError::IOError)?;
        Ok(())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn getTabWidth(&self) -> usize {
        self.tabWidth
    }
    pub fn getUseSpaces(&self) -> bool {
        self.useSpaces
    }
    pub fn getShowLineNumbers(&self) -> bool {
        self.showLineNumbers
    }
    pub fn getHighlightCurrentLine(&self) -> bool {
        self.highlightCurrentLine
    }
    pub fn getKeyBindings(&self) -> &Vec<(String, String)> {
        &self.keyBindings
    }

    fn setTabWidth(&mut self, tabWidth: usize) {
        self.tabWidth = tabWidth;
    }
    fn setUseSpaces(&mut self, useSpaces: bool) {
        self.useSpaces = useSpaces;
    }   
    fn setShowLineNumbers(&mut self, showLineNumbers: bool) {
        self.showLineNumbers = showLineNumbers;
    }
    fn setHighlightCurrentLine(&mut self, highlightCurrentLine: bool) {
        self.highlightCurrentLine = highlightCurrentLine;
    }
    fn setKeyBindings(&mut self, keyBindings: Vec<(String, String)>) {
        self.keyBindings = keyBindings;
    }
} 