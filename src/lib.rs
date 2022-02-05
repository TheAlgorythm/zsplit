#![forbid(unsafe_code)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]

#[cfg(feature = "cli-app")]
pub mod cli;
pub mod destination;
#[cfg(feature = "cli-app")]
pub mod source;
pub mod split;

#[cfg(feature = "cli-app")]
pub use cli::Cli;

pub mod prelude {
    pub use crate::destination::Destination;
    pub use crate::split::split_round_robin;
}

pub use prelude::*;
