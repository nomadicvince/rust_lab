use yew::prelude::*;

mod components;
mod models;
mod services;
mod app;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_log::init().expect("Failed to initialize console logger");
    yew::Renderer::<App>::new().render();
}