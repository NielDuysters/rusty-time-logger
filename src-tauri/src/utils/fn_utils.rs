use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::io::prelude::*;

use super::super::config;

pub fn new_project_if_none() -> std::io::Result<()> {
    match std::fs::read_dir(std::path::Path::new(format!("{}/timelogs", (*config::RUSTY_TIME_LOGGER_PATH)).as_str())) {
        Ok(mut dir) => {
            if dir.next().is_some() {
                return Ok(());
            }
        },
        Err(_) => {}
    }

    let rusty_time_logger_path = &*format!("{}/timelogs/PROJECT", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let project_file_path = std::path::Path::new(rusty_time_logger_path);
    std::fs::create_dir_all(project_file_path.parent().unwrap())?;
    std::fs::File::create(rusty_time_logger_path)?;

    let rusty_initialize_file_path = &*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string());
    let mut rusty_initialize_file = std::fs::File::create(rusty_initialize_file_path)?;
    rusty_initialize_file.write_all("PROJECT".as_bytes())?;

    Ok(())
}

pub fn counter(tx: mpsc::Sender<u64>, is_playing: Arc<(Mutex<bool>, Condvar)>, reset: Arc<Mutex<bool>>) {
    let mut start_time: std::time::Instant = std::time::Instant::now();
    let mut total_pause_seconds: u64 = 0;
    let reset_clone = Arc::clone(&reset);

    loop {
        let mut reset_lock = reset_clone.lock().expect("Could not set lock on reset state");
        if (*reset_lock) == true {
            start_time = std::time::Instant::now();
            total_pause_seconds = 0;
            *reset_lock = false;
        }
        drop(reset_lock);

        let (lock, cvar) = &*is_playing;
        let mut playing = lock.lock().expect("Could not set lock on playing state");
        
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

pub fn get_selected_project() -> Result<String, String> {
    match std::fs::read_to_string(std::path::Path::new(&*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string()))) {
        Ok(project) => Ok(project),
        Err(_) => return Err("Could not select project".to_string()),
    }
}
