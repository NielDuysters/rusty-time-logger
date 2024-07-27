import { play, save, exitProgram, createNewProject } from "./rust-bindings.js";
import { saveButton, playButton, taskDescriptionInput, projectSelectDropdown, projectAddNewInput } from "./dom-elements.js";

export function togglePlayPause(e, state) {
    e.preventDefault();

    state.isPlaying = !state.isPlaying;
    e.target.src = (state.isPlaying ? "assets/pause-button.png" : "assets/play-button.png");
    console.log(e.target.src);
    if (state.isPlaying) {
        saveButton.classList.remove("visible");
    } else {
        saveButton.classList.add("visible");
    }

    play();
}

export function saveTask() {
    save(taskDescriptionInput.value);
    saveButton.classList.remove("visible");
    playButton.src = "assets/play-button.png";
    time.textContent = "00:00:00"; //timeSpan?
}

export function showProjectDropdown() {
    if (projectSelectDropdown.classList.contains("open")) {
        projectSelectDropdown.classList.remove("open");
    } else {
        projectSelectDropdown.classList.add("open");
    }
}

export function createNewProjectFromProjectDropdown() {
    createNewProject(projectAddNewInput.value);
}

export async function exitProgramAfterClick() {
    await save(taskDescriptionInput.value);
    await exitProgram();
}
