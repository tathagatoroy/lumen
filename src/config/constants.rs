#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use clap::ValueEnum;

// Constants used throughout the editor
// These are build-time constants that define the editor's basic behavior

/// Default window size if terminal size cannot be determined
pub const DEFAULT_WINDOW_WIDTH: usize = 80;
pub const DEFAULT_WINDOW_HEIGHT: usize = 24;

/// Maximum number of undo operations to store
/// This is a memory vs functionality tradeoff
pub const MAX_UNDO_STEPS: usize = 1000;

/// Default tab width in spaces
/// This is a common default in many editors
pub const DEFAULT_TAB_WIDTH: usize = 4;

/// Maximum file size that can be opened (in bytes)
/// This prevents accidentally opening huge files
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Default log file path relative to the editor's working directory
pub const DEFAULT_LOG_PATH: &str = ".log/editor.log";

/// Maximum log file size before rotation (in bytes)
pub const MAX_LOG_SIZE: usize = 5 * 1024 * 1024; // 5MB

/// Number of backup log files to keep
pub const MAX_LOG_FILES: usize = 5;

/// Default key bindings
/// These are the basic Vim-like key bindings
pub const DEFAULT_KEYBINDINGS: &[(&'static str, &'static str)] = &[
    ("←", "move_left"), 
    ("→", "move_right"),
    ("↑", "move_up"),
    ("↓", "move_down"),
    ("i", "enter_insert_mode"),
    ("esc", "enter_normal_mode"),
    (":w", "save_file"),
    (":q", "quit"),
    (":a", "append_after_cursor"),
    (":A", "append_end_of_line"),
    (":o", "open_line_below"),
    (":O", "open_line_above"),
    (":c", "change_to_insert_mode"),
    (":C", "change_to_insert_mode_end_of_line"),
];

/// use spaces for tabs
pub const DEFAULT_USE_SPACES: bool = true;

/// default is to show line numbers
pub const DEFAULT_SHOW_LINE_NUMBERS: bool = true;

/// default is to highlight the current line
pub const DEFAULT_HIGHLIGHT_CURRENT_LINE: bool = true;

/// Editor modes
/// These define the different states the editor can be in
#[derive(Debug, Clone, Copy, PartialEq)]
/// partial eq is used to compare two instances of the same enum allows != and == to be used
/// copy makes deepcopy and clone faster and not change the original value
pub enum EditorMode {
    Normal,  // Command mode (like Vim's normal mode)
    Insert,  // Text insertion mode
    Command, // Command line mode (like Vim's : mode)
}

/// Log levels for the editor
/// These match the standard logging levels
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
} 