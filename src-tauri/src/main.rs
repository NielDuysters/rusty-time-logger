#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, Condvar};

mod config;
mod csv;
mod tauri_commands;
mod utils;


fn main() {
    utils::new_project_if_none().expect("Could not create new project.");

    let is_playing = Arc::new((Mutex::new(false), Condvar::new()));
    let first_click = Arc::new(Mutex::new(true));
    let seconds = Arc::new(Mutex::new(0));
    let reset = Arc::new(Mutex::new(false));

    tauri::Builder::default()
        .manage(tauri_commands::IsPlayingState(is_playing))
        .manage(tauri_commands::FirstClickState(first_click))
        .manage(tauri_commands::SecondsState(seconds))
        .manage(tauri_commands::ResetState(reset))
        .invoke_handler(tauri::generate_handler![tauri_commands::play, tauri_commands::save, tauri_commands::update_finished_tasks, tauri_commands::delete_task, tauri_commands::create_new_project, tauri_commands::load_projects, tauri_commands::delete_project, tauri_commands::select_project, tauri_commands::exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
