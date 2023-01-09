use super::*;

#[test]
fn hyphen_as_stdout() {
    assert_eq!(Source::from_os_str("-".as_ref()), Source::StdIn);
}

#[test]
fn path_as_path_buf() {
    let path = "test.txt";
    assert_eq!(
        Source::from_os_str(path.as_ref()),
        Source::PathBuf(PathBuf::from(path))
    );
}
