import { timeSpan, totalTimeSpentSpan, finishedTasksTable, projectSelectDropdown, projectSelectedSpan } from "./dom-elements.js";
import { deleteProject, exportProject, selectProject, deleteTask } from "./rust-bindings.js";
import { displayTotalTimeSpent, hisToMs } from "./utils.js";
const { message } = window.__TAURI__.dialog;

export function finishedTasksListener(e, state) {
    let tasks = JSON.parse(e.payload).reverse();

    finishedTasksTable.innerHTML = "<span class='table-header'>Day</span><span class='table-header'>Task description</span><span class='table-header'>Time spent</span><span class='table-header'></span>";
    state.totalMilliSecondsSpent = 0;
    tasks.forEach((row) => {

        let spanDate = document.createElement("span");
        spanDate.textContent = row[1];

        let spanDescription = document.createElement("span");
        spanDescription.textContent = row[2];
        
        let spanTime = document.createElement("span");
        spanTime.textContent = row[3];

        let spanDeleteButton = document.createElement("span");
        let deleteButtonImage = document.createElement("img");
        deleteButtonImage.src = "assets/trash-can.png";
        deleteButtonImage.classList.add("delete-button-image");
        deleteButtonImage
            .addEventListener("mouseover", (_e) => {
                _e.target.src = "assets/trash-can-red.png";
            });
        deleteButtonImage
            .addEventListener("mouseleave", (_e) => {
                _e.target.src = "assets/trash-can.png";
            });
        deleteButtonImage
            .addEventListener("click", () => {
                deleteTask(row[0]);
            });
        spanDeleteButton.appendChild(deleteButtonImage);

        state.totalMilliSecondsSpent += hisToMs(row[3]);

        finishedTasksTable.appendChild(spanDate);
        finishedTasksTable.appendChild(spanDescription);
        finishedTasksTable.appendChild(spanTime);
        finishedTasksTable.appendChild(spanDeleteButton);
    });
    
    displayTotalTimeSpent(state.totalMilliSecondsSpent);
}

export function projectListListener(e) {
    let projects = JSON.parse(e.payload);

    document.querySelectorAll(".project-select-dropdown-item:not(#project-add-new-container)").forEach(element => element.remove());
    projects.forEach((project) => {
        let projectSelectDropdownItem = document.createElement("div");
        projectSelectDropdownItem.classList.add("project-select-dropdown-item");

        let projectSelectDropdownItemSpan = document.createElement("span");
        projectSelectDropdownItemSpan.textContent = project;
        projectSelectDropdownItemSpan.addEventListener("click", () => {
            selectProject(project);
            projectSelectDropdown.classList.remove("open");
        });
        
        let projectSelectDropdownItemExportButton = document.createElement("div");
        projectSelectDropdownItemExportButton.classList.add("project-export-button");
        projectSelectDropdownItemExportButton.textContent = "EXPORT";
        projectSelectDropdownItemExportButton.addEventListener("click", () => {
            exportProject(project);
            projectSelectDropdown.classList.remove("open");
        });

        let projectSelectDropdownItemDeleteButton = document.createElement("div");
        projectSelectDropdownItemDeleteButton.classList.add("project-delete-button");
        projectSelectDropdownItemDeleteButton.textContent = "DEL";
        projectSelectDropdownItemDeleteButton.addEventListener("click", () => {
            if (projectSelectedSpan.textContent === project) {
                return;
            }

            deleteProject(project);
            projectSelectDropdown.classList.remove("open");
        });

        projectSelectDropdownItem.appendChild(projectSelectDropdownItemSpan);
        projectSelectDropdownItem.appendChild(projectSelectDropdownItemExportButton);
        projectSelectDropdownItem.appendChild(projectSelectDropdownItemDeleteButton);

        projectSelectDropdown.appendChild(projectSelectDropdownItem);
    });
}

export function selectedProjectListener(e) {
    projectSelectedSpan.textContent = e.payload;
}

