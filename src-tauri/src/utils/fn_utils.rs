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

pub fn get_selected_project() -> Result<String, String> {
    match std::fs::read_to_string(std::path::Path::new(&*format!("{}/.selected-project", (*config::RUSTY_TIME_LOGGER_PATH).to_string()))) {
        Ok(project) => Ok(project),
        Err(_) => return Err("Could not select project".to_string()),
    }
}
