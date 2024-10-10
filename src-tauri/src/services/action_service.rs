use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::super::services::task_service;

pub fn save(ms: u64, description: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    fn random_id(l: usize) -> String {
        let mut rng = thread_rng();
        (0..l).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    let task = task_service::Task::new(&random_id(12));
    let create = task.create(description, ms / 1000);

    task_service::refresh(&app_handle)?;

    create
}
