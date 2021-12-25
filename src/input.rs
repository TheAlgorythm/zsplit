use io::{BufRead, BufReader};
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[cfg(test)]
#[path = "./input_test.rs"]
mod input_test;

#[derive(Debug, PartialEq)]
pub enum Input {
    PathBuf(PathBuf),
    StdIn,
}

impl Input {
    pub fn reading_buffer(&self) -> Result<Box<dyn BufRead>, io::Error> {
        match self {
            Self::PathBuf(current_file) => Ok(Box::new(BufReader::new(File::open(current_file)?))),
            Self::StdIn => {
                let stdin = Box::leak(Box::new(io::stdin()));
                Ok(Box::new(stdin.lock()))
            }
        }
    }

    pub fn from_os_str(path: &std::ffi::OsStr) -> Self {
        if path == "-" {
            return Self::StdIn;
        }
        Self::PathBuf((*path).into())
    }
}
