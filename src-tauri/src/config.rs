use once_cell::sync::Lazy;

pub static RUSTY_TIME_LOGGER_PATH: Lazy<String> = Lazy::new(|| "../../.rusty-time-logger".to_string());

pub static SELECTED_PROJECT_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| std::path::PathBuf::from(format!("{}/.selected-project", (*RUSTY_TIME_LOGGER_PATH))));
