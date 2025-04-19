use crate::p;
use crate::point::Point;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(pub Point, pub Point);

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
        let n = u8::from_str_radix(&s[1..2], 10).map_err(|_| ())?;
        let row = p!(c);
        let col = p!(n);

        Ok(Coord(row, col))
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", <Point as Into<char>>::into(self.0), <Point as Into<u8>>::into(self.1))
    }
}
