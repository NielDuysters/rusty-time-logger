#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod services;
mod tauri_commands;
mod utils;

use utils::project;

fn main() {
    project::new_project_if_none().expect("Could not create new project.");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            tauri_commands::save,
            tauri_commands::delete_task,
            tauri_commands::create_new_project,
            tauri_commands::load_projects,
            tauri_commands::delete_project,
            tauri_commands::select_project,
            tauri_commands::export_project,
            tauri_commands::update_finished_tasks,
            tauri_commands::exit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
