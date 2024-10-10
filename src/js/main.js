const { listen } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.core;

import { save, deleteTask, createNewProject, deleteProject, loadProjects, selectProject, exitProgram } from "./rust-bindings.js";
import { togglePlayPause, saveTask, showProjectDropdown, createNewProjectFromProjectDropdown, exitProgramAfterClick } from "./event-functions.js";
import { playButton, saveButton, projectSelectedSpan, projectAddNewButton, exitButton } from "./dom-elements.js";
import { finishedTasksListener, projectListListener, selectedProjectListener } from "./rust-listeners.js";


window.addEventListener("DOMContentLoaded", () => {
    let state = {
        isPlaying: false,
        totalMilliSecondsSpent: 0,
        taskMilliSecondsSpent: 0,
        startTime: null,
        firstClick: true,
    };

    playButton.addEventListener("click", (e) => {
        togglePlayPause(e, state);
    });

    saveButton.addEventListener("click", () => {
        saveTask(state);
    });
    
    projectSelectedSpan.addEventListener("click", () => {
        showProjectDropdown();
    });

    projectAddNewButton.addEventListener("click", () => {
        createNewProjectFromProjectDropdown();
    });

    exitButton.addEventListener("click", () => {
        exitProgramAfterClick(state);
    });

    listen("finished_tasks", (event) => {
        finishedTasksListener(event, state);
    });

    setTimeout(function() {
        invoke("update_finished_tasks");
        loadProjects();
    }, 50);

    listen("project_list", (event) => {
        projectListListener(event);
    });

    listen("selected_project", (event) => {
        selectedProjectListener(event);
    });
});
