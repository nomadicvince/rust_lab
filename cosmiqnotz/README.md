# CosmiqNotz

A real-time collaborative note-taking application built with the RSTY stack (Rocket, SurrealDB, Tauri, and Yew).

![CosmiqNotz Banner](https://via.placeholder.com/1200x300/4CAF50/FFFFFF?text=CosmiqNotz)

## Features

- **Offline-First**: Work without an internet connection; changes sync when connectivity is restored
- **Real-Time Collaboration**: Collaborate with team members simultaneously
- **Version History**: Track changes and restore previous versions
- **Cross-Platform**: Available for Windows, macOS, and Linux

## Tech Stack

- **[Rocket](https://rocket.rs/)**: Rust web framework for the API backend
- **[SurrealDB](https://surrealdb.com/)**: Graph database for data storage
- **[Tauri](https://tauri.app/)**: Framework for building desktop applications
- **[Yew](https://yew.rs/)**: Rust / WebAssembly framework for frontend

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable channel)
- [Node.js](https://nodejs.org/) (v16 or newer)
- [SurrealDB](https://surrealdb.com/install)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cosmiqnotz.git
   cd cosmiqnotz
   ```

2. Install Trunk (for compiling Yew):
   ```bash
   cargo install trunk
   ```

3. Install Tauri CLI:
   ```bash
   cargo install tauri-cli
   ```

4. Start the development server:
   ```bash
   cargo tauri dev
   ```

This will automatically:
- Start SurrealDB if not already running
- Initialize the database schema
- Launch the Rocket API server
- Open the app in a Tauri window

## Usage

- Create new notes with the "New Note" button
- Edit notes in the editor
- Notes are automatically saved when you edit them
- Changes sync automatically when online
- Use the System Tray icon for quick access

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
