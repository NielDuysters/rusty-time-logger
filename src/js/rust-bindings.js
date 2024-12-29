const { invoke } = window.__TAURI__.core;
const { message } = window.__TAURI__.dialog;

export async function save(projectId, ms, description) {
    await invoke("save", {projectId, ms, description });
}

export async function deleteTask(projectId, taskId) {
    await invoke("delete_task", { projectId, taskId });
}

export async function createNewProject(projectId) {
     try {
        await invoke("create_new_project", { projectId });
        await message(`Project ${projectId} created.`, { title: 'Rusty Time Logger', kind: 'info' });
    } catch (error) {
        await message(`Error creating project ${projectId}: ${error.message}`, { title: 'Rusty Time Logger', kind: 'error' });
        return;
    }

    await selectProject(projectId);
}

export async function deleteProject(projectId) {
    try {
        await invoke("delete_project", { projectId });
        await message(`Project ${projectId} deleted.`, { title: 'Rusty Time Logger', kind: 'info' });
    } catch (error) {
        await message(`Error deleting project ${projectId}: ${error.message}`, { title: 'Rusty Time Logger', kind: 'error' });
    }
}

export async function exportProject(projectId) {
    try {
        await invoke("export_project", { projectId });
        await message(`Project ${projectId} exported.`, { title: 'Rusty Time Logger', kind: 'info' });
    } catch (error) {
        await message(`Error exporting project ${projectId}: ${error.message}`, { title: 'Rusty Time Logger', kind: 'error' });
    }
}

export async function loadProjects() {
    await invoke("load_projects");
}

export async function selectProject(projectId) {
    await invoke("select_project", { projectId });
}

export async function saveGithubSettings(
    projectId,
    authToken,
    projectUrl,
    spentTimeFieldName,
    timelogTicketNr,
) {
    try {
        await invoke("save_github_settings", { projectId, authToken, projectUrl, spentTimeFieldName, timelogTicketNr });
        await message(`GitHub settings saved.`, { title: 'Rusty Time Logger', kind: 'info' });
    } catch (error) {
        await message(`Error saving GitHub settings: ${error.message}`, { title: 'Rusty Time Logger', kind: 'error' });
    }
}

export async function syncGithub(projectId) {
    try {
        await invoke("sync_spent_time_to_pm", { projectId });
        await message(`Spent time synced to GitHub.`, { title: 'Rusty Time Logger', kind: 'info' });
    } catch (error) {
        await message(`Error syncing spent time to GitHub: ${error.message}`, { title: 'Rusty Time Logger', kind: 'error' });
    }
}

export async function exitProgram() {
    await invoke("exit");
}
