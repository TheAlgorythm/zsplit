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
use std::convert::TryInto;
use std::io;
use zsplit::split_round_robin;

fn write_map_err<T, E: std::fmt::Display>(
    res: Result<T, E>,
    code: exitcode::ExitCode,
) -> Result<T, std::process::ExitCode> {
    let code: u8 = code.try_into().unwrap();
    res.map_err(|e| {
        eprintln!("Error: {}", e);
        code.into()
    })
}

fn write_map_io_err<T>(
    res: error_stack::Result<T, io::Error>,
) -> Result<T, std::process::ExitCode> {
    res.map_err(|e| {
        eprintln!("Error: {:?}", e);

        let e = e.current_context();

        use io::ErrorKind;
        let code: u8 = match e.kind() {
            ErrorKind::PermissionDenied => exitcode::NOPERM,
            ErrorKind::AlreadyExists => exitcode::CANTCREAT,
            // NOTE Waits for stabilization of rust-lang/rust#86442
            // | ErrorKind::IsADirectory
            // | ErrorKind::ReadOnlyFilesystem
            // | ErrorKind::FilenameTooLong => exitcode::CANTCREAT,
            _ => exitcode::IOERR,
        }
        .try_into()
        .unwrap();
        code.into()
    })
}

fn try_main() -> Result<(), std::process::ExitCode> {
    let cli = Cli::parse();

    write_map_err(cli.validate(), exitcode::USAGE)?;

    let mut source = write_map_io_err(cli.source.reading_buffer())?;

    let mut destinations = write_map_io_err(cli.destinations())?;

    write_map_io_err(
        split_round_robin(&mut source, &mut destinations)
            .attach_printable("Problem occurred during splitting"),
    )
}

fn main() -> std::process::ExitCode {
    setup_panic!();

    if let Err(code) = try_main() {
        return code;
    }

    std::process::ExitCode::SUCCESS
}
