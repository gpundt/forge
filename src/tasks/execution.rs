use crate::forgefile::{Config, Task};

use std::env;

use log::{debug, error, info, warn};
use std::process::Command;
use std::process::exit;

/// Function to execute an individual task's command
///
/// config - Forgefile's populated Config struct
/// forgefile_task - Individual task struct
pub fn execute_task(config: &Config, forgefile_task: Task) -> () {
    debug!("{:}", forgefile_task);
    let command_str = format!("{} -c {}", &config.shell, forgefile_task.command,);
    info!("{}", command_str);

    // Set local ignore_fail variable
    let ignore_fail = forgefile_task.ignore_fail.clone();

    // Set local working_dir variable
    let working_dir: String = match forgefile_task.working_dir.as_str() {
        "." => {
            let cwd = env::current_dir().unwrap();
            cwd.into_os_string()
                .into_string()
                .map_err(|_| "Directory path contains invalud UTF-8 data")
                .unwrap()
        }
        _other => forgefile_task.working_dir,
    };

    // Execute task command and collect output
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
                warn!("STDERR: {}", e.to_string());
                return ();
            } else {
                error!("STDERR: {}", e.to_string());
                exit(1);
            }
        }
    };

    // Process output
    if output.status.success() {
        info!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        if ignore_fail {
            warn!("STDERR: {}", String::from_utf8_lossy(&output.stderr))
        } else {
            error!("STDERR: {}", String::from_utf8_lossy(&output.stderr))
        }
    }
    ()
}
