use crate::display::Display;

pub trait Cell: Clone + Copy + Default {
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

#[derive(Debug, Clone, Copy)]
pub enum FieldCell {
    Sea(SeaCell),
    Ship(ShipPart),
}

impl Default for FieldCell {
    fn default() -> Self { FieldCell::Sea(SeaCell::default()) }
}

impl Cell for FieldCell {
    fn on_strike(&mut self) { match self {
        FieldCell::Sea(c) => c.on_strike(),
        FieldCell::Ship(c) => c.on_strike(),
    }}
    fn draw(&self) { match self {
        FieldCell::Sea(c) => c.draw(),
        FieldCell::Ship(c) => c.draw(),
    }}

    fn is_hit(&self) -> bool { match self {
        FieldCell::Sea(c) => c.is_hit(),
        FieldCell::Ship(c) => c.is_hit(),
    }}
}

impl FieldCell {
    pub fn to_u8(&self) -> u8 {
        match self {
            FieldCell::Sea(_) => 0,
            FieldCell::Ship(_) => 1,
        }
    }
}

impl From<FieldCell> for u8 {
    fn from(cell: FieldCell) -> Self {
        match cell {
            FieldCell::Sea(_) => 0,
            FieldCell::Ship(_) => 1,
        }
    }
}