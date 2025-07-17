use crate::coord::Coord;
use crate::field::Field;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ship::Ship;
use rand::Rng;

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
        // todo!("implement spawn")
        let mut inter_matrix = Self::create_empty_matrix();
        let mut rng = rand::rng();
        for i in 0..self.spawn_queue.len() {
            // number of ship types
            // println!("{:?}-block:", self.spawn_queue[i].0);
            for j in 0..self.spawn_queue[i].1 {
                // number of ships within type
                loop {
                    let s_x = rng.random_range(0..Point::MAX);
                    let s_y = rng.random_range(0..Point::MAX);
                    let start_coord = Coord::new(s_x, s_y);

                    let end_coord = Self::get_end_coord(start_coord, self.spawn_queue[i].0);
                    let candidate = Ship::new(self.spawn_queue[i].0, start_coord, end_coord);

                    // place candidate on matrix
                    let mut candidate_matrix = Self::create_empty_matrix();
                    Matrix::place(
                        &mut candidate_matrix,
                        &candidate.start_coord,
                        &candidate.end_coord,
                    );
                    // Matrix::display(&candidate_matrix);
                    // println!("{:?}", j);

                    // check intersection
                    if !Matrix::has_intersection(&inter_matrix, &candidate_matrix) {
                        // if no intersection -> update matrix + update field
                        // todo!("implement")
                        Matrix::place(
                            &mut inter_matrix,
                            &candidate.start_coord,
                            &candidate.end_coord,
                        );

                        field.place(&candidate.start_coord, &candidate.end_coord);

                        break;
                    }
                    // if any intersection - regen rng
                }
            }
        }
    }

    fn create_empty_matrix() -> Vec<Vec<u8>> {
        vec![vec![0u8; 10]; 10]
    }

    fn place_ship(field: &mut Field, ship: &Ship) {
        field.occupy(ship);
    }

    pub fn get_end_coord(start_coord: Coord, length: u8) -> Coord {
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

        let mut rng = rand::rng();

        // get random direction's end coord
        end_points[rng.random_range(0..end_points.len())]
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::ship_spawner::ShipSpawner;

    #[test]
    fn get_end_coord_zero_zero_four_down_or_right() {
        let start = Coord::new(0, 0);
        let length = 4;

        let end = ShipSpawner::get_end_coord(start, length);

        assert!(end == Coord::new(0, 3) || end == Coord::new(3, 0));
    }

    #[test]
    fn get_end_coord_five_five_four_any() {
        let start = Coord::new(5, 5);
        let length = 4;

        let end = ShipSpawner::get_end_coord(start, length);
        println!("{:?}", end);
        assert!(
            end == Coord::new(5, 8)
                || end == Coord::new(8, 5)
                || end == Coord::new(2, 5)
                || end == Coord::new(5, 2)
        );
    }
}
