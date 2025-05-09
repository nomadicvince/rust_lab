# 🦀 Rust Lab

**A personal laboratory for exploring Rust, CLI tools, Model Context Protocol, Full-Stack Development, and WebAssembly.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)

This repository is my learning journey with Rust, focusing on several key areas that will evolve over time:

- **🔧 Rust Fundamentals** – Core language features, idioms, and best practices
- **📟 CLI Applications** – Building command-line tools with modern Rust libraries
- **🧠 Model Context Protocol** – Implementing backends for AI model interaction
- **🏗️ Full-Stack Development** – Exploring Rocket, SurrealDB, Tauri, and Yew
- **🌐 WebAssembly (Wasm)** – Compiling Rust to browser environments for web applications

## 🚀 Getting Started

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (1.80 or newer recommended)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly projects
- Development utilities (optional but recommended):
  - [`cargo-edit`](https://github.com/killercup/cargo-edit) - Dependency management
  - [`cargo-watch`](https://github.com/watchexec/cargo-watch) - Auto-rebuild on changes
  - [`just`](https://github.com/casey/just) - Command runner
  - [`nix`](https://nixos.org/download.html) - Reproducible development environment

### Installation

```bash
# Clone the repository
git clone https://github.com/nomadicvince/rust_practice.git
cd rust_practice

# Install recommended development tools
cargo install cargo-edit cargo-watch just
```

### Build and Run

Each subdirectory contains independent projects that can be run individually:

```bash
# Run a systems programming example
cd systems/file_io
cargo run

# Build a WebAssembly project
cd wasm/wasm_hello_world
wasm-pack build --target web
# Then serve the resulting files with your preferred web server
```

## 📂 Repository Structure

This repository structure will evolve as I explore different aspects of Rust:

```
rust_practice/
├── fundamentals/        # Core Rust language experiments
├── cli/                 # Command-line application projects
├── model_context/       # Model Context Protocol implementation
├── full_stack/          # Rocket, SurrealDB, Tauri and Yew projects
│   ├── backend/         # Rocket API server with SurrealDB
│   └── frontend/        # Tauri and Yew client applications
├── wasm/                # WebAssembly experiments
└── utils/               # Shared utilities and tools
```

*Note: The specific directories and project structure may change as this learning repository evolves.*

## 🎯 Project Goals

- Master Rust idioms, ownership concepts, and type system for writing safe and efficient code
- Build practical CLI tools that demonstrate Rust's strengths in systems programming
- Develop a Model Context Protocol implementation for AI model interactions
- Create a full-stack application using the Rocket, SurrealDB, Tauri, and Yew ecosystem
- Explore WebAssembly compilation and integration with web applications
- Document my learning journey with clear examples and notes for future reference

## 🔍 Focus Areas

### Rust Fundamentals

- Core language features, ownership, borrowing, and lifetimes
- Advanced traits and generics
- Error handling patterns
- Performance optimization techniques

### CLI Applications

- Command-line argument parsing and user interfaces
- File system operations and data processing
- Configuration management and environment handling
- Cross-platform compatibility

### Model Context Protocol

- Backend implementation for AI model interaction
- Efficient context management and retrieval
- API design for model integration
- Performance-focused processing pipelines

### Full-Stack Development

- RESTful API services with Rocket
- Database interactions with SurrealDB
- Cross-platform desktop applications with Tauri
- Frontend web components with Yew

### WebAssembly

- Compiling Rust code to WebAssembly
- JavaScript interoperability
- Browser integration patterns
- Performance comparison with JavaScript

## 🛠️ Technologies

This repository will explore various Rust libraries and frameworks, including:

### Core
- `serde` - Serialization framework
- `tokio` - Asynchronous runtime
- `rayon` - Data-parallelism library

### CLI
- `clap` - Command-line argument parsing
- `inquire` - Interactive user prompts
- `indicatif` - Progress bars and indicators

### Backend
- `rocket` - Web framework
- `surrealdb` - Database driver
- `axum` - Web server toolkit (alternative exploration)

### Frontend
- `tauri` - Desktop application framework
- `yew` - Web framework for Rust
- `leptos` - Reactive web framework (alternative exploration)

### WebAssembly
- `wasm-bindgen` - JS interoperability
- `web-sys` - Web API bindings
- `js-sys` - JavaScript API bindings

### Utils
- `anyhow` - Error handling
- `tracing` - Structured logging
- `criterion` - Benchmarking

## 📝 Learning Documentation

As I progress through different Rust concepts and projects, I'll document key learnings:

- Code comments explaining Rust-specific patterns and idioms
- Module-level documentation with usage examples
- Notes on performance considerations and optimizations
- Comparisons between different implementation approaches

Each directory will contain its own README with specific details about the concepts explored.

## 👨‍💻 Contributing

While this repository serves primarily as a personal learning project, contributions are welcome, especially if they:

- Demonstrate idiomatic Rust patterns
- Improve documentation or examples
- Add interesting test cases or benchmarks
- Fix bugs or improve performance

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Vincent Moore**  
**Email:** [me@vincentmoore.ai](mailto:me@vincentmoore.ai)  
**Website:** [vincentmoore.ai](https://vincentmoore.ai)  
**GitHub:** [@nomadicvince](https://github.com/nomadicvince)

---

*Built with ❤️ using Rust*
