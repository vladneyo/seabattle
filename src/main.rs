mod display;
mod point;
mod field;
mod game;
mod console;

use point::Point;
use crate::display::Display;
use crate::field::Field;
use crate::game::Game;

fn main() {
    // println!("{:?}", p!(0));
    // let u: u8 = p!('a').into();
    // println!("{:?}", u);
    //
    // let c:char = p!('b').into();
    // println!("{:?}", c);
    let game = Game::new();
    game.run();
}
