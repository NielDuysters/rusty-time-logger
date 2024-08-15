const { invoke } = window.__TAURI__.tauri;
const { message } = window.__TAURI__.dialog;

export async function save(ms, description) {
    await invoke("save", {ms, description });
}

export async function deleteTask(taskId) {
    await invoke("delete_task", { taskId });
}

export async function createNewProject(projectId) {
     try {
      // message("New project created succesfully.");
        await invoke("create_new_project", { projectId });
       // await loadProjects();
    } catch (error) {
        //await message(error);
        return;
    }

    await selectProject(projectId);
}

export async function deleteProject(projectId) {
    try {
        await invoke("delete_project", { projectId });
//        await message("Project deleted.");
    } catch (error) {
       // await message(error);
    }
}

export async function exportProject(projectId) {
    await invoke("export_project", { projectId });
 //   await message("Project exported.");
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
