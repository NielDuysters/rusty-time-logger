use std::fs::OpenOptions;
use std::io::prelude::*;

use super::config;

pub fn save(project_id: &str, row_id: &str, date: &str, task_description: &str, seconds: u64) {
    let csv_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let csv_file_path = std::path::Path::new(&*csv_file_path);
    std::fs::create_dir_all(csv_file_path.parent().unwrap()).unwrap();

    let mut csv_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(csv_file_path)
        .unwrap();

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let time_string = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);


    if let Err(e) = writeln!(csv_file, "{},{},{},{}", row_id, date, task_description, time_string) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn read(project_id: &str) -> std::vec::Vec<std::vec::Vec<String>> {
    let csv_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let csv_file = std::fs::File::open(&*csv_file_path).expect("Couldn't open file");
    let reader = std::io::BufReader::new(csv_file);

    let mut csv_content : std::vec::Vec<std::vec::Vec<String>> = std::vec::Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let line_parts = content.split(',').map(|s| s.to_string()).collect();
                csv_content.push(line_parts); 
            },
            Err(e) => eprintln!("Error reading line from csv: {}", e),
        }
    }
        
    csv_content
}

pub fn delete(project_id: &str, task_id: &str) {
    let csv_file_path = &*format!("{}/timelogs/{}", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let csv_file = std::fs::File::open(&*csv_file_path).expect("Couldn't open file");
    let reader = std::io::BufReader::new(csv_file);

    let temp_file_path = &*format!("{}/timelogs/.{}.tmp", (*config::RUSTY_TIME_LOGGER_PATH).to_string(), project_id);
    let temp_file = std::fs::File::create(&temp_file_path).expect("Couldn't create temporary file");
    let mut writer = std::io::BufWriter::new(temp_file);


    for line in reader.lines() {
        match line {
            Ok(content) => {
                let line_parts : std::vec::Vec<String> = content.split(',').map(|s| s.to_string()).collect();
                if line_parts[0] == task_id {
                    continue;
                }

                writeln!(writer, "{}", line_parts.join(",")).expect("Couldn't write to temporary file");
            },
            Err(e) => eprintln!("Error reading line from csv: {}", e),
        }
    }

    writer.flush().unwrap();

    std::fs::rename(temp_file_path, csv_file_path).expect("Couldn't rename temp file to csv file");
}
