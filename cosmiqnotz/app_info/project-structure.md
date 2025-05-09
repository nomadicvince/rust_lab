# CosmiqNotz Project Structure

```
cosmiqnotz/
├── Cargo.toml                   # Workspace configuration
│
├── src/                         # Yew frontend code
│   ├── main.rs                  # Application entry point
│   ├── app.rs                   # Main App component
│   ├── models/                  # Data models
│   │   ├── mod.rs
│   │   └── note.rs              # Note data model
│   ├── components/              # UI components
│   │   ├── mod.rs
│   │   ├── note_editor.rs       # Note editing component
│   │   ├── note_list.rs         # Note listing component
│   │   └── toolbar.rs           # App toolbar component
│   ├── services/                # API services
│   │   ├── mod.rs
│   │   └── note_service.rs      # Note API service
│   └── styles/                  # CSS styles
│       └── main.css
│
├── api/                         # Rocket API server
│   ├── Cargo.toml               # API dependencies
│   └── src/
│       └── main.rs              # API server code
│
├── src-tauri/                   # Tauri desktop app
│   ├── Cargo.toml               # Tauri dependencies
│   ├── tauri.conf.json          # Tauri configuration
│   └── src/
│       ├── main.rs              # Tauri main entry point
│       └── commands.rs          # Tauri commands
│
└── migrations/                  # Database migrations
    └── init.surql               # Initial schema setup
```
