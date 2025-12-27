use crate::coord::Coord;
use crate::field::Field;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ship::Ship;
use rand::Rng;

/// Handles the random generation and placement of ships on a Field.
///
/// Ensures that ships are placed within bounds and do not overlap or touch each other
/// (maintaining a 1-cell buffer zone around every ship).
#[allow(dead_code)]
pub struct ShipSpawner {
    spawn_queue: Vec<(u8, u8)>,
    spawned_ships: Vec<Ship>,
}
impl ShipSpawner {
    pub fn new() -> Self {
        ShipSpawner {
            spawn_queue: vec![(4, 1), (3, 2), (2, 3), (1, 4)], // (blocks, quantity)
            spawned_ships: vec![],
        }
    }

    pub fn spawn(&mut self, field: &mut Field) {
        // `occupancy_matrix` stores ships + 1-cell padding around them to enforce spacing.
        let mut occupancy_matrix = Matrix::create_empty_matrix();
        let mut rng = rand::rng();
        for i in 0..self.spawn_queue.len() {
            // number of ship types
            // println!("{:?}-block:", self.spawn_queue[i].0);
            for _j in 0..self.spawn_queue[i].1 {
                // number of ships within type
                loop {
                    // NOTE: `random_range(0..Point::MAX)` is exclusive of MAX, so we use `..=`.
                    let s_x = rng.random_range(0..=Point::MAX);
                    let s_y = rng.random_range(0..=Point::MAX);
                    let start_coord = Coord::new(s_x, s_y);

                    let end_coord = Self::get_end_coord(start_coord, self.spawn_queue[i].0);
                    let candidate = Ship::new(self.spawn_queue[i].0, start_coord, end_coord);

                    // place candidate footprint (no padding) for intersection check
                    let mut candidate_matrix = Matrix::create_empty_matrix();
                    Matrix::place(
                        &mut candidate_matrix,
                        &candidate.start_coord,
                        &candidate.end_coord,
                    );
                    // Matrix::display(&candidate_matrix);

                    // reject candidate if it intersects any prior ship OR their 1-cell padded border
                    if !Matrix::has_intersection(&occupancy_matrix, &candidate_matrix) {
                        // accept: update padded occupancy matrix and place actual ship on the field
                        Matrix::place_with_padding(
                            &mut occupancy_matrix,
                            &candidate.start_coord,
                            &candidate.end_coord,
                            1,
                        );
                        field.place(&candidate.start_coord, &candidate.end_coord);

                        break;
                    }
                    // if any intersection - regen rng
                }
            }
        }
    }

    #[allow(dead_code)]
    fn place_ship(field: &mut Field, ship: &Ship) {
        field.occupy(ship);
    }

    pub fn get_end_coord(start_coord: Coord, length: u8) -> Coord {
        let candidates = Self::get_all_valid_end_coords(start_coord, length);
        let mut rng = rand::rng();
        candidates[rng.random_range(0..candidates.len())]
    }

    /// Returns all valid end coordinates for a ship of `length` starting at `start_coord`.
    /// This function is deterministic and used for testing ship placement logic.
    pub fn get_all_valid_end_coords(start_coord: Coord, length: u8) -> Vec<Coord> {
        let mut end_points = vec![];
        let end_length = length - 1;

        // check vertical down
        let end = start_coord.0 as u8 + end_length;
        if end <= Point::MAX {
            end_points.push(Coord::new(end, start_coord.1 as u8));
        }

        // check horizontal rightward
        let end = start_coord.1 as u8 + end_length;
        if end <= Point::MAX {
            end_points.push(Coord::new(start_coord.0 as u8, end));
        }

        // check vertical up
        let end = start_coord.0 as i8 - end_length as i8;
        if end >= Point::MIN as i8 {
            end_points.push(Coord::new(end as u8, start_coord.1 as u8));
        }

        // check horizontal leftward
        let end = start_coord.1 as i8 - end_length as i8;
        if end >= Point::MIN as i8 {
            end_points.push(Coord::new(start_coord.0 as u8, end as u8));
        }

        end_points
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::ship_spawner::ShipSpawner;

    #[test]
    fn get_all_valid_end_coords_corner_start() {
        // Arrange
        let start = Coord::new(0, 0);
        let length = 4;

        // Act
        let candidates = ShipSpawner::get_all_valid_end_coords(start, length);

        // Assert: From (0,0) with length 4, we can only go Down (3,0) or Right (0,3).
        // Vertical Up and Horizontal Left are out of bounds.
        let expected = vec![
            Coord::new(3, 0), // Down
            Coord::new(0, 3), // Right
        ];
        // Order depends on implementation (Down, Right, Up, Left), so we check containment or sort.
        // Implementation order: Down, Right.
        assert_eq!(candidates, expected);
    }

    #[test]
    fn get_all_valid_end_coords_center_start() {
        // Arrange
        let start = Coord::new(5, 5);
        let length = 4;

        // Act
        let candidates = ShipSpawner::get_all_valid_end_coords(start, length);

        // Assert: From (5,5) with length 4, all 4 directions are valid.
        // Down: (5+3, 5) -> (8, 5)
        // Right: (5, 5+3) -> (5, 8)
        // Up: (5-3, 5) -> (2, 5)
        // Left: (5, 5-3) -> (5, 2)
        let expected = vec![
            Coord::new(8, 5), // Down
            Coord::new(5, 8), // Right
            Coord::new(2, 5), // Up
            Coord::new(5, 2), // Left
        ];
        assert_eq!(candidates, expected);
    }
}
