//! Algorithms for splitting a source into destination sinks.

use crate::Destination;
use io::{BufRead, Write};
use std::cell::RefCell;
use std::io;

#[cfg(test)]
#[path = "./split_test.rs"]
mod split_test;

/// Splits the `source` round robin like into `destinations`.
///
/// ```rust
/// use zsplit::prelude::*;
///
/// let data = "Hello\nWorld,\n42!";
/// let mut source = std::io::BufReader::new(data.as_bytes());
/// let destinations = [
///     Destination::new_with_sink(std::io::sink()),
///     Destination::new_with_sink(std::io::sink()),
/// ];
///
/// split_round_robin(&mut source, &destinations).unwrap();
/// ```
pub fn split_round_robin<W: Write>(
    source: &mut dyn BufRead,
    destinations: &[Destination<W>],
) -> Result<(), io::Error> {
    let mapped_line_destinations = map_line_destinations(destinations);

    write_lines(source, &mapped_line_destinations)?;

    flush_buffers(destinations)?;

    Ok(())
}

fn map_line_destinations<W: Write>(destinations: &[Destination<W>]) -> Vec<&RefCell<W>> {
    destinations
        .iter()
        .map(|destination| std::iter::repeat(&destination.sink).take(destination.assigned_lines))
        .flatten()
        .collect()
}

fn write_lines<W: Write>(
    source: &mut dyn BufRead,
    mapped_line_destinations: &[&RefCell<W>],
) -> Result<(), io::Error> {
    let line_ring_size = mapped_line_destinations.len();

    source
        .lines()
        .enumerate()
        .try_for_each(|(line_index, line)| {
            let line_index = line_index % line_ring_size;

            let mut sink = mapped_line_destinations[line_index].borrow_mut();

            writeln!(sink, "{}", line?)
        })
}

fn flush_buffers<W: Write>(destinations: &[Destination<W>]) -> Result<(), io::Error> {
    destinations
        .iter()
        .try_for_each(|destination| destination.sink.borrow_mut().flush())
}
