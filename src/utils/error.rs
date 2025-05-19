#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
/// wrapper to handle all errors
pub enum editorError {
    #[error("Failure to read File : {path} , Error : {source}")]
    ReadError { path : String, source : io::Error},

    #[error("Terminal Operation Failed : {source}")]
    TerminalError { source :  io::Error },

    #[error("Event Read Unsuccesful : {source}")]
    EventReadError { source : io::Error},

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("TOML serialization failed: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("TOML deserialization failed: {0}")]
    TomlDeserializeError(#[from] toml::de::Error)
}

