#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Point {
    Zero  = 0,
    One   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
}

impl Point {
    pub const MAX: u8 = 9;
    pub const MIN: u8 = 0;
}

impl From<Point> for u8 {
    fn from(p: Point) -> u8 { p as u8 }
}

impl From<Point> for char {
    fn from(p: Point) -> char
    {
        match p {
            Point::Zero => 'a',
            Point::One   => 'b',
            Point::Two   => 'c',
            Point::Three => 'd',
            Point::Four  => 'e',
            Point::Five  => 'f',
            Point::Six   => 'g',
            Point::Seven => 'h',
            Point::Eight => 'i',
            Point::Nine  => 'j',
        }
    }
}

impl TryFrom<u8> for Point {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Point::Zero),
            1 => Ok(Point::One),
            2 => Ok(Point::Two),
            3 => Ok(Point::Three),
            4 => Ok(Point::Four),
            5 => Ok(Point::Five),
            6 => Ok(Point::Six),
            7 => Ok(Point::Seven),
            8 => Ok(Point::Eight),
            9 => Ok(Point::Nine),
            x => Err(x),
        }
    }
}

impl TryFrom<usize> for Point {
    type Error = usize;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Point::Zero),
            1 => Ok(Point::One),
            2 => Ok(Point::Two),
            3 => Ok(Point::Three),
            4 => Ok(Point::Four),
            5 => Ok(Point::Five),
            6 => Ok(Point::Six),
            7 => Ok(Point::Seven),
            8 => Ok(Point::Eight),
            9 => Ok(Point::Nine),
            x => Err(x),
        }
    }
}

impl TryFrom<char> for Point {
    type Error = char;
    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'a' => Ok(Point::Zero),
            'b' => Ok(Point::One),
            'c' => Ok(Point::Two),
            'd' => Ok(Point::Three),
            'e' => Ok(Point::Four),
            'f' => Ok(Point::Five),
            'g' => Ok(Point::Six),
            'h' => Ok(Point::Seven),
            'i' => Ok(Point::Eight),
            'j' => Ok(Point::Nine),
            x  => Err(x),
        }
    }
}

#[macro_export]
macro_rules! p {
    ($n:literal) => {
        Point::try_from($n).unwrap()
    };
    ($n:ident) => {
        Point::try_from($n).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn u8_valid_bounds(){
        let p = p!(3u8);

        assert_eq!(p, Point::Three);
    }

    #[test]
    #[should_panic]
    fn u8_invalid_bounds(){
        let _p = p!(10u8);
    }

    #[test]
    fn char_valid_bounds(){
        let p = p!('a');

        assert_eq!(p, Point::Zero);
    }

    #[test]
    #[should_panic]
    fn char_invalid_bounds(){
        let _p = p!('k');
    }

    #[test]
    fn convert_u8_to_point(){
        let p:u8 = p!(0u8).into();

        assert_eq!(p, 0u8);
    }

    #[test]
    fn convert_char_to_point(){
        let p:char = p!('a').into();

        assert_eq!(p, 'a');
    }
}