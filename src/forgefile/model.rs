use core::fmt;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;

// ──── ForgeFile ──────────────────────────────────────────────────────────
/// Struct to hold parsed Forgefile contents
pub struct ForgeFile {
    pub configuration: Config,
    pub tasks: IndexMap<String, Task>,
    pub filepath: String,
}

/// Function to initialize a new ForgeFile struct
impl ForgeFile {
    pub fn new(configuration: Config, tasks: IndexMap<String, Task>, filepath: String) -> Self {
        Self {
            configuration,
            tasks,
            filepath,
        }
    }
}

/// Function to print out the contents of a ForgeFile struct
impl fmt::Display for ForgeFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\nTasks: {:?}",
            self.filepath, self.configuration, self.tasks,
        )
    }
}

// ──── Config ─────────────────────────────────────────────────────────────
/// Struct to hold parsed Forgefile config contents
#[derive(Deserialize)]
pub struct Config {
    pub default_task: String,
    pub shell: String,
    #[serde(default)]
    pub env_file: String,
    #[serde(default)]
    pub stop_on_failure: bool,
}

/// Function to print out the contents of a Forgefile Config struct
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config:\n\tDefault Task: {}\n\tShell: {}\n\tEnv File: {}\n\tStop on Failure: {}\n",
            self.default_task, self.shell, self.env_file, self.stop_on_failure,
        )
    }
}

// ──── ForgeFileRaw ───────────────────────────────────────────────────────
// Mirrors the top-level shape of the TOML file: [config] and [task.*]
#[derive(Deserialize)]
pub struct ForgeFileRaw {
    pub config: Config,
}
impl fmt::Display for ForgeFileRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config: {}", self.config,)
    }
}

// ──── Task ───────────────────────────────────────────────────────────────
/// Struct to hold parsed Forgefile task contents
#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    #[serde(skip)]
    pub name: String,
    pub description: String,
    pub command: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    pub working_dir: String,
    pub confirm: bool,
    pub timeout: u32,
    #[serde(default)]
    pub ignore_fail: bool,
}

/// Function to print out the contents of a Forgefile Task
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "task.{}\n\tDescription: {}\n\tCommand: {}\n\tDepends On: {:?}\n\tEnv: {:?}\n\tWorking Dir: {}\n\tConfirm: {}\n\tTimeout: {}\n\tIgnore Fail: {}\n",
            self.name,
            self.description,
            self.command,
            self.depends_on,
            self.env,
            self.working_dir,
            self.confirm,
            self.timeout,
            self.ignore_fail,
        )
    }
}
