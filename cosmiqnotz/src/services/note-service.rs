use crate::models::Note;
use gloo_net::http::Request;
use web_sys::console;

// For API requests
const API_BASE: &str = "http://localhost:8000/api";

// Remote API calls
pub async fn get_notes() -> Result<Vec<Note>, String> {
    match Request::get(&format!("{}/notes", API_BASE))
        .header("Content-Type", "application/json")
        .send()
        .await 
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Note>>().await {
                    Ok(notes) => Ok(notes),
                    Err(e) => Err(format!("Failed to parse response: {}", e))
                }
            } else {
                Err(format!("Failed to get notes: HTTP {}", response.status()))
            }
        },
        Err(e) => Err(format!("Network error: {}", e))
    }
}

pub async fn create_note(note: &Note) -> Result<Note, String> {
    match Request::post(&format!("{}/notes", API_BASE))
        .header("Content-Type", "application/json")
        .json(note)
        .map_err(|e| format!("Failed to serialize note: {}", e))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 || response.status() == 201 {
                match response.json::<Note>().await {
                    Ok(note) => Ok(note),
                    Err(e) => Err(format!("Failed to parse response: {}", e))
                }
            } else {
                Err(format!("Failed to create note: HTTP {}", response.status()))
            }
        },
        Err(e) => Err(format!("Network error: {}", e))
    }
}

pub async fn update_note(note: &Note) -> Result<Note, String> {
    let id = match &note.id {
        Some(id) => id.clone(),
        None => return Err("Note ID is missing".to_string())
    };

    match Request::put(&format!("{}/notes/{}", API_BASE, id))
        .header("Content-Type", "application/json")
        .json(note)
        .map_err(|e| format!("Failed to serialize note: {}", e))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Note>().await {
                    Ok(note) => Ok(note),
                    Err(e) => Err(format!("Failed to parse response: {}", e))
                }
            } else {
                Err(format!("Failed to update note: HTTP {}", response.status()))
            }
        },
        Err(e) => Err(format!("Network error: {}", e))
    }
}

// Local storage for offline functionality
pub fn save_note_locally(note: &Note) -> Result<(), String> {
    let storage = web_sys::window()
        .ok_or_else(|| "No window found".to_string())?
        .local_storage()
        .map_err(|_| "Failed to access localStorage".to_string())?
        .ok_or_else(|| "localStorage not available".to_string())?;

    let key = match &note.id {
        Some(id) => format!("note_{}", id),
        None => format!("note_draft_{}", js_sys::Date::now())
    };

    let json = serde_json::to_string(note)
        .map_err(|e| format!("Failed to serialize note: {}", e))?;

    storage.set_item(&key, &json)
        .map_err(|_| "Failed to save note to localStorage".to_string())?;

    // Add to sync queue if we have an ID
    if note.id.is_some() {
        add_to_sync_queue(&key)?;
    }

    Ok(())
}

pub fn get_local_notes() -> Result<Vec<Note>, String> {
    let storage = web_sys::window()
        .ok_or_else(|| "No window found".to_string())?
        .local_storage()
        .map_err(|_| "Failed to access localStorage".to_string())?
        .ok_or_else(|| "localStorage not available".to_string())?;

    let mut notes = Vec::new();

    for i in 0..storage.length().map_err(|_| "Failed to get storage length".to_string())? {
        let key = storage.key(i).map_err(|_| "Failed to get key from storage".to_string())?
            .ok_or_else(|| "No key found".to_string())?;

        if key.starts_with("note_") {
            if let Some(json) = storage.get_item(&key).map_err(|_| "Failed to get item from storage".to_string())? {
                match serde_json::from_str::<Note>(&json) {
                    Ok(note) => notes.push(note),
                    Err(e) => console::log_1(&format!("Failed to parse note: {}", e).into())
                }
            }
        }
    }

    // Sort by updated_at
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    Ok(notes)
}

fn add_to_sync_queue(key: &str) -> Result<(), String> {
    let storage = web_sys::window()
        .ok_or_else(|| "No window found".to_string())?
        .local_storage()
        .map_err(|_| "Failed to access localStorage".to_string())?
        .ok_or_else(|| "localStorage not available".to_string())?;

    let queue = match storage.get_item("sync_queue").map_err(|_| "Failed to get sync queue".to_string())? {
        Some(json) => {
            let mut queue: Vec<String> = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse sync queue: {}", e))?;
            
            if !queue.contains(&key.to_string()) {
                queue.push(key.to_string());
            }
            
            queue
        },
        None => vec![key.to_string()]
    };

    let json = serde_json::to_string(&queue)
        .map_err(|e| format!("Failed to serialize sync queue: {}", e))?;

    storage.set_item("sync_queue", &json)
        .map_err(|_| "Failed to save sync queue to localStorage".to_string())?;

    Ok(())
}

pub async fn process_sync_queue() -> Result<(), String> {
    let storage = web_sys::window()
        .ok_or_else(|| "No window found".to_string())?
        .local_storage()
        .map_err(|_| "Failed to access localStorage".to_string())?
        .ok_or_else(|| "localStorage not available".to_string())?;

    let queue = match storage.get_item("sync_queue").map_err(|_| "Failed to get sync queue".to_string())? {
        Some(json) => {
            serde_json::from_str::<Vec<String>>(&json)
                .map_err(|e| format!("Failed to parse sync queue: {}", e))?
        },
        None => return Ok(()) // No queue, nothing to sync
    };

    if queue.is_empty() {
        return Ok(());
    }

    let mut new_queue = Vec::new();

    for key in queue {
        let json = match storage.get_item(&key).map_err(|_| "Failed to get item from storage".to_string())? {
            Some(json) => json,
            None => {
                // Item doesn't exist anymore, skip it
                continue;
            }
        };

        let note: Note = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse note: {}", e))?;

        // Try to sync the note
        match update_note(&note).await {
            Ok(_) => {
                // Successfully synced, remove from queue
                console::log_1(&format!("Successfully synced note: {}", note.id.as_ref().unwrap_or(&"unknown".to_string())).into());
            },
            Err(e) => {
                // Failed to sync, keep in queue
                console::log_1(&format!("Failed to sync note: {}", e).into());
                new_queue.push(key);
            }
        }
    }

    // Update the queue
    let json = serde_json::to_string(&new_queue)
        .map_err(|e| format!("Failed to serialize sync queue: {}", e))?;

    storage.set_item("sync_queue", &json)
        .map_err(|_| "Failed to save sync queue to localStorage".to_string())?;

    Ok(())
}

pub fn check_online_status() -> bool {
    web_sys::window()
        .and_then(|window| Some(window.navigator().online()))
        .unwrap_or(false)
}