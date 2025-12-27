use crate::p;
use crate::point::Point;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(pub Point, pub Point); // (row, column)
impl Coord {
    // (row, column)
    pub fn new(x: u8, y: u8) -> Self {
        Self(p!(x), p!(y))
    }

    pub fn max(c1: Self, c2: Self) -> Self {
        if c1.0 > c2.0 || c1.1 > c2.1 { c1 } else { c2 }
    }

    pub fn min(c1: Self, c2: Self) -> Self {
        if c1.0 < c2.0 || c1.1 < c2.1 { c1 } else { c2 }
    }
}

impl TryFrom<(&char, &u8)> for Coord {
    type Error = String;

    fn try_from((c, n): (&char, &u8)) -> Result<Self, Self::Error> {
        let row = Point::try_from(*c).map_err(|_| "invalid char for Point")?;
        let col = Point::try_from(*n).map_err(|_| "invalid u8 for Point")?;
        Ok(Coord(row, col))
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s[0..2].chars().collect::<Vec<_>>();

        let c = input[0];
        let n = s[1..2].parse::<u8>().map_err(|_| ())?;
        let row = p!(c);
        let col = p!(n);

        Ok(Coord(row, col))
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}",
            <Point as Into<char>>::into(self.0),
            <Point as Into<u8>>::into(self.1)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;

    #[test]
    fn max_returns_element_wise_maximum() {
        // Arrange
        let cases = vec![
            // (c1, c2, expected_max)
            (Coord::new(3, 4), Coord::new(1, 4), Coord::new(3, 4)), // c1.row > c2.row
            (Coord::new(3, 5), Coord::new(3, 4), Coord::new(3, 5)), // c1.col > c2.col
            (Coord::new(4, 5), Coord::new(3, 4), Coord::new(4, 5)), // c1 > c2 (both)
            (Coord::new(1, 4), Coord::new(3, 4), Coord::new(3, 4)), // c2.row > c1.row
        ];

        for (c1, c2, expected) in cases {
            // Act
            let result = Coord::max(c1, c2);

            // Assert
            assert_eq!(
                result, expected,
                "Max of {:?} and {:?} should be {:?}",
                c1, c2, expected
            );
        }
    }

    #[test]
    fn min_returns_element_wise_minimum() {
        // Arrange
        let cases = vec![
            // (c1, c2, expected_min)
            (Coord::new(3, 4), Coord::new(1, 4), Coord::new(1, 4)), // c2.row < c1.row
            (Coord::new(3, 4), Coord::new(3, 2), Coord::new(3, 2)), // c2.col < c1.col
            (Coord::new(3, 4), Coord::new(1, 2), Coord::new(1, 2)), // c2 < c1 (both)
            (Coord::new(1, 4), Coord::new(3, 4), Coord::new(1, 4)), // c1.row < c2.row
        ];

        for (c1, c2, expected) in cases {
            // Act
            let result = Coord::min(c1, c2);

            // Assert
            assert_eq!(
                result, expected,
                "Min of {:?} and {:?} should be {:?}",
                c1, c2, expected
            );
        }
    }
}
