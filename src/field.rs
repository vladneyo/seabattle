#[derive(Debug)]
pub struct Field {
    pub grid: Vec<Vec<bool>>,
}
impl Field {
    pub fn new() -> Field {
        Field{
            grid: vec![vec![true; 10]; 10],
        }
    }
}