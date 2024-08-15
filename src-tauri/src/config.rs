use once_cell::sync::Lazy;
use dirs;

pub static RUSTY_TIME_LOGGER_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| dirs::home_dir().unwrap().join(".rustytimelogger"));

pub static SELECTED_PROJECT_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| RUSTY_TIME_LOGGER_PATH.join(".selected-project"));

pub static HTML_EXPORT_TEMPLATE: &str = include_str!("assets/html-export.html");
