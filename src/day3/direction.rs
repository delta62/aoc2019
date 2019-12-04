use std::str::FromStr;

use super::parseerror::ParseError;

pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            _   => Err(ParseError { }),
        }
    }
}
