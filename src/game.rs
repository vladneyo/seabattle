use crate::console::Console;
use crate::coord::Coord;
use crate::display::Display;
use crate::field::Field;
use crate::read_line;
use crate::ship_spawner::ShipSpawner;

pub struct Game {
    display: Display,
    user_field: Field,
    enemy_field: Field,
    user_strike_history: Vec<Coord>,
    enemy_strike_history: Vec<Coord>,
}
impl Game {
    pub fn new() -> Self {
        let mut spawner = ShipSpawner::new();
        let mut user_field = Field::new();
        let mut enemy_field = Field::new();
        println!("user spawn");
        spawner.spawn(&mut user_field);
        println!("enemy spawn");
        spawner.spawn(&mut enemy_field);

        Self {
            display: Display::new(),
            user_field,
            enemy_field,
            user_strike_history: Vec::with_capacity(100),
            enemy_strike_history: Vec::with_capacity(100),
        }
    }

    pub fn run(&mut self) {
        let mut input = String::new();

        loop {
            Console::clear_screen();

            self.display.draw_your_field_label();
            self.display.draw_grid(&self.user_field);

            self.display.draw_enemy_field_label();
            self.display.draw_grid(&self.enemy_field);

            self.display.draw_user_history(&self.user_strike_history);
            input.clear();

            read_line!(input);

            if input == "stop" {
                break;
            }

            let c: Coord = input.parse().unwrap();
            self.enemy_field.strike(&c);
            self.user_strike_history.push(c);
        }
    }
}
