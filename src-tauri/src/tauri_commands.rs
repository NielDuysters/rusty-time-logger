use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tauri::{Manager, State};
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::{mpsc, Arc, Mutex, Condvar};

use super::csv;
use super::config;
use super::utils;

pub struct IsPlayingState(pub Arc<(Mutex<bool>, Condvar)>);
pub struct FirstClickState(pub Arc<Mutex<bool>>);
pub struct SecondsState(pub Arc<Mutex<u64>>);
pub struct ResetState(pub Arc<Mutex<bool>>);


#[tauri::command]
pub fn play(app_handle: tauri::AppHandle, is_playing: State<IsPlayingState>, first_click: State<FirstClickState>, seconds_state: State<SecondsState>, reset_state: State<ResetState>) {
    let (tx, rx) = mpsc::channel();
    let is_playing_clone = Arc::clone(&is_playing.0);
    let mut first_click_lock = first_click.0.lock().unwrap();
    let seconds_state_clone = Arc::clone(&seconds_state.0);
    let reset_state_clone = Arc::clone(&reset_state.0);

    let (lock, cvar) = &*is_playing.0;
    let mut playing = lock.lock().expect("Could not set lock on playing state");
    *playing = !*playing;
    if *playing {
        cvar.notify_one();
    }
    
    if *first_click_lock {
        std::thread::spawn(move || {
            utils::counter(tx, is_playing_clone, reset_state_clone);
        });

        std::thread::spawn(move || {
            while let Ok(seconds) = rx.recv() {
                if let Err(_) = app_handle.emit_all("update_time", seconds) {
                    continue;
                }
                let mut seconds_state_lock = match seconds_state_clone.lock() {
                    Ok(ssl) => ssl,
                    Err(_) => continue,
                };
                *seconds_state_lock = seconds;
            }
        });

        *first_click_lock = false;
    }
}

#[tauri::command]
pub fn save(task_description: &str, seconds_state: State<SecondsState>, reset_state: State<ResetState>, app_handle: tauri::AppHandle) -> Result<(), String> {
    pub fn random_id(l: usize) -> String {
        let mut rng = thread_rng();
        (0..l).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    let seconds_state_lock = match seconds_state.0.lock() {
        Ok(ssl) => ssl,
        Err(_) => return Err("Error saving task".to_string()),
    };
    let mut reset_state_lock = match reset_state.0.lock() {
        Ok(rsl) => rsl,
        Err(_) => return Err("Error saving task".to_string()),
    };

    let today = Utc::now();
    let date_str = format!("{} {}", today.day(), today.format("%B").to_string());

    if let Err(_) = csv::save(utils::get_selected_project()?.as_str(), &random_id(12), &*date_str, task_description, *seconds_state_lock) {
        return Err("Error saving CSV file".to_string());
    };
    *reset_state_lock = true;
    
    update_finished_tasks(app_handle)?;

    Ok(())
}

#[tauri::command]
pub fn update_finished_tasks(app_handle: tauri::AppHandle) -> Result<(), String> {
    let tasks : std::vec::Vec<std::vec::Vec<String>> = csv::read(utils::get_selected_project()?.as_str())?;
    let tasks_json = match serde_json::to_string(&tasks) {
        Ok(json) => json,
        Err(_) => return Err("Error reading tasks from project file".to_string()),
    };
    if let Err(_) = app_handle.emit_all("finished_tasks", tasks_json) {
        return Err("Failed to emit tasks".to_string());
    };

    Ok(())
}

#[tauri::command]
pub fn delete_task(task_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    csv::delete(utils::get_selected_project()?.as_str(), task_id)?;
    update_finished_tasks(app_handle)?;

    Ok(())
}

#[tauri::command]
pub fn load_projects(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut project_files : std::vec::Vec<String> = std::vec::Vec::new();
    let project_directory = &*format!("{}/timelogs", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let project_directory_content = match std::fs::read_dir(project_directory) {
        Ok(content) => content,
        Err(_) => return Err("Error reading projects from timelogs directory.".to_string()),
    };
    for file in project_directory_content {
        project_files.push(file.unwrap().file_name().to_string_lossy().to_string());
    }
    
    let project_files_json = serde_json::to_string(&project_files).expect("Failed to serialize project files");
    app_handle.emit_all("project_list", project_files_json).expect("Failed to emit project list");
    app_handle.emit_all("selected_project", utils::get_selected_project()?).expect("Failed to emit selected project");

    Ok(())
}

#[tauri::command]
pub fn create_new_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id.to_uppercase());
    let project_file_path = std::path::Path::new(&*project_file_path);

    if project_file_path.exists() {
        return Err("A project with this name already exists.".to_string());
    }

    if let Err(_) = std::fs::create_dir_all(project_file_path.parent().unwrap()) {
        return Err("Couldn't create project.".to_string());
    }

    if let Err(_) = std::fs::File::create(project_file_path) {
        return Err("Couldn't create project.".to_string());
    }

    load_projects(app_handle)?;
    
    Ok(())
}

#[tauri::command]
pub fn delete_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let project_file_path = std::path::Path::new(&*project_file_path);
    
    if let Err(_) = std::fs::remove_file(project_file_path) {
        return Err("Couldn't delete project.".to_string());
    }
    
    load_projects(app_handle)?;
    
    Ok(())
}

#[tauri::command]
pub fn select_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let selected_project_file_path = &*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let selected_project_file_path = std::path::Path::new(selected_project_file_path);
    let mut selected_project_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(selected_project_file_path) {
            Ok(file) => file,
            Err(_) => return Err("Couldn't open selected project file".to_string()),
        };
    
    if let Err(_) = selected_project_file.write_all(project_id.as_bytes()) {
        return Err("Couldn't set selected project".to_string());
    };
    
    update_finished_tasks(app_handle.clone())?;
    load_projects(app_handle)?;

    Ok(())
}


#[tauri::command]
pub fn exit() {
    std::process::exit(0);
}
