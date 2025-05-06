use crate::coord::Coord;
use crate::display::Display;
use crate::field::Cell;

#[derive(Debug)]
pub struct Ship{
    capacity: usize,
    parts: Vec<ShipPart>,
    start_coord: Coord,
    end_coord: Coord,
}
impl Ship {
    pub fn new(capacity: usize, start: Coord, end: Coord) -> Ship {
        // check if it's a straight line shape
        if start.0 != end.0 || start.1 != end.1 {
            panic!("Wrong ship alignment");
        }

        // set max to length according to direction
        let max = if start.0 != end.0 {
            (u8::from(start.0).max(u8::from(end.0)) - u8::from(start.0).min(u8::from(end.0))) as usize
        } else {
            (u8::from(start.1).max(u8::from(end.1)) - u8::from(start.1).max(u8::from(end.1))) as usize
        };

        // check of factual length is in bound
        if max > capacity {
            panic!("Ship capacity exceeded");
        }

        Ship{
            capacity,
            parts: vec![ShipPart::default(); capacity],
            start_coord: Coord::min(start, end),
            end_coord: Coord::max(start, end),

        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShipPart{
    hit: bool,
}

impl ShipPart{
    pub fn new() -> ShipPart{
        ShipPart{
            hit: false,
        }
    }
}

impl Default for ShipPart {
    fn default() -> Self {
        Self::new()
    }
}

impl Cell for ShipPart{
    fn on_strike(&mut self) {
        self.hit = true;
    }

    fn draw(&self) {
        if self.hit{
            Display::draw_hit_cell();
        }
        else {
            Display::draw_ship_cell()
        }

    }

    fn is_hit(&self) -> bool {
        self.hit
    }
}

#[cfg(test)]
mod tests{

}