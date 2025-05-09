#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use std::sync::Arc;

// CORS Fairing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Import data models
#[path = "../../src/models/note.rs"]
mod note;
use note::Note;

// Application state with database connection
struct AppState {
    db: Surreal<Client>,
}

// API Endpoints
#[get("/notes")]
async fn get_notes(state: &State<Arc<AppState>>) -> Json<Vec<Note>> {
    let notes: Vec<Note> = state.db.select("note")
        .await
        .unwrap_or_default();
    
    Json(notes)
}

#[get("/notes/<id>")]
async fn get_note(id: &str, state: &State<Arc<AppState>>) -> Result<Json<Note>, Status> {
    let note: Option<Note> = state.db.select(("note", id))
        .await
        .unwrap_or(None);
    
    match note {
        Some(note) => Ok(Json(note)),
        None => Err(Status::NotFound)
    }
}

#[post("/notes", data = "<note>")]
async fn create_note(note: Json<Note>, state: &State<Arc<AppState>>) -> Result<Json<Note>, Status> {
    let mut new_note = note.into_inner();
    
    // Ensure created_at and updated_at are set
    if new_note.created_at.timestamp() == 0 {
        new_note.created_at = chrono::Utc::now();
    }
    new_note.updated_at = chrono::Utc::now();
    
    let created: Option<Note> = state.db.create("note")
        .content(&new_note)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    match created {
        Some(created_note) => Ok(Json(created_note)),
        None => Err(Status::InternalServerError)
    }
}

#[put("/notes/<id>", data = "<note>")]
async fn update_note(id: &str, note: Json<Note>, state: &State<Arc<AppState>>) -> Result<Json<Note>, Status> {
    let mut updated_note = note.into_inner();
    
    // Ensure ID matches
    match &updated_note.id {
        Some(note_id) if note_id != id => {
            return Err(Status::BadRequest);
        },
        None => {
            updated_note.id = Some(id.to_string());
        },
        _ => {}
    }
    
    // Update timestamp
    updated_note.updated_at = chrono::Utc::now();
    
    let result: Option<Note> = state.db.update(("note", id))
        .content(&updated_note)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    match result {
        Some(note) => Ok(Json(note)),
        None => Err(Status::NotFound)
    }
}

#[delete("/notes/<id>")]
async fn delete_note(id: &str, state: &State<Arc<AppState>>) -> Status {
    let result: bool = state.db.delete(("note", id))
        .await
        .unwrap_or(false);
    
    if result {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

// Options route for CORS preflight requests
#[options("/<_..>")]
fn options() -> Status {
    Status::NoContent
}

#[launch]
async fn rocket() -> _ {
    // Connect to SurrealDB
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to SurrealDB");
    
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to sign in to SurrealDB");
    
    db.use_ns("cosmiqnotz").use_db("cosmiqnotz")
        .await
        .expect("Failed to use namespace and database");
    
    let state = Arc::new(AppState { db });
    
    rocket::build()
        .manage(state)
        .attach(CORS)
        .mount("/api", routes![
            get_notes,
            get_note,
            create_note,
            update_note,
            delete_note,
            options,
        ])
}