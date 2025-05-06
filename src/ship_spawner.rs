use rand::Rng;
use crate::coord::Coord;
use crate::field::{Cell, Field};
use crate::point::Point;
use crate::ship::Ship;

pub struct ShipSpawner{
    spawn_queue: Vec<(usize, u8)>

}
impl ShipSpawner{
    pub fn new() -> Self{
        ShipSpawner{
            spawn_queue: vec![(4, 1), (3, 2), (2, 3), (1, 4)]
        }
    }

    pub fn spawn<T>(&mut self, field: &mut Field<T>)
    where T: Cell + Clone + Copy{
        // todo!("implement spawn")
        let mut rng = rand::rng();
        for i in 0..self.spawn_queue.len() {
            for j in 0..self.spawn_queue[i].1 {
                let s_x = rng.random_range(0..Point::MAX);
                let s_y = rng.random_range(0..Point::MAX);

                let e_x = rng.random_range(0..Point::MAX);
                let e_y = rng.random_range(0..Point::MAX);

                let ship = Ship::new(self.spawn_queue[i].0, Coord::new(s_x, s_y), Coord::new(e_x, e_y));
            }
            self.spawn_queue.remove(i);
        }
    }

    fn create_empty_matrix() -> Vec<Vec<u8>>{
        let matrix = vec![vec![0u8; 10]; 10];

        matrix
    }

    fn place_candidate(matrix: &mut Vec<Vec<u8>>, ship: &mut Ship){

    }
}