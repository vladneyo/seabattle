mod display;
mod point;
mod field;
mod game;
mod console;
mod coord;

use crate::game::Game;
use point::Point;

fn main() {
    let mut game = Game::new();
    game.run();
}
