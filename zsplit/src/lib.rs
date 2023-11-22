//! Split text into multiple sinks by line.
//!
//! This crate could be used to send input to multiple threads via [`channel_io`](https://lib.rs/crates/channel_io)
//! or send it to the `STDIN` of multiple [`std::process::Command`]s.
//!
//! # Warnings
//!
//! As a line of [`std::io::BufReader`] has to be completely in memory, an unbounded long line
//! could lead to a `DoS` vulnerability.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! use zsplit::prelude::*;
//!
//! let data = "Hello\nWorld,\n42!";
//! let mut source = std::io::BufReader::new(data.as_bytes());
//!
//! let mut destinations = [
//!     Destination::new(std::io::sink(), 3),
//!     Destination::new(std::io::sink(), 2),
//!     Destination::new_with_sink(std::io::sink()),
//! ];
//!
//! split_round_robin(&mut source, &mut destinations).unwrap();
//! ```
//!
//! Split a string with an unsymmetric distribution:
//!
//! ```rust
//! use zsplit::prelude::*;
//!
//! let data = "0\n1\n2\n3\n4\n5\n6\n7\n8\n9";
//! let mut source = std::io::BufReader::new(data.as_bytes());
//! let mut destinations = vec![
//!     Destination::buffer_with_lines(3), // first_destination
//!     Destination::buffer_with_lines(3), // second_destination
//!     Destination::buffer(), // third_destination
//! ];
//!
//! split_round_robin(&mut source, &mut destinations).unwrap();
//!
//! let third_destination = destinations.pop().unwrap();
//! let second_destination = destinations.pop().unwrap();
//! let first_destination = destinations.pop().unwrap();
//!
//! assert_eq!(first_destination.into_utf8_string().unwrap(), "0\n1\n2\n7\n8\n9\n");
//! assert_eq!(second_destination.into_utf8_string().unwrap(), "3\n4\n5\n");
//! assert_eq!(third_destination.into_utf8_string().unwrap(), "6\n");
//! ```
//!
//! To split the data for the `STDIN` of multiple processes, you can use the following scheme.
//!
//! ```no_run
//! use zsplit::prelude::*;
//! use std::process::{Command, Stdio};
//!
//! let data = "Hello\nWorld,\n42!";
//! let mut source = std::io::BufReader::new(data.as_bytes());
//!
//! let mut child_1 = Command::new("cat").arg("-").stdin(Stdio::piped()).spawn().unwrap();
//! let mut child_2 = Command::new("cat").arg("-").stdin(Stdio::piped()).spawn().unwrap();
//!
//! let mut destinations = [
//!     Destination::new_with_sink(child_1.stdin.take().unwrap()),
//!     Destination::new_with_sink(child_2.stdin.take().unwrap()),
//! ];
//!
//! split_round_robin(&mut source, &mut destinations).unwrap();
//!
//! child_1.kill().unwrap();
//! child_2.kill().unwrap();
//! ```

pub mod destination;
pub mod split;

/// All you need from this Crate.
pub mod prelude {
    pub use crate::destination::Destination;
    pub use crate::split::round_robin as split_round_robin;
}

#[doc(inline)]
pub use prelude::*;
