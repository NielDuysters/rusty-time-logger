import { timeSpan, totalTimeSpentSpan, finishedTasksTable, projectSelectDropdown, projectSelectedSpan } from "./dom-elements.js";

function msToHis(ms) {
    const totalSeconds = Math.floor(ms / 1000);

    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    const h = String(hours).padStart(2, '0');
    const m = String(minutes).padStart(2, '0');
    const s = String(seconds).padStart(2, '0');

    return `${h}:${m}:${s}`;
}

export function displayTaskTimeSpent(ms) {
    timeSpan.textContent = msToHis(ms);
}

export function displayTotalTimeSpent(ms) {
    totalTimeSpentSpan.textContent = msToHis(ms);
}
