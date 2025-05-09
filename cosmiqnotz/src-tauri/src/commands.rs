use std::process::Command;
use std::fs;
use std::path::Path;
use std::io::Write;
use tauri::command;

#[command]
pub fn check_api_status() -> bool {
    // Simple check to see if the API is responding
    let status = Command::new("curl")
        .arg("--silent")
        .arg("--head")
        .arg("--output")
        .arg("/dev/null")
        .arg("--write-out")
        .arg("%{http_code}")
        .arg("http://localhost:8000/api/notes")
        .output();
    
    match status {
        Ok(output) => {
            if output.status.success() {
                let status_code = String::from_utf8_lossy(&output.stdout);
                status_code.starts_with('2') || status_code.starts_with('3')
            } else {
                false
            }
        },
        Err(_) => false,
    }
}

#[command]
pub fn export_notes(path: String) -> Result<String, String> {
    // Export notes to a JSON file
    let notes = Command::new("curl")
        .arg("--silent")
        .arg("http://localhost:8000/api/notes")
        .output()
        .map_err(|e| format!("Failed to fetch notes: {}", e))?;
    
    if !notes.status.success() {
        return Err("Failed to fetch notes from API".to_string());
    }
    
    let mut file = fs::File::create(&path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    file.write_all(&notes.stdout)
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    
    Ok(format!("Notes exported to {}", path))
}

#[command]
pub fn import_notes(path: String) -> Result<String, String> {
    // Import notes from a JSON file
    if !Path::new(&path).exists() {
        return Err("File does not exist".to_string());
    }
    
    let notes = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let status = Command::new("curl")
        .arg("--silent")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&notes)
        .arg("http://localhost:8000/api/notes/import")
        .output()
        .map_err(|e| format!("Failed to import notes: {}", e))?;
    
    if !status.status.success() {
        return Err("Failed to import notes".to_string());
    }
    
    Ok("Notes imported successfully".to_string())
}

pub fn start_surrealdb() -> Result<(), String> {
    // Check if SurrealDB is already running
    let status = Command::new("pgrep")
        .arg("surreal")
        .output()
        .map_err(|e| format!("Failed to check if SurrealDB is running: {}", e))?;
    
    if status.status.success() {
        // SurrealDB is already running
        return Ok(());
    }
    
    // Start SurrealDB
    Command::new("surreal")
        .arg("start")
        .arg("--user")
        .arg("root")
        .arg("--pass")
        .arg("root")
        .arg("file:data.db")
        .spawn()
        .map_err(|e| format!("Failed to start SurrealDB: {}", e))?;
    
    // Wait for SurrealDB to start
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Initialize database if needed
    let db_path = Path::new("data.db");
    if !db_path.exists() {
        // Run the initialization script
        let init_script = include_str!("../../migrations/init.surql");
        
        let mut file = fs::File::create("init_temp.surql")
            .map_err(|e| format!("Failed to create temp init file: {}", e))?;
        
        file.write_all(init_script.as_bytes())
            .map_err(|e| format!("Failed to write to temp init file: {}", e))?;
        
        Command::new("surreal")
            .arg("import")
            .arg("--conn")
            .arg("http://localhost:8000")
            .arg("--user")
            .arg("root")
            .arg("--pass")
            .arg("root")
            .arg("--ns")
            .arg("cosmiqnotz")
            .arg("--db")
            .arg("cosmiqnotz")
            .arg("init_temp.surql")
            .output()
            .map_err(|e| format!("Failed to initialize database: {}", e))?;
        
        // Clean up
        fs::remove_file("init_temp.surql").ok();
    }
    
    Ok(())
}

pub fn start_api_server() -> Result<(), String> {
    // Check if the API server is already running
    let status = Command::new("pgrep")
        .arg("-f")
        .arg("cosmiqnotz_api")
        .output()
        .map_err(|e| format!("Failed to check if API server is running: {}", e))?;
    
    if status.status.success() {
        // API server is already running
        return Ok(());
    }
    
    // Start the API server
    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg("cosmiqnotz_api")
        .current_dir("../api")
        .spawn()
        .map_err(|e| format!("Failed to start API server: {}", e))?;
    
    // Wait for the API server to start
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    Ok(())
}