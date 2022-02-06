//! Split text into multiple sinks by line.
//!
//! This crate could be used to send input to multiple threads via [`channel_io`](https://lib.rs/crates/channel_io) or send it to the `STDIN` of multiple [`std::process::Command`]s.
//!
//! ## Crate Features
//!
//! - `cli-app`
//!   - Disabled by default.
//!   - Used for the identically named CLI application
//!
//! ## Examples
//!
//! ```rust
//! use zsplit::prelude::*;
//!
//! let data = "Hello\nWorld,\n42!";
//! let mut source = std::io::BufReader::new(data.as_bytes());
//!
//! let destinations = [
//!     Destination::new(std::io::sink(), 3),
//!     Destination::new(std::io::sink(), 2),
//!     Destination::new_sink(std::io::sink()),
//! ];
//!
//! split_round_robin(&mut source, &destinations).unwrap();
//! ```
//!
//! To split the data for the `STDIN` of multiple processes, you can use the following scheme.
//!
//! ```ignore
//! use zsplit::prelude::*;
//! use std::process::{Command, Stdio};
//!
//! let data = "Hello\nWorld,\n42!";
//! let mut source = std::io::BufReader::new(data.as_bytes());
//!
//! let mut child_1 = Command::new("cat").arg("-").stdin(Stdio::piped()).spawn().unwrap();
//! let mut child_2 = Command::new("cat").arg("-").stdin(Stdio::piped()).spawn().unwrap();
//! let destinations = [
//!     Destination::new_sink(child_1.stdin.take().unwrap()),
//!     Destination::new_sink(child_2.stdin.take().unwrap()),
//! ];
//!
//! split_round_robin(&mut source, &destinations).unwrap();
//!
//! child_1.kill().unwrap();
//! child_2.kill().unwrap();
//! ```

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

/// All you need from this Crate.
pub mod prelude {
    pub use crate::destination::Destination;
    pub use crate::split::split_round_robin;
}

#[doc(inline)]
pub use prelude::*;
