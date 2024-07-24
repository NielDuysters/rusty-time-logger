const { message } = window.__TAURI__.dialog;
const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;


async function play() {
    await invoke("play");
}

async function save(taskDescription) {
    await invoke("save", { taskDescription });
}

async function deleteTask(taskId) {
    await invoke("delete_task", { taskId });
}

async function createNewProject(projectId) {
     try {
        await invoke("create_new_project", { projectId });
//        await message("New project created succesfully.");
       // await loadProjects();
    } catch (error) {
  //      await message(error);
    }
}

async function deleteProject(projectId) {
    try {
        await invoke("delete_project", { projectId });
        //await message("Project deleted.");
    } catch (error) {
        await message(error);
    }
}

async function loadProjects() {
    await invoke("load_projects");
}

async function selectProject(projectId) {
    await invoke("select_project", { projectId });
}

async function exitProgram() {
    await invoke("exit");
}

function hisToSeconds(timeString) {
    console.log(timeString);
    const [hours, minutes, seconds] = timeString.split(':').map(Number);
    return hours * 3600 + minutes * 60 + seconds;
}

window.addEventListener("DOMContentLoaded", async () => {
    let isPlaying = false;
    let totalSecondsSpent = 0;
    
    const playButton = document.getElementById("play-button");
    const saveButton = document.getElementById("save-button");
    const timeSpan = document.getElementById("time");
    const finishedTasksTable = document.getElementById("finished-tasks-table");
    const taskDescriptionInput = document.getElementById("task-description-input"); 
    const totalTimeSpentSpan = document.getElementById("total-time-spent");
    const projectSelectedSpan = document.getElementById("project-selected");
    const projectAddNewButton = document.getElementById("project-add-new-button");
    const projectSelectDropdown = document.getElementById("project-select-dropdown");
    const projectAddNewInput = document.getElementById("project-add-new-input");
    const exitButton = document.getElementById("exit");

    playButton.addEventListener("click", (e) => {
        e.preventDefault();

        isPlaying = !isPlaying;
        e.target.src = (isPlaying ? "assets/pause-button.png" : "assets/play-button.png");
        if (isPlaying) {
            saveButton.classList.remove("visible");
        } else {
            saveButton.classList.add("visible");
        }

        play();
    });

    saveButton.addEventListener("click", () => {
        save(taskDescriptionInput.value);
        saveButton.classList.remove("visible");
        playButton.src = "assets/play-button.png";
        time.textContent = "00:00:00";
    });
    
    projectSelectedSpan.addEventListener("click", () => {
        if (projectSelectDropdown.classList.contains("open")) {
            projectSelectDropdown.classList.remove("open");
        } else {
            projectSelectDropdown.classList.add("open");
        }
    });

    projectAddNewButton.addEventListener("click", async () => {
        await createNewProject(projectAddNewInput.value);
    });

    exitButton.addEventListener("click", async () => {
        await save(taskDescriptionInput.value);
        await exitProgram();
    });

    listen("update_time", (event) => {
        const seconds = event.payload;
        const time = new Date(seconds * 1000).toISOString().slice(11, 19);
        timeSpan.textContent = `${time}`;

        const totalTime = new Date((seconds + totalSecondsSpent) * 1000).toISOString().slice(11, 19);
        totalTimeSpentSpan.textContent = `${totalTime}`;
    });

    listen("finished_tasks", (event) => {
        let tasks = JSON.parse(event.payload).reverse();

        finishedTasksTable.innerHTML = "<span class='table-header'>Day</span><span class='table-header'>Task description</span><span class='table-header'>Time spent</span><span class='table-header'></span>";
        totalSecondsSpent = 0;
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
                .addEventListener("mouseover", (e) => {
                    e.target.src = "assets/trash-can-red.png";
                });
            deleteButtonImage
                .addEventListener("mouseleave", (e) => {
                    e.target.src = "assets/trash-can.png";
                });
            deleteButtonImage
                .addEventListener("click", () => {
                    deleteTask(row[0]);
                });
            spanDeleteButton.appendChild(deleteButtonImage);

            totalSecondsSpent += hisToSeconds(row[3]);

            finishedTasksTable.appendChild(spanDate);
            finishedTasksTable.appendChild(spanDescription);
            finishedTasksTable.appendChild(spanTime);
            finishedTasksTable.appendChild(spanDeleteButton);
        });

        const time = new Date(totalSecondsSpent * 1000).toISOString().slice(11, 19);
        totalTimeSpentSpan.textContent = `${time}`;
    });

    setTimeout(function() {
        invoke("update_finished_tasks");
        loadProjects();
    },50);

    listen("project_list", (event) => {
        let projects = JSON.parse(event.payload);
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

            let projectSelectDropdownItemDeleteButton = document.createElement("div");
            projectSelectDropdownItemDeleteButton.classList.add("project-delete-button");
            projectSelectDropdownItemDeleteButton.textContent = "DEL";
            projectSelectDropdownItemDeleteButton.addEventListener("click", () => {
                deleteProject(project);
            });

            projectSelectDropdownItem.appendChild(projectSelectDropdownItemSpan);
            projectSelectDropdownItem.appendChild(projectSelectDropdownItemDeleteButton);

            projectSelectDropdown.appendChild(projectSelectDropdownItem);
        });
    });

    listen("selected_project", (event) => {
        projectSelectedSpan.textContent = event.payload;
    });
});
