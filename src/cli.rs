use crate::input::Input;
use bool_ext::BoolExt;
use clap::{Parser, ValueHint};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use thiserror::Error;

#[cfg(test)]
#[path = "./cli_test.rs"]
pub mod cli_test;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("The splitting_file is also in new_files.")]
    FileDuplicate,
}

#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(short = 'f', long, default_value = "1")]
    pub line_factor: NonZeroUsize,

    #[clap(parse(from_os_str = Input::from_os_str), value_hint(ValueHint::FilePath))]
    pub splitting_file: Input,

    #[clap(
        multiple_values(true),
        min_values(2),
        required(true),
        parse(from_os_str),
        value_hint(ValueHint::FilePath)
    )]
    pub new_files: Vec<PathBuf>,

    #[clap(short, long, multiple_values(true), min_values(0))]
    pub distribution: Vec<NonZeroUsize>,
}

impl Cli {
    pub fn validate(&self) -> Result<(), Error> {
        if let Input::PathBuf(splitting_file) = &self.splitting_file {
            self.new_files
                .iter()
                .all(|new_file| splitting_file != new_file)
                .err(Error::FileDuplicate)?;
        }
        Ok(())
    }

    pub fn new_files(&self) -> Vec<NewFile> {
        self.new_files
            .iter()
            .enumerate()
            .map(|(index, file)| NewFile {
                file: file.clone(),
                assigned_lines: usize::from(self.line_factor) * self.get_distribution(index),
            })
            .collect()
    }

    fn get_distribution(&self, index: usize) -> usize {
        self.distribution
            .get(index)
            .map(|distribution| usize::from(*distribution))
            .unwrap_or(1)
    }
}

#[derive(Debug)]
pub struct NewFile {
    pub assigned_lines: usize,
    pub file: PathBuf,
}
