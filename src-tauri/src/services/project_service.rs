use super::super::config;
use super::super::utils::{csv, export, project};
use super::clients::github_client::GitHubClient;
use super::clients::pm_client::PmClient;
use super::project_config_service::ProjectConfig;
use chrono::{NaiveTime, Timelike};
use std::fs::OpenOptions;
use std::io::prelude::*;
use tauri::Emitter;
use std::sync::{Arc, Mutex};
use crate::utils::project_config::{get_github_issue_nr_from_task_description, get_github_organization_from_url};
use crate::utils::time::seconds_to_his;

#[derive(Clone)]
pub struct Project {
    pub id: String,
    pub config: Option<Arc<Mutex<ProjectConfig>>>,
    path: std::path::PathBuf,
}

impl Project {
    pub fn new(id: &str) -> Self {
        let mut project = Project {
            id: id.to_string().to_uppercase(),
            path: config::RUSTY_TIME_LOGGER_PATH
                .clone()
                .join("timelogs")
                .join(id.to_uppercase()),
            config: None,
        };

        let project_config = ProjectConfig::new(project.clone());
        project.config = Some(Arc::new(Mutex::new(project_config)));

        project
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

        let config = self.config.as_ref().unwrap().lock().map_err(|_| "Mutex lock failed".to_string())?;
        if let Err(_) = config.create() {
            return Err("Couldn't create project config.".to_string());
        }

        Ok(())
    }

    pub fn delete(&self) -> Result<(), String> {
        if let Err(_) = std::fs::remove_file(self.path.as_path()) {
            return Err("Couldn't delete project.".to_string());
        }

        let config = self.config.as_ref().unwrap().lock().map_err(|_| "Mutex lock failed".to_string())?;
        if let Err(_) = config.delete() {
            return Err("Couldn't delete project config.".to_string());
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

    pub async fn sync_spent_time_to_pm(&self) -> Result<(), String> {
        let config = self.get_config()?;
        let (organization, project_number) = get_github_organization_from_url(&config["project_url"].as_str().unwrap())?;
        let client = GitHubClient::builder()
            .auth_token(&config["auth_token"].as_str().unwrap())
            .organization(organization.as_str())
            .project_number(project_number.parse().unwrap())
            .spent_time_field_name(config["spent_time_field_name"].as_str().unwrap())
            .build()
            .map_err(|e| e.to_string())?;

        let seconds_per_task = self.seconds_spent_per_task()?;
        for (task_id, seconds) in seconds_per_task {
            let spent_time = seconds_to_his(seconds);
            let task_nr = get_github_issue_nr_from_task_description(&task_id);
            if task_nr.is_err() {
                continue;
            }
            
            client.set_spent_time(&task_nr?, &spent_time).await.map_err(|e| e.to_string())?;
        }

        let timelog_ticket_nr = config["timelog_ticket_nr"].as_str().unwrap();
        if !timelog_ticket_nr.is_empty() {
            let total_spent_time = seconds_to_his(self.total_seconds_spent()?);
            let task_nr = get_github_issue_nr_from_task_description(&timelog_ticket_nr);
            client.set_spent_time(&task_nr?, &total_spent_time).await.map_err(|e| e.to_string())?;
        }

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
                .split(" ")
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

    pub fn get_config(&self) -> Result<serde_json::Value, String> {
        let config = self.config.as_ref().unwrap().lock().map_err(|_| "Mutex lock failed".to_string())?;
        config.read()
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
        .expect("Failed to emit project list");
    app_handle
        .emit("selected_project", project::get_selected_project()?)
        .expect("Failed to emit selected project");

    let project = Project::new(&project::get_selected_project()?);
    if let Ok(config) = project.get_config() {
        app_handle
            .emit("project_config", config.to_string())
            .expect("Failed to emit project config");
    }

    Ok(())
}
