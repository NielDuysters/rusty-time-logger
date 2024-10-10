const { invoke } = window.__TAURI__.core;
const { message } = window.__TAURI__.dialog;

export async function save(ms, description) {
    await invoke("save", {ms, description });
}

export async function deleteTask(taskId) {
    await invoke("delete_task", { taskId });
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

export async function exitProgram() {
    await invoke("exit");
}
