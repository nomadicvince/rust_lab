# CosmiqNotz Implementation Guide

This guide explains how the CosmiqNotz app works and how the different components interact with each other.

## 1. Application Architecture

CosmiqNotz follows a clean architecture with several layers:

### Frontend (Yew)
- **Components**: UI building blocks (NoteEditor, NoteList, Toolbar)
- **Models**: Data structures (Note)
- **Services**: Communication with API and local storage
- **App**: Main application logic and state management

### Backend (Rocket)
- **API Endpoints**: RESTful interface for notes
- **Database Interface**: Communication with SurrealDB
- **Error Handling**: Proper HTTP status codes and error responses

### Desktop Shell (Tauri)
- **System Tray**: Quick access to app functions
- **Window Management**: Desktop window integration
- **Native Features**: File system access for import/export
- **Background Services**: Starting SurrealDB and API server

## 2. Key Implementation Features

### Offline-First Approach

The offline-first architecture is implemented through:

1. **Local Storage Priority**:
   ```rust
   // Save to localStorage first
   note_service::save_note_locally(&note);
   
   // If online, also save to API
   if self.is_online {
       note_service::update_note(&note).await;
   }
   ```

2. **Sync Queue**:
   ```rust
   // Add to sync queue when saving locally
   fn add_to_sync_queue(key: &str) -> Result<(), String> {
       // Load existing queue or create new one
       let queue = match storage.get_item("sync_queue") {
           Some(json) => { /* parse existing queue */ },
           None => vec![key.to_string()]
       };
       
       // Save updated queue
       storage.set_item("sync_queue", &json)?;
   }
   ```

3. **Auto-Sync on Reconnect**:
   ```rust
   // In App component
   AppMsg::OnlineStatusChanged(is_online) => {
       let was_offline = !self.is_online;
       self.is_online = is_online;
       
       // If we just came online, try to sync
       if is_online && was_offline {
           ctx.link().send_message(AppMsg::SyncNotes);
       }
   }
   ```

### User Interface Flow

1. **App Initialization**:
   - App starts with Tauri
   - SurrealDB and API server are launched
   - Yew frontend loads and tries to fetch notes

2. **Note Management**:
   - Notes are displayed in the sidebar
   - Clicking a note loads it in the editor
   - Editing automatically marks the note as "dirty"
   - Saving updates the note locally and on the server

3. **Sync Process**:
   - The app periodically checks online status
   - When online, the sync button is enabled
   - Clicking sync processes the sync queue
   - Sync status is shown in the toolbar

## 3. Directory Structure Explained

```
cosmiqnotz/
├── Cargo.toml                   # Workspace configuration
│
├── src/                         # Yew frontend code
│   ├── main.rs                  # Application entry point
│   ├── app.rs                   # Main App component
│   ├── models/                  # Data models
│   ├── components/              # UI components
│   ├── services/                # API services
│   └── styles/                  # CSS styles
│
├── api/                         # Rocket API server
│   ├── Cargo.toml               # API dependencies
│   └── src/main.rs              # API server code
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

## 4. How to Extend the App

### Adding User Authentication

1. Create user model and service:
   ```rust
   // src/models/user.rs
   pub struct User {
       pub id: Option<String>,
       pub username: String,
       pub email: String,
   }
   
   // src/services/auth_service.rs
   pub async fn login(username: &str, password: &str) -> Result<User, String> {
       // Implementation
   }
   ```

2. Add login component:
   ```rust
   // src/components/login.rs
   #[function_component(Login)]
   pub fn login(props: &LoginProps) -> Html {
       // Implementation
   }
   ```

3. Add authentication to API endpoints:
   ```rust
   // api/src/main.rs
   #[get("/notes")]
   async fn get_notes(user: User, state: &State<Arc<AppState>>) -> Json<Vec<Note>> {
       // Filter notes by user
   }
   ```

### Adding Real-Time Collaboration

1. Add WebSocket support to the API server:
   ```rust
   // api/src/main.rs
   #[get("/notes/<id>/ws")]
   fn note_websocket(id: &str, ws: WebSocketUpgrade) -> impl Handler {
       ws.on_upgrade(|ws| async move {
           // Handle WebSocket connection
       })
   }
   ```

2. Add collaboration service to the frontend:
   ```rust
   // src/services/collaboration_service.rs
   pub struct CollaborationService {
       ws: Option<WebSocket>,
       // Other fields
   }
   
   impl CollaborationService {
       pub fn new() -> Self {
           // Implementation
       }
       
       pub fn connect(&mut self, note_id: &str) -> Result<(), String> {
           // Implementation
       }
       
       pub fn send_update(&self, update: &NoteUpdate) -> Result<(), String> {
           // Implementation
       }
   }
   ```

## 5. Running the App

### Development

1. Start the development server:
   ```bash
   cargo tauri dev
   ```

This will:
- Start the SurrealDB server if not running
- Run the Rocket API server
- Compile and serve the Yew frontend
- Launch the Tauri window

### Production Build

1. Build the production version:
   ```bash
   cargo tauri build
   ```

This will generate installers in `src-tauri/target/release/bundle/`.