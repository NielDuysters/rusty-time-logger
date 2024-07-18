#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{mpsc, Arc, Mutex, Condvar};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tauri::{Manager, State};
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod config;
mod csv;

struct IsPlayingState(Arc<(Mutex<bool>, Condvar)>);
struct FirstClickState(Arc<Mutex<bool>>);
struct SecondsState(Arc<Mutex<u64>>);
struct ResetState(Arc<Mutex<bool>>);

fn new_project_if_none() {
    match std::fs::read_dir(std::path::Path::new(&config::RUSTY_TIME_LOGGER_PATH.as_str())) {
        Ok(mut dir) => {
            if dir.next().is_some() {
                return;
            }
        },
        Err(_) => {}
    }

    let rusty_time_logger_path = &*format!("{}/timelogs/PROJECT", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let project_file_path = std::path::Path::new(rusty_time_logger_path);
    std::fs::create_dir_all(project_file_path.parent().unwrap()).expect("error");
    std::fs::File::create(rusty_time_logger_path).expect("error");

    let rusty_initialize_file_path = &*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let mut rusty_initialize_file = std::fs::File::create(rusty_initialize_file_path).expect("error");
    rusty_initialize_file.write_all("PROJECT".as_bytes()).expect("error");

}

fn counter(tx: mpsc::Sender<u64>, is_playing: Arc<(Mutex<bool>, Condvar)>, reset: Arc<Mutex<bool>>) {
    let mut start_time: std::time::Instant = std::time::Instant::now();
    let mut total_pause_seconds: u64 = 0;
    let reset_clone = Arc::clone(&reset);

    loop {

        let mut reset_lock = reset_clone.lock().unwrap();
        if (*reset_lock) == true {
            start_time = std::time::Instant::now();
            total_pause_seconds = 0;
            *reset_lock = false;
        }
        drop(reset_lock);

        let (lock, cvar) = &*is_playing;
        let mut playing = lock.lock().unwrap();
        
        while !*playing {
            let pause_time: std::time::Instant = std::time::Instant::now();
            playing = cvar.wait(playing).unwrap();
            total_pause_seconds += (std::time::Instant::now()).duration_since(pause_time).as_secs();
        }

        if *playing {
            let seconds: u64 = (std::time::Instant::now()).duration_since(start_time).as_secs() - total_pause_seconds;
            tx.send(seconds).expect("Failed to transmit seconds");
            std::thread::sleep(std::time::Duration::from_millis(35));
        }
    }
}

#[tauri::command]
fn play(app_handle: tauri::AppHandle, is_playing: State<IsPlayingState>, first_click: State<FirstClickState>, seconds_state: State<SecondsState>, reset_state: State<ResetState>) {
    let (tx, rx) = mpsc::channel();
    let is_playing_clone = Arc::clone(&is_playing.0);
    let mut first_click_lock = first_click.0.lock().unwrap();
    let seconds_state_clone = Arc::clone(&seconds_state.0);
    let reset_state_clone = Arc::clone(&reset_state.0);

    let (lock, cvar) = &*is_playing.0;
    let mut playing = lock.lock().unwrap();
    *playing = !*playing;
    if *playing {
        cvar.notify_one();
    }
    
    if *first_click_lock {
        std::thread::spawn(move || {
            counter(tx, is_playing_clone, reset_state_clone);
        });

        std::thread::spawn(move || {
            while let Ok(seconds) = rx.recv() {
                app_handle.emit_all("update_time", seconds).expect("Failed to emit seconds");
                let mut seconds_state_lock = seconds_state_clone.lock().unwrap();
                *seconds_state_lock = seconds;
            }
        });

        *first_click_lock = false;
    }
}

#[tauri::command]
fn save(task_description: &str, seconds_state: State<SecondsState>, reset_state: State<ResetState>, app_handle: tauri::AppHandle) {
    fn random_id(l: usize) -> String {
        let mut rng = thread_rng();
        (0..l).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    let seconds_state_lock = seconds_state.0.lock().unwrap();
    let mut reset_state_lock = reset_state.0.lock().unwrap();

    let today = Utc::now();
    let date_str = format!("{} {}", today.day(), today.format("%B").to_string());

    csv::save(get_selected_project().as_str(), &random_id(12), &*date_str, task_description, *seconds_state_lock);
    *reset_state_lock = true;
    
    update_finished_tasks(app_handle);
}

#[tauri::command]
fn update_finished_tasks(app_handle: tauri::AppHandle) {
    let tasks : std::vec::Vec<std::vec::Vec<String>> = csv::read(get_selected_project().as_str());
    let tasks_json = serde_json::to_string(&tasks).expect("Failed to serialize tasks");
    app_handle.emit_all("finished_tasks", tasks_json).expect("Failed to emit tasks");
}

#[tauri::command]
fn delete_task(task_id: &str, app_handle: tauri::AppHandle) {
    csv::delete(get_selected_project().as_str(), task_id);
    update_finished_tasks(app_handle);
}

#[tauri::command]
fn load_projects(app_handle: tauri::AppHandle) {
    let mut project_files : std::vec::Vec<String> = std::vec::Vec::new();
    let project_directory = &*format!("{}/timelogs", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    for file in std::fs::read_dir(project_directory).unwrap() {
        project_files.push(file.unwrap().file_name().to_string_lossy().to_string());
    }
    
    let project_files_json = serde_json::to_string(&project_files).expect("Failed to serialize project files");
    app_handle.emit_all("project_list", project_files_json).expect("Failed to emit project list");
    app_handle.emit_all("selected_project", get_selected_project()).expect("Failed to emit selected project");
}

#[tauri::command]
fn create_new_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
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

    load_projects(app_handle);
    Ok(())
}

#[tauri::command]
fn delete_project(project_id: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    let project_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let project_file_path = std::path::Path::new(&*project_file_path);
    
    if let Err(_) = std::fs::remove_file(project_file_path) {
        return Err("Couldn't delete project.".to_string());
    }
    
    load_projects(app_handle);
    Ok(())
}

#[tauri::command]
fn select_project(project_id: &str, app_handle: tauri::AppHandle) {
    let selected_project_file_path = &*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let selected_project_file_path = std::path::Path::new(selected_project_file_path);
    let mut selected_project_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(selected_project_file_path).expect("Couldn't set selected project");
    selected_project_file.write_all(project_id.as_bytes()).expect("Couldn't set selected project");
    
    update_finished_tasks(app_handle.clone());
    load_projects(app_handle);
}

fn get_selected_project() -> String {
    std::fs::read_to_string(std::path::Path::new(&*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string()))).expect("Couldn't get selected project")
}

fn main() {
    new_project_if_none();

    let is_playing = Arc::new((Mutex::new(false), Condvar::new()));
    let first_click = Arc::new(Mutex::new(true));
    let seconds = Arc::new(Mutex::new(0));
    let reset = Arc::new(Mutex::new(false));

    tauri::Builder::default()
        .manage(IsPlayingState(is_playing))
        .manage(FirstClickState(first_click))
        .manage(SecondsState(seconds))
        .manage(ResetState(reset))
        .invoke_handler(tauri::generate_handler![play, save, update_finished_tasks, delete_task, create_new_project, load_projects, delete_project, select_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
