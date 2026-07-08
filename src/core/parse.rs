use indexmap::{IndexMap, IndexSet};
use clap::Error;
use log::{debug, error, info};
use serde::Deserialize;
use toml::Table;
use std::collections::{HashMap, HashSet};
use std::fs;

pub struct ForgeFile {
    configuration: Config,
    tasks: IndexMap<String, Task>,
    filepath: String,
}

impl ForgeFile {
    fn new(configuration: Config, tasks: IndexMap<String, Task>, filepath: String) -> Self {
        Self {
            configuration,
            tasks,
            filepath,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    default_task: String,
    shell: String,
    #[serde(default)]
    env_file: String,
    #[serde(default)]
    stop_on_failure: bool,
}

// Mirrors the top-level shape of the TOML file: [config] and [task.*]
#[derive(Debug, Deserialize)]
struct ForgeFileRaw {
    config: Config,
}

impl Config {
    fn new(default_task: String, shell: String, env_file: String, stop_on_failure: bool) -> Self {
        Self {
            default_task,
            shell,
            env_file,
            stop_on_failure,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Task {
    #[serde(skip)]
    name: String,
    description: String,
    command: String,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
    working_dir: String,
    confirm: bool,
    timeout: u32,
    #[serde(default)]
    ignore_fail: bool,
}

impl Task {
    fn new(
        name: String,
        description: String,
        command: String,
        depends_on: Vec<String>,
        env: HashMap<String, String>,
        working_dir: String,
        confirm: bool,
        timeout: u32,
        ignore_fail: bool,
    ) -> Self {
        Self {
            name,
            description,
            command,
            depends_on,
            env,
            working_dir,
            confirm,
            timeout,
            ignore_fail,
        }
    }
}

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

fn parse_forgefile_tasks(content: String) -> Result<IndexMap<String, Task>, String> {
    let table = match content.parse::<Table>() {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    let task_table = match table.get("task").and_then(|v| v.as_table()) {
        Some(t) => t,
        None => return Ok(IndexMap::new()),
    };

    let mut tasks = IndexMap::new();
    for (name, value) in task_table {
        let mut task: Task = value.clone().try_into().map_err(|e| {
            format!("Failed to parse task '{}': {}", name, e)
        })?;

        task.name = name.clone();
        tasks.insert(name.clone(), task);
    }

    Ok(tasks)
}
