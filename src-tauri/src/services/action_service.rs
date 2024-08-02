use tauri::{Manager, State};
use std::sync::{mpsc, Arc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use super::super::tauri_commands;
use super::super::utils::fn_utils;

use super::super::services::task_service;

pub fn play(app_handle: &tauri::AppHandle, is_playing: &State<tauri_commands::IsPlayingState>, first_click: &State<tauri_commands::FirstClickState>, seconds_state: &State<tauri_commands::SecondsState>, reset_state: &State<tauri_commands::ResetState>) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let is_playing_clone = Arc::clone(&is_playing.0);
    let mut first_click_lock = first_click.0.lock().unwrap();
    let seconds_state_clone = Arc::clone(&seconds_state.0);
    let reset_state_clone = Arc::clone(&reset_state.0);
    let app_handle_clone = app_handle.clone();

    let (lock, cvar) = &*is_playing.0;
    let mut playing = lock.lock().expect("Could not set lock on playing state");
    *playing = !*playing;
    if *playing {
        cvar.notify_one();
    }
    
    if *first_click_lock {
        std::thread::spawn(move || {
            fn_utils::counter(tx, is_playing_clone, reset_state_clone);
        });

        std::thread::spawn(move || {
            while let Ok(seconds) = rx.recv() {
                if let Err(_) = app_handle_clone.emit_all("update_time", seconds) {
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
    
    Ok(())
}

pub fn save(description: &str, seconds_state: &State<tauri_commands::SecondsState>, reset_state: &State<tauri_commands::ResetState>, app_handle: &tauri::AppHandle) -> Result<(), String> {
    fn random_id(l: usize) -> String {
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

    let task = task_service::Task::new(&random_id(12));
    let create = task.create(description, *seconds_state_lock);
    *reset_state_lock = true;
    
    task_service::refresh(&app_handle)?;

    create
}
