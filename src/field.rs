use crate::coord::Coord;
use crate::display::Display;

#[derive(Debug)]
pub struct Field<T: Cell + Clone + Copy> {
    pub grid: Vec<Vec<T>>,
}
impl<T: Cell + Clone + Copy + Default> Field<T> {
    pub fn new() -> Field<T> {
        Field{
            grid: vec![vec![T::default(); 10]; 10],
        }
    }

    pub fn strike(&mut self, strike: &Coord) {
        let row: u8 = strike.0.into();
        let col: u8 = strike.1.into();

        self.grid[row as usize][col as usize].on_strike();
    }
}

pub trait Cell {
    fn on_strike(&mut self);
    fn draw(&self);
    fn is_hit(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct SeaCell{
    hit: bool,
}
impl SeaCell{
    fn new() -> SeaCell{
        SeaCell{hit: false}
    }
}

impl Default for SeaCell {
    fn default() -> Self {
        Self::new()
    }
}

impl Cell for SeaCell{
    fn on_strike(&mut self) {
        self.hit = true;
    }

    fn draw(&self) {
        if self.hit{
            Display::draw_miss_cell()
        }
        else {
            Display::draw_empty_cell()
        }
    }

    fn is_hit(&self) -> bool{
        self.hit
    }
}