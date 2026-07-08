mod core;

use clap::Parser;
use core::{arguments::Args, filepath, logging};
use log::{debug, error, info};
use std::process::exit;

fn main() {
    let args: Args = Args::parse();
    logging::_set_log_level(args.debug);

    debug!("{:<20}{}", "Forge File:", args.forge_file);
    debug!("{:<20}{}", "Verbose:", args.debug);

    match filepath::verify_forgefile_exists(&args.forge_file) {
        Ok(_) => debug!("Using Forgefile: {}", args.forge_file),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }
    ()
}
