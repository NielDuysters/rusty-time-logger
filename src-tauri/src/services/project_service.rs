use super::super::config;
use super::super::utils::{csv, export, project};
use chrono::{NaiveTime, Timelike};
use std::fs::OpenOptions;
use std::io::prelude::*;
use tauri::Emitter;

pub struct Project {
    pub id: String,
    path: std::path::PathBuf,
}

impl Project {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string().to_uppercase(),
            path: config::RUSTY_TIME_LOGGER_PATH
                .clone()
                .join("timelogs")
                .join(id.to_uppercase()),
        }
    }

    pub fn create(&self) -> Result<(), String> {
        if self.path.as_path().exists() {
            return Err("A project with this name already exists.".to_string());
        }

        if let Err(_) = std::fs::create_dir_all(self.path.parent().unwrap()) {
            return Err("Couldn't create project.".to_string());
        }

        if let Err(_) = std::fs::File::create(self.path.as_path()) {
            return Err("Couldn't create project.".to_string());
        }

        Ok(())
    }

    pub fn delete(&self) -> Result<(), String> {
        if let Err(_) = std::fs::remove_file(self.path.as_path()) {
            return Err("Couldn't delete project.".to_string());
        }

        Ok(())
    }

    pub fn select(&self) -> Result<(), String> {
        let mut selected_project_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config::SELECTED_PROJECT_PATH.as_path())
        {
            Ok(file) => file,
            Err(_) => return Err("Couldn't open selected project file".to_string()),
        };

        if let Err(_) = selected_project_file.write_all(self.id.as_bytes()) {
            return Err("Couldn't set selected project".to_string());
        };

        Ok(())
    }

    pub fn export(&self) -> Result<(), String> {
        export::export_to_html(&self);
        Ok(())
    }

    pub fn total_seconds_spent(&self) -> Result<u32, String> {
        let mut total = 0;
        for task in self.tasks().unwrap() {
            let time_str = task.get(3).unwrap();
            if let Ok(naive_time) = NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
                let seconds =
                    naive_time.hour() * 3600 + naive_time.minute() * 60 + naive_time.second();
                total += seconds;
            } else {
                eprintln!("Failed to parse time: {}", time_str);
            }
        }

        Ok(total)
    }

    pub fn seconds_spent_per_task(&self) -> Result<std::collections::HashMap<String, u32>, String> {
        let mut seconds_per_task: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();

        for task in self.tasks().unwrap() {
            let task_description = task.get(2).unwrap();
            let task_id = task_description
                .split(" - ")
                .next()
                .unwrap_or(task_description);
            let time_str = task.get(3).unwrap();
            if let Ok(naive_time) = NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
                let seconds =
                    naive_time.hour() * 3600 + naive_time.minute() * 60 + naive_time.second();
                let entry = seconds_per_task.entry(task_id.to_string()).or_insert(0);
                *entry += seconds;
            } else {
                eprintln!("Failed to parse time: {}", time_str);
            }
        }

        Ok(seconds_per_task)
    }

    pub fn tasks(&self) -> Result<std::vec::Vec<std::vec::Vec<String>>, String> {
        csv::read(&self.id)
    }
}

pub fn refresh(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let mut project_files: std::vec::Vec<String> = std::vec::Vec::new();
    let project_directory = config::RUSTY_TIME_LOGGER_PATH.join("timelogs");
    let project_directory_content = match std::fs::read_dir(project_directory) {
        Ok(content) => content,
        Err(_) => return Err("Error reading projects from timelogs directory.".to_string()),
    };
    for file in project_directory_content {
        project_files.push(file.unwrap().file_name().to_string_lossy().to_string());
    }

    let project_files_json =
        serde_json::to_string(&project_files).expect("Failed to serialize project files");
    app_handle
        .emit("project_list", project_files_json)
        .expect("Failed to emit_all project list");
    app_handle
        .emit("selected_project", project::get_selected_project()?)
        .expect("Failed to emit_all selected project");

    Ok(())
}
