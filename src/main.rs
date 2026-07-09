mod cli;
mod forgefile;
mod fs;
mod logging;
mod tasks;

use clap::Parser;
use cli::Args;
use forgefile::parser::parse_forgefile;
use fs::verify_forgefile_exists;
use logging::set_log_level;
use tasks::execution::execute_task;

use log::{debug, error, info};
use std::process::exit;

/// Entrypoint
fn main() {
    let args: Args = Args::parse();
    set_log_level(args.debug);

    match verify_forgefile_exists(&args.forgefile) {
        Ok(_) => debug!("Using Forgefile: {}", args.forgefile),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }

    let forgefile = match parse_forgefile(args.forgefile) {
        Ok(forgefile_struct) => forgefile_struct,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };
    debug!("Forgefile successfully parsed!");
    debug!("{}", forgefile.configuration);
    for (_, value) in forgefile.tasks {
        let _ = match execute_task(&forgefile.configuration, value) {
            Ok(msg) => {
                info!("{}", msg);
                msg
            }
            Err(e) => {
                error!("{}", e.to_string());
                e.to_string()
            }
        };
    }
}
