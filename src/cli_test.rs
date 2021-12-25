use super::*;
use std::convert::TryInto;

fn empty_cli() -> Cli {
    Cli {
        line_factor: 1_usize.try_into().unwrap(),
        source: Source::PathBuf(PathBuf::new()),
        destinations: vec![PathBuf::new(), PathBuf::new(), PathBuf::new()],
        distribution: Vec::new(),
    }
}

#[test]
fn invalid_splitting_file_in_new_files() {
    let cli = empty_cli();

    assert_eq!(cli.validate(), Err(Error::FileDuplicate));
}

#[test]
fn valid_splitting_file_not_in_new_files() {
    let mut cli = empty_cli();
    cli.source = Source::PathBuf(PathBuf::from("test.txt"));

    assert_eq!(cli.validate(), Ok(()));
}

#[test]
fn valid_splitting_file_stdin() {
    let mut cli = empty_cli();
    cli.source = Source::StdIn;

    assert_eq!(cli.validate(), Ok(()));
}

#[test]
fn default_distribution() {
    let new_files = empty_cli().new_files();

    assert_eq!(new_files.len(), 3);
    assert!(new_files
        .iter()
        .all(|new_file| new_file.assigned_lines == 1));
}

#[test]
fn default_distribution_with_line_factor() {
    let mut cli = empty_cli();
    cli.line_factor = 2_usize.try_into().unwrap();

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert!(new_files
        .iter()
        .all(|new_file| new_file.assigned_lines == 2));
}

#[test]
fn partial_distribution() {
    let mut cli = empty_cli();
    cli.distribution = vec![3_usize.try_into().unwrap(), 3_usize.try_into().unwrap()];

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert_eq!(new_files[0].assigned_lines, 3);
    assert_eq!(new_files[1].assigned_lines, 3);
    assert_eq!(new_files[2].assigned_lines, 1);
}

#[test]
fn partial_distribution_with_line_factor() {
    let mut cli = empty_cli();
    cli.distribution = vec![3_usize.try_into().unwrap(), 3_usize.try_into().unwrap()];
    cli.line_factor = 2_usize.try_into().unwrap();

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert_eq!(new_files[0].assigned_lines, 6);
    assert_eq!(new_files[1].assigned_lines, 6);
    assert_eq!(new_files[2].assigned_lines, 2);
}
