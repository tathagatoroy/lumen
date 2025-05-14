use std::{error::Error as StdError, fmt, io};
use crossterm::ErrorKind as CrosstermErrorKind;

#[derive(Debug)]
pub enum EditorError {
    Io(io::Error),
    // Crossterm(CrosstermErrorKind),
    ConfigError(Box<dyn StdError>),
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::Io(e) => write!(f, "IO error: {}", e),
            // EditorError::Crossterm(e) => write!(f, "Crossterm error: {}", e),
            EditorError::ConfigError(e) => write!(f, "Config error: {}", e),
        }
    }
}

impl StdError for EditorError {}

impl From<io::Error> for EditorError {
    fn from(err: io::Error) -> Self {
        EditorError::Io(err)
    }
}

// impl From<CrosstermErrorKind> for EditorError {
//     fn from(err: CrosstermErrorKind) -> Self {
//         EditorError::Crossterm(err)
//     }
// }

impl From<Box<dyn StdError>> for EditorError {
    fn from(err: Box<dyn StdError>) -> Self {
        EditorError::ConfigError(err)
    }
}

// pub type Result<T> = std::result::Result<T, EditorError>;
