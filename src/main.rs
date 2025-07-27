// src/main.rs
/*
 * Main executable for SmartWeb
 */

use clap::Parser;
use smartweb::{Result, run};

#[derive(Parser)]
#[command(version, about = "SmartWeb - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
