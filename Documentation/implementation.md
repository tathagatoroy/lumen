## OBJECTIVE:
    1. To learn Rust
    2. Learn how to architect and design a complex system in safe, extendable, optimised and efficient manner
    3. Get used to using cursor and how good it is.
    4. To learn how code/text editor works 


## Tentative Project Structure

```
lumen/
├── src/
│   ├── main.rs           # Entry point
│   ├── editor/           # Core editor functionality
│   │   ├── mod.rs
│   │   ├── buffer.rs     # Text buffer implementation
│   │   ├── cursor.rs     # Cursor handling
│   │   └── view.rs       # View/display logic
│   ├── ui/              # User interface components
│   │   ├── mod.rs
│   │   ├── window.rs     # Window management
│   │   └── renderer.rs   # Rendering logic
│   ├── input/           # Input handling
│   │   ├── mod.rs
│   │   └── events.rs     # Event processing
│   └── utils/           # Utility functions
│       ├── mod.rs
│       └── helpers.rs
├── tests/               # Integration tests
├── examples/            # Example usage
├── Cargo.toml          # Project manifest
├── Cargo.lock          # Dependency lock file
├── .gitignore
├── LICENSE
└── README.md
```

## Description

Lumen is a modern, terminal-based text editor written in Rust. It aims to provide a fast, efficient, and extensible editing experience.

## Features (Planned)

- Terminal-based interface
- Syntax highlighting
- Multiple file support
- Search and replace
- Configurable keybindings
- Plugin system
- Autosave 
- Parenthesis matching 
- GUI 
- Configurable Themes


## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## ACKNOWLEDGEMENTS:
1. Heavy use of AI tools (DeepSeek, Gemini, Cursor)
2. [Xi](https://xi-editor.io/docs/)
3. [Zed](https://github.com/zed-industries/zed)