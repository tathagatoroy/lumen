#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod utils;


use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::config::constants::*;
use crate::utils::error::editorError;
use crate::utils::file_ops::readFile;

/// Editor modes
/// These define the different states the editor can be in
#[derive(Debug, Clone, Copy, PartialEq)]
/// partial eq is used to compare two instances of the same enum allows != and == to be used
/// copy makes deepcopy and clone faster and not change the original value
pub enum EditorMode {
    Insert,  // Text insertion mode
    Command, // Command line mode (like Vim's : mode)
}

// represents each row of the editor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    /// The characters in the row
    pub chars: Vec<char>,
    /// The rendered string of the row
    pub render: String,
    /// The index of the row
    pub idx: u32,
    /// The size of the row
    pub rsize : u32,
    /// The size of the row without the null terminator
    pub size: u32,
}
impl Default for Row {
    fn default() -> Self {
        Self {
            chars: Vec::new(),
            render: String::new(),
            idx: 0,
            rsize: 0,
            size: 0,
        }
    }
}

/// Represents the different modes the editor can be in
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EditorMode {
    /// Insert mode for text input
    Insert,
    /// Command mode for executing commands
    Command,
    /// Visual mode for text selection
    Visual,
}

/// Represents the core structure of the editor, containing all state information
/// and configuration for the current editing session.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorStructure {
    /// Path to the currently open file
    pub filePath: PathBuf,
    
    /// Current vertical scroll position (number of rows scrolled)
    pub rowOffset: u32,
    
    /// Current horizontal scroll position (number of columns scrolled)
    pub colOffset: u32,
    
    /// Number of rows that can be displayed in the terminal window
    pub screenRows: u32,
    
    /// Number of columns that can be displayed in the terminal window
    pub screenCols: u32,
    
    /// Total number of rows in the current file
    pub numRows: u32,
    
    /// Total number of columns in the current file
    pub numCols: u32,
    
    /// Flag indicating if the file has been modified since last save
    pub dirty: bool,
    
    /// Number of spaces to use for indentation
    pub indentWidth: u32,
    
    /// Number of spaces to use for tab characters
    pub tabWidth: u32,
    
    /// Current status message to display in the status bar
    pub statusMessage: String,

    /// Current mode of the editor
    pub mode: EditorMode,
}

// defaults for the editor structure
impl Default for EditorStructure {
    fn default() -> Self {
        Self {
            filePath: PathBuf::new(),
            rowOffset: 0,
            colOffset: 0,
            screenRows: DEFAULT_WINDOW_WIDTH as u32,
            screenCols: DEFAULT_WINDOW_HEIGHT as u32,
            numRows: 0,
            numCols: 0,
            dirty: false,
            indentWidth: DEFAULT_TAB_WIDTH as u32,
            tabWidth: DEFAULT_TAB_WIDTH as u32,
            statusMessage: String::new(),
            mode: EditorMode::Insert
        }
    }
}

// methods for the editor structure
impl EditorStructure {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn openFile(&mut self, path: PathBuf) -> Result<(), editorError> {
        self.filePath = path;
        self.dirty = false;
        Ok(())
    }

    pub fn initialiseEditor(&mut self) -> Result<(), editorError>{
        let openedFile = readFile(self.path)?;
        let reader = BufReader::new(openedFile);
        for line in reader.lines(){

            
        }


    }

    /// Change the editor mode
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    /// Check if the editor is in a specific mode
    pub fn is_mode(&self, mode: EditorMode) -> bool {
        self.mode == mode
    }
}
