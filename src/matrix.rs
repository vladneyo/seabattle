use crate::coord::Coord;

/// A 10x10 boolean matrix used for collision detection and ship placement validation.
///
/// It supports placing "footprints" of ships and calculating intersections.
/// It also supports placing with padding to enforce the rule that ships cannot touch.
#[derive(Debug)]
pub struct Matrix {}
impl Matrix {
    const SIZE: usize = 10;

    pub fn create_empty_matrix() -> Vec<Vec<u8>> {
        vec![vec![0u8; Self::SIZE]; Self::SIZE]
    }
    pub fn place(matrix: &mut [Vec<u8>], start: &Coord, end: &Coord) {
        let max = Coord::max(start.clone().to_owned(), end.clone().to_owned());
        let min = Coord::min(start.clone().to_owned(), end.clone().to_owned());

        // vertical
        if min.0 == max.0 {
            for i in min.1 as u8..=max.1 as u8 {
                matrix[min.0 as usize][i as usize] = 1u8;
            }
        // horizontal
        } else {
            for i in min.0 as u8..=max.0 as u8 {
                matrix[i as usize][min.1 as usize] = 1u8;
            }
        }
    }

    /// Places a ship footprint on `matrix` and also fills a `padding`-cell border around it.
    ///
    /// This is used to enforce the classic "ships can't touch" rule (including diagonals)
    /// by doing intersection checks against a padded occupancy matrix.
    pub fn place_with_padding(matrix: &mut [Vec<u8>], start: &Coord, end: &Coord, padding: u8) {
        let max = Coord::max(start.clone().to_owned(), end.clone().to_owned());
        let min = Coord::min(start.clone().to_owned(), end.clone().to_owned());

        // Collect all footprint cells (row, col) as i16 for easy +/- padding math.
        let mut cells: Vec<(i16, i16)> = Vec::new();

        // vertical
        if min.0 == max.0 {
            let row = min.0 as i16;
            for col in min.1 as u8..=max.1 as u8 {
                cells.push((row, col as i16));
            }
        // horizontal
        } else {
            let col = min.1 as i16;
            for row in min.0 as u8..=max.0 as u8 {
                cells.push((row as i16, col));
            }
        }

        let pad = padding as i16;
        for (r, c) in cells {
            for dr in -pad..=pad {
                for dc in -pad..=pad {
                    let rr = r + dr;
                    let cc = c + dc;
                    if rr >= 0
                        && cc >= 0
                        && (rr as usize) < Self::SIZE
                        && (cc as usize) < Self::SIZE
                    {
                        matrix[rr as usize][cc as usize] = 1u8;
                    }
                }
            }
        }
    }

    fn multiply(matrix1: &[Vec<u8>], matrix2: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut result = Self::create_empty_matrix();

        for i in 0..matrix1.len() {
            for j in 0..matrix1[i].len() {
                result[i][j] = matrix1[i][j] * matrix2[i][j];
            }
        }

        result
    }

    pub fn has_intersection(matrix1: &[Vec<u8>], matrix2: &[Vec<u8>]) -> bool {
        let result = Self::multiply(matrix1, matrix2);

        result.iter().any(|row| row.contains(&1u8))
    }

    #[allow(unused)]
    pub fn display(matrix: &[Vec<u8>]) {
        for row in matrix {
            for &val in row {
                print!("{}", val);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::matrix::Matrix;

    #[test]
    fn place_with_padding_marks_neighbors_for_single_cell_ship() {
        // Arrange
        let mut m = Matrix::create_empty_matrix();
        let start = Coord::new(5, 5);
        let end = Coord::new(5, 5);

        // Act
        Matrix::place_with_padding(&mut m, &start, &end, 1);

        // Assert (3x3 block centered at (5,5) must be 1s)
        for r in 4..=6 {
            for c in 4..=6 {
                assert_eq!(
                    m[r][c], 1u8,
                    "expected cell ({},{}) to be padded/occupied",
                    r, c
                );
            }
        }
        // A cell just outside the padding should remain 0
        assert_eq!(m[3][5], 0u8);
    }

    #[test]
    fn padded_intersection_rejects_adjacent_ships() {
        // Arrange
        let mut occupancy = Matrix::create_empty_matrix();
        let ship1_start = Coord::new(0, 0);
        let ship1_end = Coord::new(0, 0);
        Matrix::place_with_padding(&mut occupancy, &ship1_start, &ship1_end, 1);

        // Candidate ship is adjacent (touching the padded border)
        let mut candidate = Matrix::create_empty_matrix();
        let ship2_start = Coord::new(0, 1);
        let ship2_end = Coord::new(0, 1);
        Matrix::place(&mut candidate, &ship2_start, &ship2_end);

        // Act
        let intersects = Matrix::has_intersection(&occupancy, &candidate);

        // Assert
        assert!(intersects);
    }

    #[test]
    fn padded_intersection_allows_one_cell_gap() {
        // Arrange
        let mut occupancy = Matrix::create_empty_matrix();
        let ship1_start = Coord::new(0, 0);
        let ship1_end = Coord::new(0, 0);
        Matrix::place_with_padding(&mut occupancy, &ship1_start, &ship1_end, 1);

        // Candidate ship has a one-cell gap from the padded border
        let mut candidate = Matrix::create_empty_matrix();
        let ship2_start = Coord::new(0, 2);
        let ship2_end = Coord::new(0, 2);
        Matrix::place(&mut candidate, &ship2_start, &ship2_end);

        // Act
        let intersects = Matrix::has_intersection(&occupancy, &candidate);

        // Assert
        assert!(!intersects);
    }
}
