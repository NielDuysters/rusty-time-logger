use super::super::config;
use super::super::services::project_service;
use super::super::utils::time;
use dirs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn export_to_html(project: &project_service::Project) {
    let mut html = config::HTML_EXPORT_TEMPLATE.to_string();
    html = html.replace("%project_id%", &*project.id);
    html = html.replace(
        "%total_time_spent%",
        &time::seconds_to_his(project.total_seconds_spent().unwrap()),
    );

    let tbody_time_per_task = project
        .seconds_spent_per_task()
        .unwrap()
        .iter()
        .map(|(task, time)| {
            format!(
                "<tr><td>{}</td><td>{}</td></tr>",
                task,
                time::seconds_to_his(*time)
            )
        })
        .collect::<String>();
    html = html.replace("%tbody_time_per_task%", &*tbody_time_per_task);

    let tbody_tasks_log = project
        .tasks()
        .unwrap()
        .iter()
        .rev()
        .map(|task| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                task.get(1).unwrap(),
                task.get(2).unwrap(),
                task.get(3).unwrap()
            )
        })
        .collect::<String>();
    html = html.replace("%tbody_tasks_log%", &*tbody_tasks_log);

    if let Some(download_dir) = dirs::download_dir() {
        let export_file_path =
            download_dir.join(format!("rusty-time-logger-export-{}.html", project.id));

        let mut export_file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .truncate(true)
            .open(export_file_path)
            .expect("Failed to create export file.");

        writeln!(export_file, "{}", html).expect("Failed to write to export file.");
    } else {
        eprintln!("Couldn't get Download-directory.");
    }
}
