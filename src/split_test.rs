use super::*;
use std::path::PathBuf;

#[test]
fn simple_map_line_buffers() {
    let destinations = [
        Destination {
            file: PathBuf::new(),
            assigned_lines: 1,
        },
        Destination {
            file: PathBuf::new(),
            assigned_lines: 2,
        },
        Destination {
            file: PathBuf::new(),
            assigned_lines: 3,
        },
    ];

    let mock_buffers = [0, 1, 2];

    let mapped_line_buffers = map_line_buffers(&destinations, &mock_buffers);

    assert_eq!(
        mapped_line_buffers.len(),
        destinations.iter().map(|new| new.assigned_lines).sum()
    );

    assert_eq!(*mapped_line_buffers[&0], 0);
    (1..=2).for_each(|index| assert_eq!(*mapped_line_buffers[&index], 1));
    (3..=5).for_each(|index| assert_eq!(*mapped_line_buffers[&index], 2));
}

#[test]
#[should_panic]
fn unsymmetric_map_line_buffers() {
    let destinations = [
        Destination {
            file: PathBuf::new(),
            assigned_lines: 1,
        },
        Destination {
            file: PathBuf::new(),
            assigned_lines: 1,
        },
    ];

    let mock_buffers = [0];

    map_line_buffers(&destinations, &mock_buffers);
}

#[test]
fn empty_assigned_lines_map_line_buffers() {
    let destinations = [
        Destination {
            file: PathBuf::new(),
            assigned_lines: 0,
        },
        Destination {
            file: PathBuf::new(),
            assigned_lines: 1,
        },
    ];

    let mock_buffers = [0, 1];

    let mapped_line_buffers = map_line_buffers(&destinations, &mock_buffers);

    assert_eq!(
        mapped_line_buffers.len(),
        destinations.iter().map(|new| new.assigned_lines).sum()
    );

    assert_eq!(*mapped_line_buffers[&0], 1);
}
