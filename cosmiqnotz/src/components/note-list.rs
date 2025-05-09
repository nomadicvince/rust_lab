use yew::prelude::*;
use crate::models::Note;

#[derive(Properties, PartialEq)]
pub struct NoteListProps {
    pub notes: Vec<Note>,
    pub on_select: Callback<Note>,
    pub on_create: Callback<()>,
}

#[function_component(NoteList)]
pub fn note_list(props: &NoteListProps) -> Html {
    let on_create = {
        let on_create = props.on_create.clone();
        Callback::from(move |_| {
            on_create.emit(());
        })
    };

    html! {
        <div class="note-list">
            <div class="note-list-header">
                <h2>{ "Notes" }</h2>
                <button onclick={on_create} class="create-button">{ "New Note" }</button>
            </div>
            <div class="note-list-items">
                if props.notes.is_empty() {
                    <div class="empty-list">
                        <p>{ "No notes yet. Create one to get started!" }</p>
                    </div>
                } else {
                    { for props.notes.iter().map(|note| {
                        let on_click = {
                            let note = note.clone();
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                on_select.emit(note.clone());
                            })
                        };
                        
                        html! {
                            <div class="note-item" onclick={on_click}>
                                <h3 class="note-title">{ &note.title }</h3>
                                <p class="note-date">
                                    { format!("Updated: {}", note.updated_at.format("%Y-%m-%d %H:%M")) }
                                </p>
                            </div>
                        }
                    }) }
                }
            </div>
        </div>
    }
}