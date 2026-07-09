use std::str;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Pathfinder")]
#[command(version = "1.0")]
#[command(about = "Web scraper - Written in Rust", long_about = None)]
pub struct Args {
    /// Target URL to spider and scrape
    #[arg(short, long)]
    #[arg(default_value_t = String::from("./forge.toml"))]
    pub forgefile: String,

    /// Test the forge file's workflow, but don't execute any tasks
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Validate the forge file's syntax and formatting
    #[arg(short, long, default_value_t = false)]
    pub validate: bool,

    /// Turn on debug strings
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
