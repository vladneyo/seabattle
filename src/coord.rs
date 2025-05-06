use crate::p;
use crate::point::Point;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(pub Point, pub Point);
impl Coord{
    pub fn new(x: u8, y: u8) -> Self {
        Self{ 0: p!(x), 1: p!(y) }
    }

    pub fn max(c1: Self, c2: Self) -> Self {
        if c1.0 > c2.0 || c1.1 > c2.1 {
            c1
        }
        else {
            c2
        }
    }

    pub fn min(c1: Self, c2: Self) -> Self {
        if c1.0 < c2.0 || c1.1 < c2.1 {
            c1
        }
        else {
            c2
        }
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

#[cfg(test)]
mod tests {
    use crate::coord::Coord;

    #[test]
    fn max_correct(){
        // c1 is righter
        let c1 = Coord::new(3u8, 4u8);
        let c2 = Coord::new(1u8, 4u8);

        assert_eq!(Coord::max(c1, c2), Coord::new(3, 4));

        // c1 is lower
        let c1 = Coord::new(3u8, 5u8);
        let c2 = Coord::new(3u8, 4u8);

        assert_eq!(Coord::max(c1, c2), Coord::new(3, 5));

        // c1 is lower and righter
        let c1 = Coord::new(4u8, 5u8);
        let c2 = Coord::new(3u8, 4u8);

        assert_eq!(Coord::max(c1, c2), Coord::new(4, 5));
    }

    #[test]
    fn min_correct(){
        // c2 is lefter
        let c1 = Coord::new(3u8, 4u8);
        let c2 = Coord::new(1u8, 4u8);

        assert_eq!(Coord::min(c1, c2), Coord::new(1, 4));

        // c2 is higher
        let c1 = Coord::new(3u8, 4u8);
        let c2 = Coord::new(3u8, 2u8);

        assert_eq!(Coord::min(c1, c2), Coord::new(3, 2));

        // c2 is higher and lefter
        let c1 = Coord::new(3u8, 4u8);
        let c2 = Coord::new(1u8, 2u8);

        assert_eq!(Coord::min(c1, c2), Coord::new(1, 2));
    }
}
