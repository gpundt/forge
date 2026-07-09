use crate::forgefile::{Config, Task};

use std::env;

use clap::builder::Str;
use log::{debug, error, info};
use std::process::Command;

pub fn execute_task(config: &Config, forgefile_task: Task) -> Result<String, String> {
    debug!("{:}", forgefile_task);
    let command_str = format!("{} -c {}", &config.shell, forgefile_task.command,);
    info!("{}", command_str);

    let ignore_fail = forgefile_task.ignore_fail.clone();

    let working_dir: String = match forgefile_task.working_dir.as_str() {
        "." => {
            let cwd = env::current_dir().unwrap();
            cwd.into_os_string()
                .into_string()
                .map_err(|_| "Directory path contains invalud UTF-8 data")?
        }
        _other => forgefile_task.working_dir,
    };

    let output = match Command::new(&config.shell)
        .arg("-c")
        .arg(forgefile_task.command)
        .envs(&forgefile_task.env)
        .current_dir(working_dir)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            if ignore_fail {
                return Ok(format!("STDERR: {}", e.to_string()));
            } else {
                return Err(format!("STDERR: {}", e.to_string()));
            }
        }
    };

    if output.status.success() {
        return Ok(format!("{}", String::from_utf8_lossy(&output.stdout),));
    } else {
        if ignore_fail {
            return Ok(format!(
                "STDERR: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        } else {
            return Err(format!(
                "STDERR: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
}
