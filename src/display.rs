use crate::field::Field;
use crate::p;
use crate::Point;

#[derive(Debug)]
pub struct Display {}
impl Display {
    pub fn new() -> Display {
        Display {}
    }

    pub fn draw_header(){
        println!("_|0|1|2|3|4|5|6|7|8|9|");
    }

    pub fn draw_row(row_i: usize){
        print!("{}|", char::from(p!(row_i)));
    }

    fn draw_cell(){
        print!("_|");
    }

    pub fn draw_grid(&self, field: &Field){
        Self::draw_header();
        for i in 0..field.grid.len() {
            Self::draw_row(i);
            for j in 0..field.grid[i].len() {
                if field.grid[i][j]{
                    Self::draw_cell();
                }
            }
            print!("\n");

        }
    }

}