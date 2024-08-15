use tauri::Manager;
use chrono::prelude::*;
use super::super::utils::{project, csv};

pub struct Task {
    id: String
}

impl Task {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
    
    pub fn create(&self, task_description: &str, seconds: u64) -> Result<(), String> {
        let today = Utc::now();
        let date_str = today.format("%e %B").to_string();

        if let Err(_) = csv::save(project::get_selected_project()?.as_str(), &self.id, &*date_str, task_description, seconds) {
            return Err("Error saving CSV file".to_string());
        };

        Ok(())
    }

    pub fn delete(&self) -> Result<(), String> {
        csv::delete(project::get_selected_project()?.as_str(), &self.id)
    }
    
}

pub fn refresh(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let tasks : std::vec::Vec<std::vec::Vec<String>> = csv::read(project::get_selected_project()?.as_str())?;
    let tasks_json = match serde_json::to_string(&tasks) {
        Ok(json) => json,
        Err(_) => return Err("Error reading tasks from project file".to_string()),
    };
    if let Err(_) = app_handle.emit_all("finished_tasks", tasks_json) {
        return Err("Failed to emit_all tasks".to_string());
    };

    Ok(())
}
