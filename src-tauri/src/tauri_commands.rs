use super::services::{project_service, task_service, action_service};

#[tauri::command]
pub fn save(ms: u64, description: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    action_service::save(ms, description, &app_handle)
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
pub fn export_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let export = project.export();
    
    export
}

#[tauri::command]
pub fn load_projects(app_handle: tauri::AppHandle) -> Result<(), String> {
    project_service::refresh(&app_handle)
}

#[tauri::command]
pub fn exit() {
    std::process::exit(0);
}
