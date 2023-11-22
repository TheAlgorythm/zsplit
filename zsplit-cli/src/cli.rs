use crate::source::Source;
use bool_ext::BoolExt;
use clap::{Parser, ValueHint};
use error_stack::ResultExt;
use std::io;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use zsplit::Destination;

#[cfg(test)]
#[path = "./cli_test.rs"]
pub(crate) mod cli_test;

#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Cli {
    /// A factor to multiply the grouping size of the distribution.
    #[clap(short = 'f', long, default_value = "1")]
    pub(crate) line_factor: NonZeroUsize,

    /// The file which should be splitted. Use '-' for piping the content to zsplit.
    #[clap(parse(from_os_str = Source::from_os_str), value_hint(ValueHint::FilePath))]
    pub(crate) source: Source,

    /// A list of destinations for the splitted contents.
    #[clap(
        multiple_values(true),
        min_values(2),
        required(true),
        parse(from_os_str),
        value_hint(ValueHint::FilePath)
    )]
    pub(crate) destinations: Vec<PathBuf>,

    /// Defines how many lines are assigned to a destination. The distributions have to be in the
    /// same order as the destinations. It defaults to 1.
    #[clap(short, long, multiple_values(true), min_values(0))]
    pub(crate) distributions: Vec<NonZeroUsize>,
}

impl Cli {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        if let Source::PathBuf(source) = &self.source {
            self.destinations
                .iter()
                .all(|destination| source != destination)
                .err(crate::Error::FileDuplicate)?;
        }

        let (destinations_len, distributions_len) =
            (self.destinations.len(), self.distributions.len());
        (destinations_len >= distributions_len).err(
            crate::Error::MoreDistributionsAsDestinations {
                destinations_len,
                distributions_len,
            },
        )?;

        Ok(())
    }

    pub(crate) fn destinations(
        &self,
    ) -> crate::Result<Vec<Destination<impl io::Write + std::fmt::Debug>>> {
        self.destinations
            .iter()
            .enumerate()
            .map(|(index, file)| {
                Destination::new_with_path_and_lines(
                    file.clone(),
                    usize::from(self.line_factor) * self.get_distribution(index),
                )
                .change_context(crate::Error::Destination)
                .attach_printable_lazy(|| {
                    format!("Couldn't open file `{}` as writable", file.display())
                })
            })
            .collect()
    }

    fn get_distribution(&self, index: usize) -> usize {
        self.distributions
            .get(index)
            .map_or(1, |distribution| usize::from(*distribution))
    }
}
