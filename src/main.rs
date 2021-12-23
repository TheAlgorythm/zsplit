#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

use clap::Parser;
use zsplit::{reading_buffer, split, Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(error) = cli.validate() {
        eprintln!("Error: {}", error);
        std::process::exit(-1);
    }

    let mut source = match reading_buffer(&cli.splitting_file) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(-1);
        }
    };

    if let Err(error) = split(&mut source, &cli.new_files()) {
        eprintln!("Error: {}", error);
        std::process::exit(-1);
    }
}
