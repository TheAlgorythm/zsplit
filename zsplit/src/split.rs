//! Algorithms for splitting a source into destination sinks.

use crate::Destination;
use io::{BufRead, Write};
use std::io;

#[cfg(test)]
#[path = "./split_test.rs"]
mod split_test;

/// Splits the `source` round robin like into `destinations`.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use zsplit::prelude::*;
///
/// let data = "Hello\nWorld,\n42!";
/// let mut source = std::io::BufReader::new(data.as_bytes());
/// let mut destinations = [
///     Destination::new_with_sink(std::io::sink()),
///     Destination::new_with_sink(std::io::sink()),
/// ];
///
/// split_round_robin(&mut source, &mut destinations).unwrap();
/// ```
///
/// Split Text:
///
/// ```rust
/// use zsplit::prelude::*;
///
/// let data = "Hello\nWorld,\n42!";
/// let mut source = std::io::BufReader::new(data.as_bytes());
/// let mut destinations = vec![
///     Destination::buffer(), // first_destination
///     Destination::buffer(), // second_destination
/// ];
///
/// split_round_robin(&mut source, &mut destinations).unwrap();
///
/// let second_destination = destinations.pop().unwrap();
/// let first_destination = destinations.pop().unwrap();
///
/// assert_eq!(first_destination.into_utf8_string().unwrap(), "Hello\n42!\n");
/// assert_eq!(second_destination.into_utf8_string().unwrap(), "World,\n");
/// ```
pub fn split_round_robin<S: Write>(
    source: &mut dyn BufRead,
    destinations: &mut [Destination<S>],
) -> io::Result<()> {
    let mapped_line_destinations = round_robin::map_line_destinations(destinations);

    round_robin::write_lines(source, destinations, &mapped_line_destinations)?;

    flush_buffers(destinations)?;

    Ok(())
}

/// Round Robin specific algorithms.
mod round_robin {
    use crate::Destination;
    use io::{BufRead, Write};
    use std::io;

    /// Maps a [`Destination`] with the line number.
    ///
    /// The output represents:
    ///
    /// ```plain
    /// mapped_line_destinations[line % mapped_line_destinations.len()] -> index(destination)
    /// ```
    pub fn map_line_destinations<S: Write>(destinations: &[Destination<S>]) -> Vec<usize> {
        destinations
            .iter()
            .enumerate()
            .flat_map(|(index, destination)| {
                std::iter::repeat(index).take(destination.assigned_lines)
            })
            .collect()
    }

    pub fn write_lines<S: Write>(
        source: &mut dyn BufRead,
        destinations: &mut [Destination<S>],
        mapped_line_destinations: &[usize],
    ) -> io::Result<()> {
        let line_ring_size = mapped_line_destinations.len();

        source
            .lines()
            .enumerate()
            .try_for_each(|(line_index, line)| {
                let line_index = line_index % line_ring_size;

                let sink = &mut destinations[mapped_line_destinations[line_index]];

                sink.write_all(line?.as_bytes())?;
                sink.write_all(b"\n")?;
                Ok(())
            })
    }
}

fn flush_buffers<S: Write>(destinations: &mut [Destination<S>]) -> io::Result<()> {
    destinations.iter_mut().try_for_each(Destination::flush)
}
