use io::{BufWriter, Write};
use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Destination<W: Write> {
    pub assigned_lines: usize,
    pub sink: RefCell<W>,
}

impl Destination<BufWriter<File>> {
    pub fn from_path(path: PathBuf, assigned_lines: usize) -> Result<Self, io::Error> {
        let buffer = Self::create_buffer(path)?;

        Ok(Destination {
            sink: buffer,
            assigned_lines,
        })
    }

    fn create_buffer(path: PathBuf) -> Result<RefCell<BufWriter<File>>, io::Error> {
        File::create(path).map(BufWriter::new).map(RefCell::new)
    }
}
