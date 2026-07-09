use super::model::{ForgeFile, ForgeFileRaw, Task};
use indexmap::IndexMap;
use std::fs;
use toml::Table;

/// Function to parse the contents of a Forgefile and populate a ForgeFile struct
///
/// filepath - Filepath to the Forgefile we need to parse
///
/// Returns populated ForgeFile struct
pub fn parse_forgefile(filepath: String) -> Result<ForgeFile, String> {
    let contents: String = match fs::read_to_string(&filepath) {
        Ok(text) => text,
        Err(e) => {
            return Err(format!(
                "Failed to read file contents of {}: {}",
                filepath,
                e.to_string(),
            ));
        }
    };

    let raw: ForgeFileRaw = match toml::from_str(&contents) {
        Ok(raw) => raw,
        Err(e) => {
            return Err(format!(
                "Failed to parse Forgefile Config: {}",
                e.to_string()
            ));
        }
    };

    let tasks: IndexMap<String, Task> = match parse_forgefile_tasks(contents) {
        Ok(t) => t,
        Err(e) => {
            return Err(format!(
                "Failed to parse Forgefile Tasks: {}",
                e.to_string(),
            ));
        }
    };

    Ok(ForgeFile::new(raw.config, tasks, filepath))
}

/// Function to parse the contents of a Forgefile and populate Task structs
///
/// content - Content of the parsed Forgefile
///
/// Returns IndexMap of populated Forgefile Tasks
fn parse_forgefile_tasks(content: String) -> Result<IndexMap<String, Task>, String> {
    // Entire content string is parsed into a toml::Table
    let table = match content.parse::<Table>() {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    // Isolate 'task' headers
    let task_table = match table.get("task").and_then(|v| v.as_table()) {
        Some(t) => t,
        None => return Ok(IndexMap::new()),
    };

    // Iterate over each named task entry
    let mut tasks = IndexMap::new();
    for (name, value) in task_table {
        let mut task: Task = value
            .clone()
            .try_into()
            .map_err(|e| format!("Failed to parse task '{}': {}", name, e))?;

        task.name = name.clone();
        tasks.insert(name.clone(), task);
    }

    Ok(tasks)
}
