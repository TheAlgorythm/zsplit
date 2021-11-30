use clap::Parser;
use zsplit::{split, Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(error) = split(&cli.splitting_file, &cli.new_files()) {
        eprintln!("Error: {}", error);
    }
}
