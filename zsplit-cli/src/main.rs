#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::disallowed_types)]

mod cli;
mod source;

use clap::Parser;
use cli::Cli;
use human_panic::setup_panic;
use std::io::ErrorKind;
use zsplit::split_round_robin;

fn main() {
    setup_panic!();

    let cli = Cli::parse();

    if let Err(error) = cli.validate() {
        eprintln!("Error: {}", error);
        std::process::exit(exitcode::USAGE);
    }

    let mut source = match cli.source.reading_buffer() {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(exitcode::NOINPUT);
        }
    };

    let mut destinations = match cli.destinations() {
        Ok(destinations) => destinations,
        Err(error) => {
            eprintln!("Error: {}", error);
            let code = match error.kind() {
                ErrorKind::PermissionDenied => exitcode::NOPERM,
                _ => exitcode::IOERR,
            };
            std::process::exit(code);
        }
    };

    if let Err(error) = split_round_robin(&mut source, &mut destinations) {
        eprintln!("Error: {}", error);
        let code = match error.kind() {
            ErrorKind::PermissionDenied => exitcode::NOPERM,
            ErrorKind::AlreadyExists => exitcode::CANTCREAT,
            // NOTE Waits for stabilization of rust-lang/rust#86442
            // | ErrorKind::IsADirectory
            // | ErrorKind::ReadOnlyFilesystem
            // | ErrorKind::FilenameTooLong => exitcode::CANTCREAT,
            _ => exitcode::IOERR,
        };
        std::process::exit(code);
    }
}
