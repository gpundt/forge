use clap::Error;
use indexmap::IndexSet;
use log::{debug, error, info};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs;
use std::ops::Index;
use std::path::Path;

struct ForgeFile {
    configuration: Config,
    tasks: IndexSet<Task>,
    filepath: String,
}

impl ForgeFile {
    fn new(configuration: Config, tasks: IndexSet<Task>, filepath: String) -> Self {
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
    env_file: String,
    stop_on_failure: bool,
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

#[derive(Debug, Deserialize)]
struct Task {
    description: String,
    command: String,
    depends_on: Vec<String>,
    env: HashMap<String, String>,
    working_dir: String,
    confirm: bool,
    timeout: u32,
    ignore_fail: bool,
}

impl Task {
    fn new(
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
    let contents: String = match fs::read_to_string(filepath) {
        Ok(text) => text,
        Err(e) => {
            return Err(format!(
                "Failed to read file contents of {}: {}",
                filepath,
                e.to_string(),
            ));
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            return Err(format!(
                "Failed to parse Forgefile Config: {}",
                e.to_string()
            ));
        }
    };

    let tasks: IndexSet<Task> = match parse_forgefile_tasks(contents) {
        Ok(hashset) => hashset,
        Err(e) => {
            return Err(format!(
                "Failed to parse Forgefile Tasks: {}",
                e.to_string(),
            ));
        }
    };

    Ok(ForgeFile::new(config, tasks, filepath))
}

// fn parse_forgefile_tasks(content: String) -> Result<HashSet<Task>, Error> {

// }
