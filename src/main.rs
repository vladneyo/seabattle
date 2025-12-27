mod cell;
mod console;
mod coord;
mod display;
mod field;
mod game;
mod matrix;
mod point;
mod ship;
mod ship_spawner;

use crate::game::Game;
use point::Point;

fn main() {
    let mut game = Game::new();
    game.run();
}
