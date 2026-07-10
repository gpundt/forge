use crate::forgefile::{Config, Task};
use std::io::{self, Write};

use std::collections::HashMap;
use std::env;

use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::process::{Command, Stdio};

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

    // If 'confirm = true' is in Forgefile, prompt user before execution
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

    // Create new child process using config and Forgefile settings
    let mut child = match Command::new(&config.shell)
        .arg("-c")
        .arg(&command)
        .envs(&env)
        .current_dir(working_dir)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to spawn child process: {}", e.to_string());
            exit(1);
        }
    };

    // If child process has stdout, capture it and stream it as output
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(text) => println!("{}", text),
                Err(e) => {
                    error!("Failed to read stdout: {}", e.to_string());
                    exit(1);
                }
            }
        }
        println!("\n")
    }

    // Wait for child process to finish before return
    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to wait on child process: {}", e.to_string());
            exit(1);
        }
    };
    debug!("Process exited with status: {}", status);
    ()
}
