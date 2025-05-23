#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod editor;
mod ui;
mod input;
mod utils;
mod config;


use std::env;
use crate::config::{Config};
use crate::config::settings::{Args};
use std::error::Error;
use crossterm::event::KeyEventKind;
use utils::file_ops::readFile;
use utils::error::{editorError};
use clap::Parser;
use std::io::{Write, stdout};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    style::Print,
    Result as CrosstermResult,
    QueueableCommand
};
use log::{debug, info, error};
use env_logger;

fn run_editor() -> Result<(), editorError>{
    let mut stdout = stdout();

    // enable raw mode 
    terminal::enable_raw_mode().map_err(|e| editorError:: TerminalError {source : e})?;
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), cursor::MoveTo(0,0))
    .map_err(|e|editorError::TerminalError { source: e })?;

    let (cols, rows) = terminal::size().map_err(|e| editorError::TerminalError { source: e })?;
    info!("Initial terminal size: {} cols, {} rows", cols, rows); // Avoid println in TUI


    let mut rowString = String::from("");
    loop {
        let event = event::read().map_err(|e| editorError::EventReadError { source: e })?;

        match event{
            Event::Key (KeyEvent {
                code,
                modifiers,
                kind,
                state: _,
            }) => {
                // handle control Q to exit 
                if kind == KeyEventKind::Press || kind == KeyEventKind::Repeat{
                    if code == KeyCode::Char('w') && modifiers.contains(KeyModifiers::CONTROL){
                        info!("kind : {:?} code : {:?}  modifiers : {:?}", kind, code, modifiers);
                        break;
                    }
                }
                if kind == KeyEventKind::Press{
                    info!("kind : {:?} code : {:?}  modifiers : {:?}", kind, code, modifiers);

                    match code { 
                        KeyCode::Char(c) => {
                            rowString.push(c);
                        }
                        KeyCode::Backspace => {
                            rowString.pop();
                        }
                        KeyCode::Enter => {
                            rowString.push('\n');
                        }

                        _ => {}
                    }
                }
            }
            _ => {}
        }
        execute!(
            stdout,
            cursor::MoveTo(0,0),
            Clear(ClearType::CurrentLine),
            Print(rowString.as_str())
        )
        .map_err(|e| editorError::TerminalError { source: e })?;

        stdout.flush().map_err(editorError::IOError)?;


    }
    //cleanup

    

    execute!(stdout, LeaveAlternateScreen, cursor::Show)
    .map_err(|e|editorError::TerminalError { source: e })?;
    terminal::disable_raw_mode()
    .map_err(|e| editorError::TerminalError{source : e})?;
    Ok(())
}

fn main() -> Result<(), editorError> {
    // Initialize logging
    std::fs::create_dir_all("logs").map_err(editorError::IOError)?;
    let file = std::fs::File::create("logs/lumen.log").map_err(editorError::IOError)?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
    
    // Initialize editor components
    info!("Lumen Text Editor - Starting up...");
    debug!("Debug message test");
    info!("Info message test");
    error!("Error message test");
    
    // Parse command line arguments and create config
    // parse the command line arguments and create a config defined in settings.rs
    let args = Args::parse();
    let config = Config::new(args)?;
    let settings = config.settings.clone();
    // get the args from the config as the old args was moved and is now in the config
    let args = config.args.clone();


    info!("Settings : {:?}", settings);
    info!("Args : {:?}", args);


    // Print to stdout for user feedback

    info!("Configuration loaded successfully");
    

    if let Some(file) = config.fileToOpen() {
        info!("Opening file: {:?}", file);
        // open the file
        let file = readFile(&file)?;
    }
    run_editor()?;

    Ok(())
} 
