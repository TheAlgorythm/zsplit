use io::{BufWriter, Write};
use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[cfg(test)]
#[path = "./destination_test.rs"]
pub mod destination_test;

#[derive(Debug, Clone)]
pub struct Destination<W: Write> {
    pub assigned_lines: usize,
    pub sink: RefCell<W>,
}

impl<W: Write> Destination<W> {
    pub fn new(sink: W, assigned_lines: usize) -> Self {
        Self {
            sink: RefCell::new(sink),
            assigned_lines,
        }
    }
}

impl Destination<BufWriter<File>> {
    pub fn from_path(path: PathBuf, assigned_lines: usize) -> Result<Self, io::Error> {
        let sink = Self::create_sink(path)?;

        Ok(Self::new(sink, assigned_lines))
    }

    fn create_sink(path: PathBuf) -> Result<BufWriter<File>, io::Error> {
        File::create(path).map(BufWriter::new)
    }
}
