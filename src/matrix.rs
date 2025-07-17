use crate::coord::Coord;

#[derive(Debug)]
pub struct Matrix {}
impl Matrix {
    pub fn place(matrix: &mut Vec<Vec<u8>>, start: &Coord, end: &Coord) {
        let max = Coord::max(start.clone().to_owned(), end.clone().to_owned());
        let min = Coord::min(start.clone().to_owned(), end.clone().to_owned());

        if min.0 == max.0 {
            for i in min.1 as u8..=max.1 as u8 {
                matrix[min.0 as usize][i as usize] = 1u8;
            }
        } else {
            for i in min.0 as u8..=max.0 as u8 {
                matrix[i as usize][min.1 as usize] = 1u8;
            }
        }
    }

    fn multiply(matrix1: &Vec<Vec<u8>>, matrix2: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut result = vec![vec![0u8; 10]; 10];

        for i in 0..matrix1.len() {
            for j in 0..matrix1[i].len() {
                result[i][j] = matrix1[i][j] * matrix2[i][j];
            }
        }

        result
    }

    pub fn has_intersection(matrix1: &Vec<Vec<u8>>, matrix2: &Vec<Vec<u8>>) -> bool {
        let result = Self::multiply(matrix1, matrix2);

        result.iter().any(|row| row.contains(&1u8))
    }

    #[allow(unused)]
    pub fn display(matrix: &Vec<Vec<u8>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                print!("{}", matrix[i][j]);
            }
            println!();
        }
    }
}
