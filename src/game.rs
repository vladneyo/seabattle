use std::io;
use crate::display::Display;
use crate::field::Field;
use crate::read_line;

pub struct Game{}
impl Game {
    pub fn new () -> Self {Self{}}

    pub fn run(&self){
        let mut input = String::new();
        let f = Field::new();
        let d = Display::new();

        loop{
            d.draw_grid(&f);
            read_line!(input);

            if input == "stop" { break;}
        }
    }
}