use crate::console::Console;
use crate::coord::Coord;
use crate::display::Display;
use crate::field::Field;
use crate::read_line;

pub struct Game{
    display: Display,
    user_field: Field,
    enemy_field: Field,
    user_strike_history: Vec<Coord>,
    enemy_strike_history: Vec<Coord>,
}
impl Game {
    pub fn new () -> Self {Self{
        display: Display::new(),
        user_field: Field::new(),
        enemy_field: Field::new(),
        user_strike_history: Vec::with_capacity(100),
        enemy_strike_history: Vec::with_capacity(100),
    }}

    pub fn run(&mut self) {
        let mut input = String::new();

        loop{
            Console::clear_screen();

            self.display.draw_your_field();
            self.display.draw_grid(&self.user_field);

            self.display.draw_enemy_field();
            self.display.draw_grid(&self.enemy_field);

            self.display.draw_user_history(&self.user_strike_history);
            input.clear();

            read_line!(input);

            if input == "stop" {
                break;
            }

            let c:Coord = input.parse().unwrap();
            self.enemy_field.strike(&c);
            self.user_strike_history.push(c);
        }
    }
}