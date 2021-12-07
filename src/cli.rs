use clap::Parser;
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[cfg(test)]
#[path = "./cli_test.rs"]
pub mod cli_test;

#[derive(Parser, Debug)]
#[clap(about, author)]
pub struct Cli {
    #[clap(short = 'f', long, default_value = "1")]
    pub line_factor: NonZeroUsize,
    pub splitting_file: PathBuf,
    #[clap(short, long)]
    pub new_files: Vec<PathBuf>,
    #[clap(short, long)]
    pub distribution: Vec<NonZeroUsize>,
}

impl Cli {
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
