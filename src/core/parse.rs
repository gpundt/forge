use indexmap::IndexMap;
use std::collections::HashMap;

struct ForgeFile {
    configuration: Config,
    tasks: IndexMap,
    filepath: String,
}

struct Config {
    default_task: String,
    shell: String,
    env_file: String,
    stop_on_failure: bool,
}

struct Task {
    description: String,
    command: String,
    depends_on: Vec<String>,
    env: HashMap,
    working_dir: String,
    confirm: bool,
    timeout: u32,
    ignore_fail: bool,
}
