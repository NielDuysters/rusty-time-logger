use std::fs::OpenOptions;
use std::io::prelude::*;

use super::super::config;
use super::super::utils::time;

pub fn save(project_id: &str, row_id: &str, date: &str, task_description: &str, seconds: u64) -> std::io::Result<()> {
    let csv_file_path = config::RUSTY_TIME_LOGGER_PATH.join("timelogs").join(project_id.to_uppercase());
    std::fs::create_dir_all(csv_file_path.parent().unwrap())?;

    let mut csv_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(csv_file_path)?;

    let time_string = time::seconds_to_his(seconds as u32);

    writeln!(csv_file, "{},{},{},{}", row_id, date, task_description, time_string)?;

    Ok(())
}

pub fn read(project_id: &str) -> Result<std::vec::Vec<std::vec::Vec<String>>, String> {
    let csv_file_path = config::RUSTY_TIME_LOGGER_PATH.join("timelogs").join(project_id.to_uppercase());
    let csv_file = match std::fs::File::open(&*csv_file_path) {
        Ok(file) => file,
        Err(_) => return Err("Could not open CSV-file to read".to_string()),
    };
    let reader = std::io::BufReader::new(csv_file);

    let mut csv_content : std::vec::Vec<std::vec::Vec<String>> = std::vec::Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let line_parts = content.split(',').map(|s| s.to_string()).collect();
                csv_content.push(line_parts); 
            },
            Err(_) => return Err("Error reading line from CSV".to_string()),
        }
    }
        
    Ok(csv_content)
}

pub fn delete(project_id: &str, task_id: &str) -> Result<(), String> {
    let csv_file_path = config::RUSTY_TIME_LOGGER_PATH.join("timelogs").join(project_id.to_uppercase());
    let csv_file = match std::fs::File::open(&*csv_file_path) {
        Ok(file) => file,
        Err(_) => return Err("Could not open CSV-file to delete".to_string()),
    };
    let reader = std::io::BufReader::new(csv_file);

    let temp_file_path = config::RUSTY_TIME_LOGGER_PATH.join("timelogs").join(format!(".{}.tmp", project_id.to_uppercase()));
    let temp_file = match std::fs::File::create(temp_file_path.clone()) {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not create temporary file to delete {}", e)),
    };
    let mut writer = std::io::BufWriter::new(temp_file);

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let line_parts : std::vec::Vec<String> = content.split(',').map(|s| s.to_string()).collect();
                if line_parts[0] == task_id {
                    continue;
                }

                if let Err(_) = writeln!(writer, "{}", line_parts.join(",")) {
                    return Err("Could not write line to temporary file to delete".to_string());
                };
            },
            Err(_) => return Err("Error reading line from CSV to delete".to_string()),
        }
    }

    writer.flush().unwrap();

    if let Err(_) = std::fs::rename(temp_file_path, csv_file_path) {
        return Ok(());
    };

    Ok(())
}
