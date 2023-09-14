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
use error_stack::ResultExt;
use human_panic::setup_panic;
use std::io;
use sysexits::ExitCode;
use zsplit::split_round_robin;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("The source is also in destinations")]
    FileDuplicate,
    #[error("The quantity of destinations ({destinations_len}) is smaller as of distributions ({distributions_len})")]
    MoreDistributionsAsDestinations {
        destinations_len: usize,
        distributions_len: usize,
    },
    #[error("Couldn't read from source")]
    Source,
    #[error("Couldn't write to destination")]
    Destination,
    #[error("Problem occurred during splitting")]
    Split,
}

type Result<T> = error_stack::Result<T, Error>;

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    cli.validate().attach(ExitCode::Usage)?;

    let mut source = cli.source.reading_buffer().change_context(Error::Source)?;

    let mut destinations = cli.destinations()?;

    split_round_robin(&mut source, &mut destinations).change_context(Error::Split)
}

fn main() -> std::process::ExitCode {
    setup_panic!();

    if let Err(report) = try_main() {
        eprintln!("Error: {:?}", report);

        if let Some(exit_code) = report.downcast_ref::<ExitCode>() {
            return (*exit_code).into();
        }

        if let Some(io_error) = report.downcast_ref::<io::Error>() {
            return ExitCode::from(io_error.kind()).into();
        }

        return 1.into();
    }

    std::process::ExitCode::SUCCESS
}
