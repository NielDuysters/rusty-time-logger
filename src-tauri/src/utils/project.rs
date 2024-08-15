use std::io::prelude::*;
use super::super::config;

pub fn new_project_if_none() -> std::io::Result<()> {
    match std::fs::read_dir((*config::RUSTY_TIME_LOGGER_PATH).join("timelogs")) {
        Ok(mut dir) => {
            if dir.next().is_some() {
                return Ok(());
            }
        },
        Err(_) => {}
    }

    let project_file_path = config::RUSTY_TIME_LOGGER_PATH.join("timelogs/PROJECT");
    std::fs::create_dir_all(project_file_path.parent().unwrap())?;
    std::fs::File::create(project_file_path)?;

    let mut rusty_initialize_file = std::fs::File::create(config::SELECTED_PROJECT_PATH.clone())?;
    rusty_initialize_file.write_all("PROJECT".as_bytes())?;

    Ok(())
}

pub fn get_selected_project() -> Result<String, String> {
    match std::fs::read_to_string(config::SELECTED_PROJECT_PATH.clone()) {
        Ok(project) => Ok(project),
        Err(_) => return Err("Could not select project".to_string()),
    }
}
