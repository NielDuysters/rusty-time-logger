use dirs;
use once_cell::sync::Lazy;

pub static RUSTY_TIME_LOGGER_PATH: Lazy<std::path::PathBuf> =
    Lazy::new(|| dirs::home_dir().unwrap().join(".rustytimelogger"));

pub static SELECTED_PROJECT_PATH: Lazy<std::path::PathBuf> =
    Lazy::new(|| RUSTY_TIME_LOGGER_PATH.join(".selected-project"));

pub static HTML_EXPORT_TEMPLATE: &str = include_str!("assets/html-export.html");

pub static GITHUB_PROJECT_API_URL: Lazy<String> =
    Lazy::new(|| "https://api.github.com/graphql".to_string());
