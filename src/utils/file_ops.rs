#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io;
use std::path::PathBuf;
use log::{info, error};
use crate::utils::error::editorError;

// read a file and log if it is opened successfully
pub fn readFile(path: &PathBuf) -> Result<File, editorError> {
    match File::open(path) {
        Ok(file) => {
            //  file read successfully
            info!("Successfully opened file: {:?}", path);
            Ok(file)
        }
        Err(e) => {
            Err(editorError::ReadError{path: path.to_string_lossy().to_string(), source: e})
        }
    }
}

