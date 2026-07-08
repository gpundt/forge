mod core;

use clap::Parser;
use core::{arguments::Args, logging};
use log::{debug, error, info};
use std::process::exit;

fn main() {
    let args: Args = Args::parse();
    logging::_set_log_level(args.debug);

    debug!("{:<20}{}", "Forge File:", args.forge_file);
    debug!("{:<20}{}", "Verbose:", args.debug);
    ()
}
