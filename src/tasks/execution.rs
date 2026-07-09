use crate::forgefile::{Config, Task};
use std::io::{self, Write};

use std::collections::HashMap;
use std::env;

use log::{debug, error, info, warn};
use std::process::Command;
use std::process::exit;

/// Function to execute an individual task's command
///
/// config - Forgefile's populated Config struct
/// forgefile_task - Individual task struct
pub fn execute_task(config: &Config, forgefile_task: Task) -> () {
    // Set owned task variables
    let command: String = forgefile_task.command.clone();
    let env: HashMap<String, String> = forgefile_task.env.clone();
    let confirm: bool = forgefile_task.confirm.clone();
    let ignore_fail: bool = forgefile_task.ignore_fail.clone();

    let command_str = format!("{} -c {}", &config.shell, forgefile_task.command,);
    debug!("{:}", forgefile_task);
    info!("{}", command_str);

    // Set local working_dir variable
    let working_dir: String = match forgefile_task.working_dir.as_str() {
        "." => {
            let current_dir_path = env::current_dir().unwrap();
            current_dir_path.to_string_lossy().into_owned()
        }
        _other => forgefile_task.working_dir,
    };

    if confirm {
        info!("Proceed with executing: '{}'?", command);
        print!("(y/n) >>>   ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "N" || input.trim() == "n" {
            if !ignore_fail {
                error!("Closing...");
                exit(1);
            } else {
                warn!("Skipping...\n");
                return ();
            }
        }
    }

    // Execute task command and collect output
    let output = match Command::new(&config.shell)
        .arg("-c")
        .arg(&command)
        .envs(&env)
        .current_dir(working_dir)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            if ignore_fail {
                warn!("STDERR: {}\n", e.to_string());
                return ();
            } else {
                error!("STDERR: {}", e.to_string());
                exit(1);
            }
        }
    };

    // Process output
    if output.status.success() {
        info!("STDOUT: {}\n", String::from_utf8_lossy(&output.stdout));
    } else {
        if ignore_fail {
            warn!("STDERR: {}\n", String::from_utf8_lossy(&output.stderr))
        } else {
            error!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    }
    ()
}
