# Lumen - A Modern Text Editor in Rust

## Learning Context

This project is a learning exercise to understand both Rust programming and text editor design. The goal is to build a terminal-based text editor from scratch, learning concepts like:

- Rust's ownership and borrowing system
- Terminal manipulation and raw mode
- Text buffer management
- Event handling
- State management
- Error handling
- Logging and debugging

## System Architecture

### Core Components

1. **Terminal Interface Layer**
   - Raw mode handling (disabling terminal features for direct input)
   - Screen management (clearing, drawing, cursor positioning)
   - Input event processing

2. **Text Buffer Layer**
   - Gap buffer implementation (efficient text storage)
   - Line management
   - Undo/redo functionality

3. **Editor State**
   - Current file information
   - Cursor position
   - Viewport state
   - Mode management (normal/insert)

4. **Configuration System**
   - Build-time constants
   - Runtime configuration
   - Key binding customization

5. **Logging System**
   - File-based logging
   - Different log levels
   - Rotating log files

## Project Structure

```
lumen/
├── src/
│   ├── main.rs           # Entry point and argument parsing
│   ├── config/           # Configuration management
│   │   ├── mod.rs
│   │   ├── constants.rs  # Build-time constants
│   │   └── settings.rs   # Runtime settings
│   ├── editor/           # Core editor functionality
│   │   ├── mod.rs
│   │   ├── buffer.rs     # Gap buffer implementation
│   │   ├── cursor.rs     # Cursor handling
│   │   └── view.rs       # View/display logic
│   ├── ui/              # User interface components
│   │   ├── mod.rs
│   │   ├── window.rs     # Window management
│   │   └── renderer.rs   # Rendering logic
│   ├── input/           # Input handling
│   │   ├── mod.rs
│   │   └── events.rs     # Event processing
│   ├── logging/         # Logging system
│   │   ├── mod.rs
│   │   └── logger.rs     # Log file management
│   └── utils/           # Utility functions
│       ├── mod.rs
│       └── helpers.rs
├── .log/                # Log files directory
├── tests/               # Integration tests
├── examples/            # Example usage
├── Cargo.toml          # Project manifest
├── Cargo.lock          # Dependency lock file
├── .gitignore
├── LICENSE
└── README.md
```

## MVP Features

1. **Basic Text Editing**
   - Open and save files
   - Insert and delete text
   - Basic cursor movement
   - Simple viewport management

2. **Terminal Interface**
   - Raw mode support
   - Basic screen clearing and drawing
   - Cursor positioning

3. **Configuration**
   - Command-line arguments
   - Basic settings file
   - Logging configuration

4. **Logging**
   - File-based logging
   - Different log levels
   - Basic log rotation

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Basic understanding of terminal operations

### Building

```bash
cargo build
```

### Running

```bash
# Basic usage
cargo run

# With configuration file
cargo run -- --config path/to/config.toml

# With debug logging
cargo run -- --log-level debug
```

## Learning Path

1. **Phase 1: Terminal Basics**
   - Understanding raw mode
   - Basic screen manipulation
   - Input handling

2. **Phase 2: Text Buffer**
   - Implementing gap buffer
   - Basic editing operations
   - Cursor management

3. **Phase 3: Editor Features**
   - File I/O
   - Viewport management
   - Basic commands

4. **Phase 4: Configuration & Logging**
   - Settings management
   - Logging system
   - Error handling

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
