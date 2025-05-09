use yew::prelude::*;
use gloo::timers::callback::Interval;
use web_sys::console;

use crate::components::{NoteEditor, NoteList, Toolbar};
use crate::models::Note;
use crate::services::note_service;

pub enum AppMsg {
    LoadNotes,
    NotesLoaded(Result<Vec<Note>, String>),
    SelectNote(Note),
    CreateNote,
    NoteCreated(Result<Note, String>),
    SaveNote(String, String),
    NoteSaved(Result<Note, String>),
    SyncNotes,
    SyncCompleted(Result<(), String>),
    CheckOnline,
    OnlineStatusChanged(bool),
    Error(String),
}

pub struct App {
    notes: Vec<Note>,
    selected_note: Option<Note>,
    is_loading: bool,
    is_syncing: bool,
    is_online: bool,
    error: Option<String>,
    _sync_interval: Option<Interval>,
    _online_interval: Option<Interval>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Start loading notes
        ctx.link().send_message(AppMsg::LoadNotes);
        
        // Check online status
        ctx.link().send_message(AppMsg::CheckOnline);

        // Create intervals for periodic checking
        let sync_interval = {
            let link = ctx.link().clone();
            Interval::new(30_000, move || {
                link.send_message(AppMsg::CheckOnline);
            })
        };
        
        let online_interval = {
            let link = ctx.link().clone();
            Interval::new(5_000, move || {
                link.send_message(AppMsg::CheckOnline);
            })
        };

        Self {
            notes: Vec::new(),
            selected_note: None,
            is_loading: true,
            is_syncing: false,
            is_online: false,
            error: None,
            _sync_interval: Some(sync_interval),
            _online_interval: Some(online_interval),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::LoadNotes => {
                self.is_loading = true;
                // First try to load from API
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match note_service::get_notes().await {
                        Ok(notes) => link.send_message(AppMsg::NotesLoaded(Ok(notes))),
                        Err(_) => {
                            // If API fails, load from local storage
                            match note_service::get_local_notes() {
                                Ok(notes) => link.send_message(AppMsg::NotesLoaded(Ok(notes))),
                                Err(e) => link.send_message(AppMsg::NotesLoaded(Err(e))),
                            }
                        }
                    }
                });
                false
            },
            AppMsg::NotesLoaded(result) => {
                self.is_loading = false;
                match result {
                    Ok(notes) => {
                        self.notes = notes;
                        // Select the first note if none is selected
                        if self.selected_note.is_none() && !self.notes.is_empty() {
                            self.selected_note = Some(self.notes[0].clone());
                        }
                    },
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            },
            AppMsg::SelectNote(note) => {
                self.selected_note = Some(note);
                true
            },
            AppMsg::CreateNote => {
                let user_id = "current_user".to_string(); // In a real app, this would come from auth
                let new_note = Note::new(
                    "Untitled Note".to_string(),
                    "".to_string(),
                    user_id,
                );
                
                self.selected_note = Some(new_note.clone());
                
                // Try to create on API
                if self.is_online {
                    ctx.link().send_message(AppMsg::SaveNote(
                        new_note.title.clone(),
                        new_note.content.clone(),
                    ));
                } else {
                    // Just save locally
                    match note_service::save_note_locally(&new_note) {
                        Ok(_) => {
                            // Add to notes list
                            self.notes.insert(0, new_note);
                        },
                        Err(e) => {
                            self.error = Some(e);
                        }
                    }
                }
                
                true
            },
            AppMsg::SaveNote(title, content) => {
                if let Some(mut note) = self.selected_note.clone() {
                    note.update(title, content);
                    
                    // Always save locally first
                    match note_service::save_note_locally(&note) {
                        Ok(_) => {
                            // Update in notes list
                            if let Some(idx) = self.notes.iter().position(|n| {
                                match (&n.id, &note.id) {
                                    (Some(n_id), Some(note_id)) => n_id == note_id,
                                    _ => false,
                                }
                            }) {
                                self.notes[idx] = note.clone();
                            } else {
                                self.notes.insert(0, note.clone());
                            }
                            
                            self.selected_note = Some(note.clone());
                        },
                        Err(e) => {
                            self.error = Some(e);
                        }
                    }
                    
                    // If online, also save to API
                    if self.is_online {
                        let link = ctx.link().clone();
                        let note_clone = note.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let result = if note_clone.id.is_some() {
                                note_service::update_note(&note_clone).await
                            } else {
                                note_service::create_note(&note_clone).await
                            };
                            link.send_message(AppMsg::NoteSaved(result));
                        });
                    }
                }
                
                true
            },
            AppMsg::NoteSaved(result) => {
                match result {
                    Ok(note) => {
                        // Update in notes list with the server version
                        if let Some(idx) = self.notes.iter().position(|n| {
                            match (&n.id, &note.id) {
                                (Some(n_id), Some(note_id)) => n_id == note_id,
                                _ => false,
                            }
                        }) {
                            self.notes[idx] = note.clone();
                        } else {
                            self.notes.insert(0, note.clone());
                        }
                        
                        self.selected_note = Some(note);
                    },
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                
                true
            },
            AppMsg::SyncNotes => {
                if self.is_online && !self.is_syncing {
                    self.is_syncing = true;
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let result = note_service::process_sync_queue().await;
                        link.send_message(AppMsg::SyncCompleted(result));
                    });
                }
                
                true
            },
            AppMsg::SyncCompleted(result) => {
                self.is_syncing = false;
                
                match result {
                    Ok(_) => {
                        // Reload notes after sync
                        ctx.link().send_message(AppMsg::LoadNotes);
                    },
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                
                true
            },
            AppMsg::CheckOnline => {
                let is_online = note_service::check_online_status();
                ctx.link().send_message(AppMsg::OnlineStatusChanged(is_online));
                false
            },
            AppMsg::OnlineStatusChanged(is_online) => {
                let was_offline = !self.is_online;
                self.is_online = is_online;
                
                // If we just came online, try to sync
                if is_online && was_offline {
                    ctx.link().send_message(AppMsg::SyncNotes);
                }
                
                true
            },
            AppMsg::Error(error) => {
                self.error = Some(error);
                true
            },
            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_note_select = ctx.link().callback(AppMsg::SelectNote);
        let on_note_create = ctx.link().callback(|_| AppMsg::CreateNote);
        let on_note_save = ctx.link().callback(|(title, content)| AppMsg::SaveNote(title, content));
        let on_sync = ctx.link().callback(|_| AppMsg::SyncNotes);

        html! {
            <div class="app">
                <Toolbar 
                    on_sync={on_sync}
                    is_syncing={self.is_syncing}
                    is_online={self.is_online}
                />
                <div class="main-content">
                    <NoteList 
                        notes={self.notes.clone()}
                        on_select={on_note_select}
                        on_create={on_note_create}
                    />
                    <div class="content-area">
                        {
                            if self.is_loading {
                                html! { <div class="loading">{ "Loading..." }</div> }
                            } else if let Some(note) = &self.selected_note {
                                html! { <NoteEditor note={note.clone()} on_save={on_note_save} /> }
                            } else {
                                html! { 
                                    <div class="empty-state">
                                        <h2>{ "Select a note or create a new one" }</h2>
                                    </div> 
                                }
                            }
                        }
                    </div>
                </div>
                {
                    if let Some(error) = &self.error {
                        html! {
                            <div class="error-notification">
                                <p>{ error }</p>
                                <button onclick={ctx.link().callback(|_| AppMsg::Error(String::new()))}>
                                    { "×" }
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}