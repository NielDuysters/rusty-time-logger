const { message } = window.__TAURI__.dialog;
const { listen } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.tauri;

import { play, save, deleteTask, createNewProject, deleteProject, loadProjects, selectProject, exitProgram } from "./rust-bindings.js";
import { togglePlayPause, saveTask, showProjectDropdown, createNewProjectFromProjectDropdown, exitProgramAfterClick } from "./event-functions.js";
import { playButton, saveButton, projectSelectedSpan, projectAddNewButton, exitButton } from "./dom-elements.js";
import { updateTimeListener, finishedTasksListener, projectListListener, selectedProjectListener } from "./rust-listeners.js";


window.addEventListener("DOMContentLoaded", () => {
    let state = {
        isPlaying: false,
        totalSecondsSpent: 0,
    };

    playButton.addEventListener("click", (e) => {
        togglePlayPause(e, state);
    });

    saveButton.addEventListener("click", () => {
        saveTask();
    });
    
    projectSelectedSpan.addEventListener("click", () => {
        showProjectDropdown();
    });

    projectAddNewButton.addEventListener("click", () => {
        createNewProjectFromProjectDropdown();
    });

    exitButton.addEventListener("click", () => {
        exitProgramAfterClick();
    });

    listen("update_time", (event) => {
        updateTimeListener(event, state);
    });

    listen("finished_tasks", (event) => {
        finishedTasksListener(event, state);
    });

    setTimeout(function() {
        invoke("update_finished_tasks");
        loadProjects();
    },50);

    listen("project_list", (event) => {
        projectListListener(event);
    });

    listen("selected_project", (event) => {
        selectedProjectListener(event);
    });
});
