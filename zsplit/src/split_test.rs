use super::*;
mod round_robin {
    use super::*;
    use crate::split::round_robin::*;

    #[test]
    fn simple_map_line_destinations() {
        let destinations = [
            Destination::new(io::sink(), 1),
            Destination::new(io::sink(), 2),
            Destination::new(io::sink(), 3),
        ];

        let mapped_line_destinations = map_line_destinations(&destinations);

        assert_eq!(
            mapped_line_destinations.len(),
            destinations.iter().map(|new| new.assigned_lines).sum()
        );

        assert_eq!(mapped_line_destinations[0], 0);
        (1..=2).for_each(|index| assert_eq!(mapped_line_destinations[index], 1));
        (3..=5).for_each(|index| assert_eq!(mapped_line_destinations[index], 2));
    }

    #[test]
    fn empty_assigned_lines_map_line_destinations() {
        let destinations = [
            Destination::new(io::sink(), 0),
            Destination::new(io::sink(), 1),
        ];

        let mapped_line_destinations = map_line_destinations(&destinations);

        assert_eq!(
            mapped_line_destinations.len(),
            destinations.iter().map(|new| new.assigned_lines).sum()
        );

        assert_eq!(mapped_line_destinations[0], 1);
    }
}
