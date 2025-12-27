use crate::cell::{FieldCell, ShipPart};
use crate::coord::Coord;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ship {
    capacity: u8,
    parts: Vec<FieldCell>,
    pub start_coord: Coord,
    pub end_coord: Coord,
}
impl Ship {
    pub fn new(capacity: u8, start: Coord, end: Coord) -> Ship {
        // check if it's a straight line shape
        if start.0 != end.0 && start.1 != end.1 {
            panic!("Wrong ship alignment");
        }

        // set max to length according to the direction
        // Calculate length (distance + 1)
        let len = if start.0 != end.0 {
            // Vertical
            (u8::from(start.0).max(u8::from(end.0)) - u8::from(start.0).min(u8::from(end.0))) + 1
        } else {
            // Horizontal
            (u8::from(start.1).max(u8::from(end.1)) - u8::from(start.1).min(u8::from(end.1))) + 1
        };

        // check if factual length is in bound
        if len > capacity {
            panic!("Ship capacity exceeded");
        }

        Ship {
            capacity,
            parts: vec![FieldCell::Ship(ShipPart::default()); capacity as usize],
            start_coord: Coord::min(start, end),
            end_coord: Coord::max(start, end),
        }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::ship::Ship;

    #[test]
    fn new_creates_valid_horizontal_ship() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(0, 3); // Length 4 (0,1,2,3)
        let capacity = 4;

        // Act
        let ship = Ship::new(capacity, start, end);

        // Assert
        assert_eq!(ship.capacity, capacity);
        // Start/End are normalized by min/max in constructor
        assert_eq!(ship.start_coord, start);
        assert_eq!(ship.end_coord, end);
    }

    #[test]
    fn new_creates_valid_vertical_ship() {
        // Arrange
        let start = Coord::new(3, 0);
        let end = Coord::new(0, 0); // Length 4 (3,2,1,0)
        let capacity = 4;

        // Act
        let ship = Ship::new(capacity, start, end);

        // Assert
        assert_eq!(ship.capacity, capacity);
        // Constructor normalizes start/end using min/max logic
        assert_eq!(ship.start_coord, Coord::new(0, 0));
        assert_eq!(ship.end_coord, Coord::new(3, 0));
    }

    #[test]
    #[should_panic(expected = "Wrong ship alignment")]
    fn new_panics_on_diagonal_alignment() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(1, 1);
        let capacity = 4;

        // Act
        Ship::new(capacity, start, end);
    }

    #[test]
    #[should_panic(expected = "Ship capacity exceeded")]
    fn new_panics_if_length_exceeds_capacity() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(0, 4); // Length 5
        let capacity = 4;

        // Act
        Ship::new(capacity, start, end);
    }
}
