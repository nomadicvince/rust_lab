use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use wasm_bindgen::JsCast;
use crate::models::Note;

#[derive(Properties, PartialEq)]
pub struct NoteEditorProps {
    pub note: Note,
    pub on_save: Callback<(String, String)>,
}

#[function_component(NoteEditor)]
pub fn note_editor(props: &NoteEditorProps) -> Html {
    let title = use_state(|| props.note.title.clone());
    let content = use_state(|| props.note.content.clone());
    let is_dirty = use_state(|| false);

    // Update states when note changes
    {
        let title = title.clone();
        let content = content.clone();
        let is_dirty = is_dirty.clone();
        
        use_effect_with_deps(move |note| {
            title.set(note.title.clone());
            content.set(note.content.clone());
            is_dirty.set(false);
            || ()
        }, props.note.clone());
    }

    let on_title_change = {
        let title = title.clone();
        let is_dirty = is_dirty.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
            is_dirty.set(true);
        })
    };

    let on_content_change = {
        let content = content.clone();
        let is_dirty = is_dirty.clone();
        
        Callback::from(move |e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(textarea.value());
            is_dirty.set(true);
        })
    };

    let on_save_click = {
        let title = title.clone();
        let content = content.clone();
        let on_save = props.on_save.clone();
        let is_dirty = is_dirty.clone();
        
        Callback::from(move |_| {
            on_save.emit(((*title).clone(), (*content).clone()));
            is_dirty.set(false);
        })
    };

    html! {
        <div class="note-editor">
            <div class="editor-header">
                <input 
                    type="text"
                    class="title-input"
                    placeholder="Note title"
                    value={(*title).clone()}
                    onchange={on_title_change}
                />
                <button 
                    onclick={on_save_click}
                    disabled={!*is_dirty}
                    class={classes!("save-button", (!*is_dirty).then(|| "disabled"))}
                >
                    { "Save" }
                </button>
            </div>
            <div class="editor-content">
                <textarea 
                    class="content-textarea"
                    placeholder="Write your note here..."
                    value={(*content).clone()}
                    onchange={on_content_change}
                />
            </div>
        </div>
    }
}