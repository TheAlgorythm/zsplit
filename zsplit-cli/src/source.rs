use error_stack::{Result, ResultExt};
use io::{BufRead, BufReader};
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[cfg(test)]
#[path = "./source_test.rs"]
mod source_test;

#[derive(Debug, PartialEq, Eq)]
pub enum Source {
    PathBuf(PathBuf),
    StdIn,
}

impl Source {
    pub fn reading_buffer(&self) -> Result<Box<dyn BufRead>, io::Error> {
        match self {
            Self::PathBuf(current_file) => Ok(Box::new(BufReader::new(
                File::open(current_file).attach_printable_lazy(|| {
                    format!(
                        "Couldn't open file `{}` as readable",
                        current_file.display()
                    )
                })?,
            ))),
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
