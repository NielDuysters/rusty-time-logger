use super::services::{action_service, project_service, task_service};

#[tauri::command]
pub fn save(project_id: &str, ms: u64, description: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    action_service::save(project_id, ms, description, &app_handle)
}

#[tauri::command]
pub fn update_finished_tasks(app_handle: tauri::AppHandle) -> Result<(), String> {
    task_service::refresh(&app_handle)
}

#[tauri::command]
pub fn delete_task(project_id: &str, task_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let task = task_service::Task::new(task_id, project_service::Project::new(project_id));
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
pub fn export_project(project_id: &str) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let export = project.export();

    export
}

#[tauri::command]
pub fn load_projects(app_handle: tauri::AppHandle) -> Result<(), String> {
    project_service::refresh(&app_handle)
}

#[tauri::command]
pub fn save_github_settings(
        project_id: &str,
        auth_token: &str,
        project_url: &str,
        spent_time_field_name: &str,
        timelog_ticket_nr: &str,
    ) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let project_config = project.config.as_ref().unwrap().lock().map_err(|_| "Mutex lock failed".to_string())?;
    
    if let Err(_) = project_config.update("auth_token", &auth_token) {
        return Err("Couldn't update auth_token".to_string());
    }

    if let Err(_) = project_config.update("project_url", &project_url) {
        return Err("Couldn't update project_url".to_string());
    }

    if let Err(_) = project_config.update("spent_time_field_name", &spent_time_field_name.to_lowercase()) {
        return Err("Couldn't update spent_time_field_name".to_string());
    }

    if let Err(_) = project_config.update("timelog_ticket_nr", &timelog_ticket_nr) {
        return Err("Couldn't update timelog_ticket_nr".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn sync_spent_time_to_pm(project_id: &str) -> Result<(), String> {
    let project = project_service::Project::new(project_id);
    let sync = project.sync_spent_time_to_pm().await;

    sync
}

#[tauri::command]
pub fn exit() {
    std::process::exit(0);
}
