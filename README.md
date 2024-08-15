![logo](https://github.com/user-attachments/assets/059b21de-d829-445a-867b-b9bc65e7fb82)
# Rusty Time Logger
The fastest time logger application in the world! Built in Rust.

I do some freelance web-development and needed a basic and easy-to-use project time logger to track the time spent on a specific project to bill clients. As a hobby-project I made my own debloated version as alternative to Toggl, Clockify or Zapier.

**Features:**
- Time track multiple projects.
- Play/pause button to start of pause timer.
- Save a finished task with a custom description.
- Export project log to a HTML table.
- View total time spent, time spent per task and a full log of the workflow.

## Screenshots
Interface
![Interface](screenshots/general-interface.png)

Select, add, delete or export projects
![Project Actions](screenshots/project-actions.png)

Export project log
![Project log](screenshots/project-export.jpg)

## Technical specifications
- Rust (1.79.0)
- Tauri (1.7.1)
- Developed on UNIX

## Installation
Prerequisites:
- Rust
- Tauri
- Tauri-cli

```
# Clone repository
git clone https://github.com/NielDuysters/rusty-time-logger.git
cd rusty-time-logger

# Build
cargo tauri build

# Move bundled file to Applications folder
# E.g for MacOS:
cp -r src-tauri/target/release/bundle/macos/rusty-time-logger.app /Applications
```


## TODO
- Fix Tuari dialog issue [(View issue)](https://github.com/tauri-apps/tauri/issues/10586#issue-2462152966)
- Use SQLite instead of CSV
- Code improvements and best practices

This was a hobby-project. So feedback is always appreciated!
