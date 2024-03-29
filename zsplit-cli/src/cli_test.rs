use super::*;
use crate::Error;
use std::convert::TryInto;

fn empty_cli() -> Cli {
    Cli {
        line_factor: non_zero_usize(1),
        source: Source::PathBuf(PathBuf::new()),
        destinations: vec![PathBuf::new(), PathBuf::new(), PathBuf::new()],
        distributions: Vec::new(),
    }
}

#[inline]
fn non_zero_usize(num: usize) -> NonZeroUsize {
    num.try_into().unwrap()
}

#[test]
fn invalid_source_in_destinations() {
    let cli = empty_cli();

    assert_eq!(
        *cli.validate().unwrap_err().current_context(),
        Error::FileDuplicate
    );
}

#[test]
fn valid_source_not_in_destinations() {
    let mut cli = empty_cli();
    cli.source = Source::PathBuf(PathBuf::from("test.txt"));

    cli.validate().unwrap();
}

#[test]
fn valid_source_stdin() {
    let mut cli = empty_cli();
    cli.source = Source::StdIn;

    cli.validate().unwrap();
}

#[test]
fn valid_balanced_distributions_destinations() {
    let mut cli = empty_cli();
    cli.source = Source::StdIn;
    cli.distributions = vec![non_zero_usize(3), non_zero_usize(3), non_zero_usize(3)];

    cli.validate().unwrap();
}

#[test]
fn invalid_more_distributions_than_destinations() {
    let mut cli = empty_cli();
    cli.destinations = Vec::new();
    cli.distributions = vec![non_zero_usize(3), non_zero_usize(3)];

    assert_eq!(
        *cli.validate().unwrap_err().current_context(),
        Error::MoreDistributionsAsDestinations {
            destinations_len: 0,
            distributions_len: 2
        }
    );
}

#[test]
fn default_distribution() {
    let destinations = empty_cli().destinations().unwrap();

    assert_eq!(destinations.len(), 3);
    assert!(destinations
        .iter()
        .all(|destination| destination.assigned_lines == 1));
}

#[test]
fn default_distribution_with_line_factor() {
    let mut cli = empty_cli();
    cli.line_factor = non_zero_usize(2);

    let destinations = cli.destinations().unwrap();

    assert_eq!(destinations.len(), 3);
    assert!(destinations
        .iter()
        .all(|destination| destination.assigned_lines == 2));
}

#[test]
fn partial_distribution() {
    let mut cli = empty_cli();
    cli.distributions = vec![non_zero_usize(3), non_zero_usize(3)];

    let destinations = cli.destinations().unwrap();

    assert_eq!(destinations.len(), 3);
    assert_eq!(destinations[0].assigned_lines, 3);
    assert_eq!(destinations[1].assigned_lines, 3);
    assert_eq!(destinations[2].assigned_lines, 1);
}

#[test]
fn partial_distribution_with_line_factor() {
    let mut cli = empty_cli();
    cli.distributions = vec![non_zero_usize(3), non_zero_usize(3)];
    cli.line_factor = non_zero_usize(2);

    let destinations = cli.destinations().unwrap();

    assert_eq!(destinations.len(), 3);
    assert_eq!(destinations[0].assigned_lines, 6);
    assert_eq!(destinations[1].assigned_lines, 6);
    assert_eq!(destinations[2].assigned_lines, 2);
}

#[cfg(any(unix, target_os = "redox"))]
#[test]
fn error_during_sink_creation() {
    use std::os::unix::ffi::OsStrExt;
    let mut cli = empty_cli();
    cli.destinations[0] = std::ffi::OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]).into();

    let _ = cli.destinations().unwrap_err();
}
