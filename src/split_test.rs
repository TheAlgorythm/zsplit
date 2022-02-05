use super::*;
use crate::destination::destination_test::IdSink;

#[test]
fn simple_map_line_destinations() {
    let destinations = [
        Destination::new(IdSink::new(0), 1),
        Destination::new(IdSink::new(1), 2),
        Destination::new(IdSink::new(2), 3),
    ];

    let mapped_line_destinations = map_line_destinations(&destinations);

    assert_eq!(
        mapped_line_destinations.len(),
        destinations.iter().map(|new| new.assigned_lines).sum()
    );

    assert_eq!(mapped_line_destinations[0].borrow().id(), 0);
    (1..=2).for_each(|index| assert_eq!(mapped_line_destinations[index].borrow().id(), 1));
    (3..=5).for_each(|index| assert_eq!(mapped_line_destinations[index].borrow().id(), 2));
}

#[test]
fn empty_assigned_lines_map_line_destinations() {
    let destinations = [
        Destination::new(IdSink::new(0), 0),
        Destination::new(IdSink::new(1), 1),
    ];

    let mapped_line_destinations = map_line_destinations(&destinations);

    assert_eq!(
        mapped_line_destinations.len(),
        destinations.iter().map(|new| new.assigned_lines).sum()
    );

    assert_eq!(mapped_line_destinations[0].borrow().id(), 1);
}
