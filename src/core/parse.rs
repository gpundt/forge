use clap::Error;
use indexmap::IndexSet;
use log::{debug, error, info};
use std::collections::{HashMap, HashSet};
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

// pub fn parse_forgefile(filepath: String) -> Result<ForgeFile, Error> {
//     let path = Path::new(&filepath);
// }

// fn parse_forgefile_config(content: String) -> Result<Config, Error> {

// }

// fn parse_forgefile_tasks(content: String) -> Result<HashSet<Task>, Error> {

// }
