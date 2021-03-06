use crate::source::Source;
use crate::Destination;
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
    #[error("The source is also in destinations.")]
    FileDuplicate,
    #[error("The quantity of destinations ({destinations_len}) is smaller as of distributions ({distributions_len}).")]
    MoreDistributionsAsDestinations {
        destinations_len: usize,
        distributions_len: usize,
    },
}

#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub struct Cli {
    /// A factor to multiply the grouping size of the distribution.
    #[clap(short = 'f', long, default_value = "1")]
    pub line_factor: NonZeroUsize,

    /// The file which should be splitted. Use '-' for piping the content to zsplit.
    #[clap(parse(from_os_str = Source::from_os_str), value_hint(ValueHint::FilePath))]
    pub source: Source,

    /// A list of destinations for the splitted contents.
    #[clap(
        multiple_values(true),
        min_values(2),
        required(true),
        parse(from_os_str),
        value_hint(ValueHint::FilePath)
    )]
    pub destinations: Vec<PathBuf>,

    /// Defines how many lines are assigned to a destination. The distributions have to be in the
    /// same order as the destinations. It defaults to 1.
    #[clap(short, long, multiple_values(true), min_values(0))]
    pub distributions: Vec<NonZeroUsize>,
}

impl Cli {
    pub fn validate(&self) -> Result<(), Error> {
        if let Source::PathBuf(source) = &self.source {
            self.destinations
                .iter()
                .all(|destination| source != destination)
                .err(Error::FileDuplicate)?;
        }

        let (destinations_len, distributions_len) =
            (self.destinations.len(), self.distributions.len());
        (destinations_len >= distributions_len).err(Error::MoreDistributionsAsDestinations {
            destinations_len,
            distributions_len,
        })?;

        Ok(())
    }

    pub fn destinations(&self) -> Vec<Destination> {
        self.destinations
            .iter()
            .enumerate()
            .map(|(index, file)| Destination {
                file: file.clone(),
                assigned_lines: usize::from(self.line_factor) * self.get_distribution(index),
            })
            .collect()
    }

    fn get_distribution(&self, index: usize) -> usize {
        self.distributions
            .get(index)
            .map(|distribution| usize::from(*distribution))
            .unwrap_or(1)
    }
}
