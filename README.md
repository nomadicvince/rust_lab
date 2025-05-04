# Rust Practice

**A personal Rust development lab for exploring Systems Programming, Artificial Intelligence, and WebAssembly.**

This repository is a collection of hands-on projects and code samples built to deepen proficiency with Rust in three major areas:

- **Systems Programming** – Concurrency, memory safety, file I/O, OS interaction
- **Artificial Intelligence (AI)** – Machine learning workflows, numerical computing, and model serving
- **WebAssembly (Wasm)** – Compiling Rust to the web, UI logic, and performance testing

---

## Repository Structure

```
rust_practice/
├── systems/           # Low-level and OS-related Rust projects
├── ai/                # AI, ML, and numerical computing examples
├── wasm/              # WebAssembly modules and browser-side logic
├── cli_tools/         # Command-line utilities and tools
├── benchmarks/        # Micro-benchmarking and performance profiling
└── README.md
```

---

## Getting Started

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- `wasm-pack` (for WebAssembly)
- Optionally: `cargo-edit`, `cargo-watch`, `just`, or `nix`

### Clone the Repository

```bash
git clone https://github.com/nomadicvince/rust_practice.git
cd rust_practice
```

### Build and Run

Navigate into any project folder:

```bash
cd systems/file_io
cargo run
```

For WebAssembly:

```bash
cd wasm/wasm_hello_world
wasm-pack build --target web
```

---

## Project Goals

- Develop real-world expertise in safe and efficient systems code
- Prototype AI/ML logic with performance-aware Rust code
- Learn and document Rust-to-Wasm workflows
- Create a knowledge base of idiomatic, reusable Rust patterns

---

## Highlights

- Systems modules using `std::fs`, `std::thread`, and `unsafe` blocks
- AI experiments using `ndarray`, `tch-rs`, and integration with `llm` libraries
- WebAssembly projects using `wasm-bindgen`, `Yew`, and raw JS interop
- CLI projects built with `clap`, `anyhow`, and `log`
- Benchmarking tools using `criterion`

---

## Contributing

This repo is maintained as a solo learning project, but contributions are welcome if:

- Code is idiomatic and well-documented
- It adds clarity, performance, or educational value

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Author

**Vincent Moore**  
Founder, Travel Astronomy LLC  
**Email:** [me@vincentmoore.ai](mailto:me@vincentmoore.ai)  
**Website:** [vincentmoore.ai](https://vincentmoore.ai)