#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

pub mod cli;
pub mod source;
pub mod split;

pub use cli::Cli;
pub use split::split;
