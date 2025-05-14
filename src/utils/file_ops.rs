use std::fs::File;
use std::io;
use std::path::PathBuf;
use log::{info, error};

// read a file and log if it is opened successfully
pub fn readFile(path: &PathBuf) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => {
            //  file read successfully
            info!("Successfully opened file: {:?}", path);
            Ok(file)
        }
        Err(e) => {
            error!("Failed to open file {:?}: {}", path, e);
            Err(e)
        }
    }
}

