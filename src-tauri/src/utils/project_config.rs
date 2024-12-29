use regex::Regex;

pub fn get_github_organization_from_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim_end_matches('/');
    let url_parts: Vec<&str> = url.split('/').collect();
    if url_parts.len() < 3 {
        return Err("Invalid URL".to_string());
    }

    let organization = url_parts[url_parts.len() - 3];
    let project = url_parts[url_parts.len() - 1];

    Ok((organization.to_string(), project.to_string()))
}

pub fn get_github_issue_nr_from_task_description(task_description: &str) -> Result<String, String> {
    let re = Regex::new(r"#(\d+)").map_err(|e| e.to_string())?;
    if let Some(captures) = re.captures(task_description) {
        Ok(captures[1].to_string())
    } else {
        Err("No issue number found in the task description".to_string())
    }
}
