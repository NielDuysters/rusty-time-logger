import { save, exitProgram, createNewProject } from "./rust-bindings.js";
import { saveButton, playButton, taskDescriptionInput, projectSelectDropdown, projectAddNewInput } from "./dom-elements.js";
import { displayTaskTimeSpent, displayTotalTimeSpent } from "./utils.js";

export function togglePlayPause(e, state) {
    e.preventDefault();

    if (state.firstClick) {
        state.firstClick = false;
        setInterval(() => {
            if (!state.isPlaying) {
                return;
            }
           
            const taskTimeSpent = state.taskMilliSecondsSpent + (Date.now() - state.startTime);
            displayTaskTimeSpent(taskTimeSpent);
            displayTotalTimeSpent(state.totalMilliSecondsSpent + taskTimeSpent);

        }, 1000);
    }

    state.isPlaying = !state.isPlaying;
    e.target.src = (state.isPlaying ? "assets/pause-button.png" : "assets/play-button.png");
    if (state.isPlaying) {
        state.startTime = Date.now();
        saveButton.classList.remove("visible");
    } else {
        state.taskMilliSecondsSpent += Date.now() - state.startTime;
        saveButton.classList.add("visible");
    }
}

export function saveTask(state) {
    save(state.taskMilliSecondsSpent, taskDescriptionInput.value);
    state.taskMilliSecondsSpent = 0;
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

export async function exitProgramAfterClick(state) {
    await save(state.taskMilliSecondsSpent, taskDescriptionInput.value);
    await exitProgram();
}
