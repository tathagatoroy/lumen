#![allow(unused_imports)]
#![allow(unused_variables)]

mod editor;
mod ui;
mod input;
mod utils;
mod config;


use crate::config::{Config};
use crate::config::settings::{Args};
use std::error::Error;
use utils::file_ops::readFile;
use utils::error::EditorError;
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

fn run_editor() -> CrosstermResult<()> {
    let mut stdout = stdout();

    // enable raw mode 
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    let mut rowString = String::from("");
    loop {
        // get terminal size 
        if let Event::Resize(cols, rows) = event::read()?{
            let rows = rows as usize;
            let cols = cols as usize;
            println!("Terminal resized to {}x{}", cols, rows);

        }

        // read input event 
        if let Event::Key(KeyEvent {code, modifiers, kind, state}) = event::read()?{
            if code == KeyCode::Char('q') && modifiers.contains(KeyModifiers::CONTROL) {
                break;
            }
            match code {
                KeyCode::Char(c) => {
                    rowString.push(c);
                }
                _ => {}
            }
        }
        // update the string to be rendered
        execute!(stdout, cursor::MoveTo(0, 0), Print(rowString.clone()))?;
        stdout.flush()?;

    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<(), EditorError> {
    // Initialize logging
    env_logger::init();
    
    // Initialize editor components
    println!("Lumen Text Editor - Starting up...");
    
    // Parse command line arguments and create config
    // parse the command line arguments and create a config defined in settings.rs
    let args = Args::parse();
    let config = Config::new(args)?;
    let settings = config.settings.clone();
    // get the args from the config as the old args was moved and is now in the config
    let args = config.args.clone();

    if args.debug {
        println!("Settings: {:?}", settings);
        println!("Args: {:?}", args);
    }


    // Print to stdout for user feedback
    if args.debug {
        println!("Configuration loaded successfully");
    }

    if let Some(file) = config.fileToOpen() {
        if args.debug {
            println!("Opening file: {:?}", file);
        }
        // open the file
        let file = readFile(&file)?;
    }
    run_editor()?;

    Ok(())
} 
