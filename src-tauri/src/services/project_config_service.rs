use super::{super::config, project_service::Project};
use serde_json::json;


pub struct ProjectConfig {
    path: std::path::PathBuf,
}

impl ProjectConfig {
    pub fn new(project: Project) -> Self {
        Self {
            path: config::RUSTY_TIME_LOGGER_PATH
                .clone()
                .join("project-configs")
                .join(project.id.to_string().to_uppercase())
                .with_extension("json"),
        }
    }

    pub fn create(&self) -> Result<(), String> {
        if self.path.as_path().exists() {
            return Err("A project-config with this name already exists.".to_string());
        }

        if let Err(_) = std::fs::create_dir_all(self.path.parent().unwrap()) {
            return Err("Couldn't create project-config.".to_string());
        }

        if let Err(_) = std::fs::File::create(self.path.as_path()) {
            return Err("Couldn't create project-config.".to_string());
        }

        let default_config = json!({
            "auth_token": "",
            "project_url": "",
            "spent_time_field_name": "SPENT TIME",
            "timelog_ticket_nr": "",
        });

        if let Err(_) = std::fs::write(self.path.as_path(), serde_json::to_string_pretty(&default_config).unwrap()) {
            return Err("Couldn't write default config to project-config.".to_string());
        }

        Ok(())
    }

    pub fn delete(&self) -> Result<(), String> {
        if let Err(_) = std::fs::remove_file(self.path.as_path()) {
            return Err("Couldn't delete project-config.".to_string());
        }

        Ok(())
    }

    pub fn read(&self) -> Result<serde_json::Value, String> {
        let file = match std::fs::File::open(&self.path) {
            Ok(file) => file,
            Err(_) => return Err("Could not open project-config file to read".to_string()),
        };
        let reader = std::io::BufReader::new(file);
        let config: serde_json::Value = serde_json::from_reader(reader).unwrap();
        
        Ok(config)
    }

    pub fn update(&self, key: &str, value: &str) -> Result<(), String> {
        let mut config = self.read().unwrap();
        config[key] = value.into();

        if let Err(_) = std::fs::write(self.path.as_path(), serde_json::to_string_pretty(&config).unwrap()) {
            return Err("Couldn't write update to project-config.".to_string());
        }

        Ok(())
    }
} 
