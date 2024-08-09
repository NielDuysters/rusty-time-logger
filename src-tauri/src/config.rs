use once_cell::sync::Lazy;

pub static RUSTY_TIME_LOGGER_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| std::path::PathBuf::from("../../.rusty-time-logger"));

pub static SELECTED_PROJECT_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| RUSTY_TIME_LOGGER_PATH.join(".selected-project"));
