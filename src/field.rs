use crate::cell::{Cell, FieldCell, ShipPart};
use crate::coord::Coord;
use crate::ship::Ship;

#[derive(Debug)]
pub struct Field {
    pub grid: Vec<Vec<FieldCell>>,
}
impl Field {
    pub fn new() -> Field {
        Field {
            grid: vec![vec![FieldCell::default(); 10]; 10],
        }
    }

    pub fn strike(&mut self, strike: &Coord) {
        let row: u8 = strike.0.into();
        let col: u8 = strike.1.into();

        self.grid[row as usize][col as usize].on_strike();
    }

    pub fn place(&mut self, start: &Coord, end: &Coord) {
        let max = Coord::max(start.clone().to_owned(), end.clone().to_owned());
        let min = Coord::min(start.clone().to_owned(), end.clone().to_owned());

        if min.0 == max.0 {
            for i in min.1 as u8..=max.1 as u8 {
                self.grid[min.0 as usize][i as usize] = FieldCell::Ship(ShipPart::default());
            }
        } else {
            for i in min.0 as u8..=max.0 as u8 {
                self.grid[i as usize][min.1 as usize] = FieldCell::Ship(ShipPart::default());
            }
        }
    }

    #[allow(dead_code)]
    pub fn occupy(&mut self, _ship: &Ship) {
        println!("occupied")

        //ship.start_coord
    }
}
