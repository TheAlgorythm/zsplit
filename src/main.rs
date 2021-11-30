#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

use clap::Parser;
use zsplit::{split, Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(error) = split(&cli.splitting_file, &cli.new_files()) {
        eprintln!("Error: {}", error);
    }
}
