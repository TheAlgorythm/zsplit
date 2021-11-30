use crate::cli::NewFile;
use io::{BufRead, BufReader, BufWriter, Write};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn split(current_file: &PathBuf, new_files: &Vec<NewFile>) -> Result<(), io::Error> {
    let new_buffers = create_buffers(new_files)?;

    let mapped_line_buffers = map_line_buffers(new_files, &new_buffers);

    let source = BufReader::new(File::open(current_file)?);

    write_lines(source, &mapped_line_buffers)?;

    flush_buffers(&new_buffers)?;

    Ok(())
}

fn create_buffers(new_files: &Vec<NewFile>) -> Result<Vec<RefCell<BufWriter<File>>>, io::Error> {
    new_files
        .iter()
        .map(|new| {
            File::create(new.file.clone())
                .map(BufWriter::new)
                .map(RefCell::new)
        })
        .collect()
}

fn map_line_buffers<'a>(
    new_files: &Vec<NewFile>,
    new_buffers: &'a Vec<RefCell<BufWriter<File>>>,
) -> HashMap<usize, &'a RefCell<BufWriter<File>>> {
    new_files
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut mapped_line_buffers, (index, new)| {
            let line_ring_size = mapped_line_buffers.len();
            (line_ring_size..line_ring_size + new.assigned_lines).for_each(|line_index| {
                assert!(mapped_line_buffers
                    .insert(line_index, &new_buffers[index])
                    .is_none());
            });
            mapped_line_buffers
        })
}

fn write_lines(
    source: BufReader<File>,
    mapped_line_buffers: &HashMap<usize, &RefCell<BufWriter<File>>>,
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

fn flush_buffers(buffers: &Vec<RefCell<BufWriter<File>>>) -> Result<(), io::Error> {
    buffers
        .iter()
        .try_for_each(|buffer| buffer.borrow_mut().flush())
}
