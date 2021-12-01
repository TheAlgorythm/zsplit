use super::*;

fn empty_cli() -> Cli {
    Cli {
        line_factor: 1,
        splitting_file: PathBuf::new(),
        new_files: vec![PathBuf::new(), PathBuf::new(), PathBuf::new()],
        distribution: Vec::new(),
    }
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
    cli.line_factor = 2;

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert!(new_files
        .iter()
        .all(|new_file| new_file.assigned_lines == 2));
}

#[test]
fn partial_distribution() {
    let mut cli = empty_cli();
    cli.distribution = vec![3, 3];

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert_eq!(new_files[0].assigned_lines, 3);
    assert_eq!(new_files[1].assigned_lines, 3);
    assert_eq!(new_files[2].assigned_lines, 1);
}

#[test]
fn partial_distribution_with_line_factor() {
    let mut cli = empty_cli();
    cli.distribution = vec![3, 3];
    cli.line_factor = 2;

    let new_files = cli.new_files();

    assert_eq!(new_files.len(), 3);
    assert_eq!(new_files[0].assigned_lines, 6);
    assert_eq!(new_files[1].assigned_lines, 6);
    assert_eq!(new_files[2].assigned_lines, 2);
}
