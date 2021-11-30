use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about, author)]
pub struct Cli {
    #[clap(short = 'f', long, default_value = "1")]
    pub line_factor: usize,
    pub splitting_file: PathBuf,
    #[clap(short, long)]
    pub new_files: Vec<PathBuf>,
    #[clap(short, long)]
    pub distribution: Vec<usize>,
}

impl Cli {
    pub fn new_files(&self) -> Vec<NewFile> {
        self.new_files
            .iter()
            .enumerate()
            .map(|(index, file)| NewFile {
                file: file.clone(),
                assigned_lines: self.line_factor * self.distribution.get(index).unwrap_or(&1),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct NewFile {
    pub assigned_lines: usize,
    pub file: PathBuf,
}
