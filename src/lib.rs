#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

pub mod cli;
pub mod split;

pub use cli::Cli;
pub use split::split;

use io::{BufRead, BufReader};
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn reading_buffer(current_file: &Option<PathBuf>) -> Result<Box<dyn BufRead>, io::Error> {
    if let Some(current_file) = current_file {
        Ok(Box::new(BufReader::new(File::open(current_file)?)))
    } else {
        let stdin = Box::leak(Box::new(io::stdin()));
        Ok(Box::new(stdin.lock()))
    }
}
