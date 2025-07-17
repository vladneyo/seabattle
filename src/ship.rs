use crate::cell::{FieldCell, ShipPart};
use crate::coord::Coord;

#[derive(Debug)]
pub struct Ship{
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
        let max = if start.0 != end.0 {
            u8::from(start.0).max(u8::from(end.0)) - u8::from(start.0).min(u8::from(end.0))
        } else {
            u8::from(start.1).max(u8::from(end.1)) - u8::from(start.1).max(u8::from(end.1))
        };

        // check of factual length is in bound
        if max > capacity {
            panic!("Ship capacity exceeded");
        }

        Ship{
            capacity,
            parts: vec![FieldCell::Ship(ShipPart::default()); capacity as usize],
            start_coord: Coord::min(start, end),
            end_coord: Coord::max(start, end),

        }
    }
}



#[cfg(test)]
mod tests{

}