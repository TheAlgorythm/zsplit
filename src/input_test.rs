use super::*;

#[test]
fn hyphen_as_stdout() {
    assert_eq!(Input::from_os_str("-".as_ref()), Input::StdIn);
}

#[test]
fn path_as_path_buf() {
    let path = "test.txt";
    assert_eq!(
        Input::from_os_str(path.as_ref()),
        Input::PathBuf(PathBuf::from(path))
    );
}
