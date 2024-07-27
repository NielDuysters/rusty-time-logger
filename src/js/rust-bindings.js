const { invoke } = window.__TAURI__.tauri;

export async function play() {
    await invoke("play");
}

export async function save(taskDescription) {
    await invoke("save", { taskDescription });
}

export async function deleteTask(taskId) {
    await invoke("delete_task", { taskId });
}

export async function createNewProject(projectId) {
     try {
        await invoke("create_new_project", { projectId });
      // await message("New project created succesfully.");
       // await loadProjects();
    } catch (error) {
  //      await message(error);
    }
}

export async function deleteProject(projectId) {
    try {
        await invoke("delete_project", { projectId });
        //await message("Project deleted.");
    } catch (error) {
        await message(error);
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
