use tauri::State;
use std::sync::{Arc, Mutex, Condvar};
use super::services::{project_service, task_service, action_service};

pub struct IsPlayingState(pub Arc<(Mutex<bool>, Condvar)>);
pub struct FirstClickState(pub Arc<Mutex<bool>>);
pub struct SecondsState(pub Arc<Mutex<u64>>);
pub struct ResetState(pub Arc<Mutex<bool>>);

#[tauri::command]
pub fn play(app_handle: tauri::AppHandle, is_playing: State<IsPlayingState>, first_click: State<FirstClickState>, seconds_state: State<SecondsState>, reset_state: State<ResetState>) -> Result<(), String> {
    action_service::play(&app_handle, &is_playing, &first_click, &seconds_state, &reset_state)
}

#[tauri::command]
pub fn save(description: &str, seconds_state: State<SecondsState>, reset_state: State<ResetState>, app_handle: tauri::AppHandle) -> Result<(), String> {
    action_service::save(description, &seconds_state, &reset_state, &app_handle)
}

#[tauri::command]
pub fn update_finished_tasks(app_handle: tauri::AppHandle) -> Result<(), String> {
    task_service::refresh(&app_handle)
}

#[tauri::command]
pub fn delete_task(task_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let task = task_service::Task::new(task_id);
    let delete = task.delete();
    task_service::refresh(&app_handle)?;
    
    delete
}

#[tauri::command]
pub fn create_new_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let create = project.create();
    project_service::refresh(&app_handle)?;
    
    create
}

#[tauri::command]
pub fn delete_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let delete = project.delete();
    project_service::refresh(&app_handle)?;
    
    delete

}

#[tauri::command]
pub fn select_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let select = project.select();
    project_service::refresh(&app_handle)?;
    task_service::refresh(&app_handle)?;
    
    select
}

#[tauri::command]
pub fn load_projects(app_handle: tauri::AppHandle) -> Result<(), String> {
    project_service::refresh(&app_handle)
}

#[tauri::command]
pub fn exit() {
    std::process::exit(0);
}
