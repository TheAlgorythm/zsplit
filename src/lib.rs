#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

use std::path::PathBuf;

#[cfg(feature = "cli-app")]
pub mod cli;
#[cfg(feature = "cli-app")]
pub mod source;
pub mod split;

#[cfg(feature = "cli-app")]
pub use cli::Cli;
pub use split::split;

#[derive(Debug, Clone)]
pub struct Destination {
    pub assigned_lines: usize,
    pub file: PathBuf,
}
