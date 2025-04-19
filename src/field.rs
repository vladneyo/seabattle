use crate::coord::Coord;

#[derive(Debug)]
pub struct Field {
    pub grid: Vec<Vec<bool>>,
}
impl Field {
    pub fn new() -> Field {
        Field{
            grid: vec![vec![true; 10]; 10],
        }
    }

    pub fn strike(&mut self, strike: &Coord) {
        let row: u8 = strike.0.into();
        let col: u8 = strike.1.into();

        self.grid[row as usize][col as usize] = false;
    }
}