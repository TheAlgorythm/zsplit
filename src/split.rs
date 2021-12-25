use crate::cli::Destination;
use io::{BufRead, BufWriter, Write};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;

#[cfg(test)]
#[path = "./split_test.rs"]
mod split_test;

pub fn split(source: &mut dyn BufRead, destinations: &[Destination]) -> Result<(), io::Error> {
    let destination_buffers = create_buffers(destinations)?;

    let mapped_line_buffers = map_line_buffers(destinations, &destination_buffers);

    write_lines(source, &mapped_line_buffers)?;

    flush_buffers(&destination_buffers)?;

    Ok(())
}

fn create_buffers(
    destinations: &[Destination],
) -> Result<Vec<RefCell<BufWriter<File>>>, io::Error> {
    destinations
        .iter()
        .map(|new| {
            File::create(new.file.clone())
                .map(BufWriter::new)
                .map(RefCell::new)
        })
        .collect()
}

fn map_line_buffers<'a, B>(
    destinations: &[Destination],
    destination_buffers: &'a [B],
) -> HashMap<usize, &'a B> {
    destinations
        .iter()
        .enumerate()
        .scan(0, |line_ring_size, (index, new)| {
            let old_line_ring_size = *line_ring_size;
            *line_ring_size += new.assigned_lines;
            let buffer = &destination_buffers[index];

            Some(create_line_buffer_mapping(
                old_line_ring_size,
                *line_ring_size,
                buffer,
            ))
        })
        .flatten()
        .collect()
}

fn create_line_buffer_mapping<B>(
    old_line_ring_size: usize,
    line_ring_size: usize,
    buffer: &B,
) -> HashMap<usize, &B> {
    assert!(old_line_ring_size <= line_ring_size);

    (old_line_ring_size..line_ring_size)
        .map(|line_index| (line_index, buffer))
        .collect()
}

fn write_lines<W: Write>(
    source: &mut dyn BufRead,
    mapped_line_buffers: &HashMap<usize, &RefCell<W>>,
) -> Result<(), io::Error> {
    let line_ring_size = mapped_line_buffers.len();

    source
        .lines()
        .enumerate()
        .try_for_each(|(line_index, line)| {
            let line_index = line_index % line_ring_size;

            let mut buffer = mapped_line_buffers[&line_index].borrow_mut();

            writeln!(buffer, "{}", line?)
        })
}

fn flush_buffers<W: Write>(buffers: &[RefCell<BufWriter<W>>]) -> Result<(), io::Error> {
    buffers
        .iter()
        .try_for_each(|buffer| buffer.borrow_mut().flush())
}
