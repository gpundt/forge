mod cli;
mod forgefile;
mod fs;
mod logging;

use clap::Parser;
use cli::Args;
use forgefile::parser::parse_forgefile;
use fs::verify_forgefile_exists;
use logging::set_log_level;

use log::{debug, error, info};
use std::process::exit;

fn main() {
    let args: Args = Args::parse();
    set_log_level(args.debug);

    debug!("{:<20}{}", "Forgefile:", args.forgefile);
    debug!("{:<20}{}", "Debug:", args.debug);

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
    debug!("Forgefile successfully parsed:\n{}", forgefile);

    ()
}
