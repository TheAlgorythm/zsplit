use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use itertools::Itertools;
use std::fs::read_to_string;
use std::path::Path;

fn seq(from: isize, to: isize, step_width: usize) -> String {
    (from..=to).step_by(step_width).join("\n") + "\n"
}

fn seq_ring(from: isize, to: isize, modulo: isize, reminders: &[isize]) -> String {
    (from..=to)
        .filter(|i| reminders.iter().any(|reminder| i % modulo == *reminder))
        .join("\n")
        + "\n"
}

fn outputs(prefix: &str, count: usize) -> Vec<NamedTempFile> {
    (0..count)
        .map(|i| NamedTempFile::new(prefix.to_string() + "_output_" + &i.to_string()).unwrap())
        .collect()
}

fn paths(files: &[NamedTempFile]) -> Vec<&Path> {
    files.iter().map(NamedTempFile::path).collect()
}

fn close(files: Vec<NamedTempFile>) {
    files
        .into_iter()
        .try_for_each(NamedTempFile::close)
        .unwrap()
}

#[test]
fn simple() {
    let input = NamedTempFile::new("simple_input").unwrap();

    let to = 1000;

    input.write_str(&seq(0, to, 1)).unwrap();

    let outputs = outputs("simple", 4);

    Command::cargo_bin("zsplit")
        .unwrap()
        .arg(input.path())
        .args(&paths(&outputs))
        .assert()
        .success();

    for i in 0..outputs.len() {
        assert_eq!(
            read_to_string(&outputs[i]).unwrap(),
            seq(i as isize, to, outputs.len())
        );
    }

    input.close().unwrap();
    close(outputs);
}

#[test]
fn simple_pipe() {
    let to = 100000;

    let outputs = outputs("simple_pipe", 4);

    Command::cargo_bin("zsplit")
        .unwrap()
        .write_stdin(seq(0, to, 1))
        .arg("-")
        .args(&paths(&outputs))
        .assert()
        .success();

    for i in 0..outputs.len() {
        assert_eq!(
            read_to_string(&outputs[i]).unwrap(),
            seq(i as isize, to, outputs.len())
        );
    }

    close(outputs);
}

#[test]
fn unsymmetric_pipe() {
    let to = 100000;

    let outputs = outputs("unsymmetric_pipe", 4);

    Command::cargo_bin("zsplit")
        .unwrap()
        .write_stdin(seq(0, to, 1))
        .arg("-")
        .args(&paths(&outputs))
        .args(&["-d", "3", "3", "2"])
        .assert()
        .success();

    assert_eq!(
        read_to_string(&outputs[0]).unwrap(),
        seq_ring(0, to, 9, &[0, 1, 2])
    );
    assert_eq!(
        read_to_string(&outputs[1]).unwrap(),
        seq_ring(0, to, 9, &[3, 4, 5])
    );
    assert_eq!(
        read_to_string(&outputs[2]).unwrap(),
        seq_ring(0, to, 9, &[6, 7])
    );
    assert_eq!(
        read_to_string(&outputs[3]).unwrap(),
        seq_ring(0, to, 9, &[8])
    );

    close(outputs);
}

#[test]
fn multiple_pipe() {
    let to = 100000;

    let outputs = outputs("multiple_pipe", 4);

    Command::cargo_bin("zsplit")
        .unwrap()
        .write_stdin(seq(0, to, 1))
        .arg("-")
        .args(&paths(&outputs))
        .arg("--line-factor=2")
        .assert()
        .success();

    assert_eq!(
        read_to_string(&outputs[0]).unwrap(),
        seq_ring(0, to, 8, &[0, 1])
    );
    assert_eq!(
        read_to_string(&outputs[1]).unwrap(),
        seq_ring(0, to, 8, &[2, 3])
    );
    assert_eq!(
        read_to_string(&outputs[2]).unwrap(),
        seq_ring(0, to, 8, &[4, 5])
    );
    assert_eq!(
        read_to_string(&outputs[3]).unwrap(),
        seq_ring(0, to, 8, &[6, 7])
    );

    close(outputs);
}

#[test]
fn multiple_unsymmetric_pipe() {
    let to = 100000;

    let outputs = outputs("multiple_unsymmetric_pipe", 3);

    Command::cargo_bin("zsplit")
        .unwrap()
        .write_stdin(seq(0, to, 1))
        .arg("-")
        .args(&paths(&outputs))
        .arg("--line-factor=2")
        .args(&["-d", "1", "3", "2"])
        .assert()
        .success();

    assert_eq!(
        read_to_string(&outputs[0]).unwrap(),
        seq_ring(0, to, 12, &[0, 1])
    );
    assert_eq!(
        read_to_string(&outputs[1]).unwrap(),
        seq_ring(0, to, 12, &[2, 3, 4, 5, 6, 7])
    );
    assert_eq!(
        read_to_string(&outputs[2]).unwrap(),
        seq_ring(0, to, 12, &[8, 9, 10, 11])
    );

    close(outputs);
}

#[test]
fn many_outputs() {
    let to = 100000;

    let outputs = outputs("many_outputs", 42);

    Command::cargo_bin("zsplit")
        .unwrap()
        .write_stdin(seq(0, to, 1))
        .arg("-")
        .args(&paths(&outputs))
        .assert()
        .success();

    for i in 0..outputs.len() {
        assert_eq!(
            read_to_string(&outputs[i]).unwrap(),
            seq(i as isize, to, outputs.len())
        );
    }

    close(outputs);
}

#[test]
fn usage_error() {
    let input = NamedTempFile::new("usage_error_input").unwrap();
    let outputs = outputs("usage_error", 2);

    Command::cargo_bin("zsplit")
        .unwrap()
        .arg(input.path())
        .arg(input.path())
        .args(&paths(&outputs))
        .assert()
        .failure()
        .code(exitcode::USAGE);

    for i in 0..outputs.len() {
        read_to_string(&outputs[i]).unwrap_err();
    }

    input.close().unwrap();
    close(outputs);
}
