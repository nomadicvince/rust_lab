# Rust Practice

Sharpen your Rust skills through hands-on projects in Systems Programming, Artificial Intelligence, and WebAssembly.

## Overview

This repository is a curated Rust development lab designed to support continuous learning and experimentation. It includes standalone modules and projects that explore core and advanced topics in:

- Systems Programming – low-level memory management, multithreading, OS interaction
- Artificial Intelligence (AI) – model serving, numerical computing, and ML integration
- WebAssembly (Wasm) – compile-to-web workflows, UI interactivity, and embedded logic

Each folder is scoped to a specific concept or domain to encourage modular exploration.

## Repository Structure

rust_practice/
├── systems_programming/
│   ├── memory_management/
│   ├── concurrency/
│   └── file_io/
├── ai_projects/
│   ├── rust_ml_basics/
│   ├── llama_cpp_integration/
│   └── model_serving/
├── webassembly/
│   ├── wasm_hello_world/
│   ├── wasm_ui_bindings/
│   └── wasm_ai_demo/
├── utilities/
│   ├── cli_tools/
│   └── benchmarking/
└── README.md

## Getting Started

### Prerequisites

- Rust toolchain: https://www.rust-lang.org/tools/install
- wasm-pack for WebAssembly targets
- cargo for building and running projects
- Optional: just, make, or nix for automation and reproducibility

### Clone the Repo

git clone https://github.com/nomadicvince/rust_practice.git
cd rust_practice

## Usage

Navigate to any directory and build using:

cargo build
cargo run

To run WebAssembly examples:

wasm-pack build --target web

## Goals

- Build strong intuition for Rust ownership, lifetimes, and traits
- Apply Rust to AI workloads including bindings to C/C++ or Python (e.g. using tch-rs or llm-rs)
- Develop fast, portable Wasm components for web and embedded platforms
- Establish a personal knowledge base of reusable patterns and performance techniques

## Highlights

- Memory-safe system calls and unsafe Rust examples (with documentation)
- AI microservices in Rust with model loading logic
- WebAssembly demos using Yew, Leptos, and vanilla JS bindings
- CLI tool prototypes with clap, anyhow, and log
- Benchmarks for comparing Rust to C or Python equivalents

## Contributing

This repo is a personal learning sandbox.

## License

MIT License. See LICENSE file for details.
