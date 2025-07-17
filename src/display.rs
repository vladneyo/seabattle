use colored::Colorize;
use crate::cell::Cell;
use crate::coord::Coord;
use crate::field::{Field};
use crate::p;
use crate::Point;

#[derive(Debug)]
pub struct Display {}
impl Display {
    pub fn new() -> Display {
        Display {}
    }

    pub fn draw_your_field_label(&self){
        println!("------YOUR FIELD------");
    }

    pub fn draw_enemy_field_label(&self){
        println!("-----ENEMY FIELD------");
    }
    pub fn draw_header(){
        println!("_|0|1|2|3|4|5|6|7|8|9|");
    }

    pub fn draw_row(row_i: usize){
        print!("{}|", char::from(p!(row_i)));
    }

    pub fn draw_empty_cell(){
        print!("{}", "_|".blue());
    }

    pub fn draw_miss_cell(){
        print!("{}{}", "•".yellow(), "|".blue());
    }

    pub fn draw_hit_cell(){
        print!("{}{}", "X".custom_color((255, 165, 0)), "|".blue());
    }

    pub fn draw_destoyed_cell(){
        print!("{}{}", "X".red(), "|".blue());
    }

    pub fn draw_ship_cell(){
        print!("{}{}", "#", "|".blue());
    }

    pub fn draw_grid(&self, field: &Field) {
        Self::draw_header();
        for i in 0..field.grid.len() {
            Self::draw_row(i);
            for j in 0..field.grid[i].len() {
                field.grid[i][j].draw();
            }
            print!("\n");

        }
    }

    pub fn draw_user_history(&self, history: &Vec<Coord>){
        if history.len() > 0 {
            println!("Your strikes:");
            for coord in history{
                print!("{} ", coord);
            }
            print!("\n")
        }
    }

}