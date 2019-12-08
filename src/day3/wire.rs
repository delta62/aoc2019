use std::str::FromStr;

use super::cartesian::Direction;
use super::errors::ParseError;

pub struct WireDirection {
    pub direction: Direction,
    pub length:    usize,
}

impl FromStr for WireDirection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, length) = s.split_at(1);
        let direction = direction.parse::<Direction>().expect("Unable to parse direction");
        let length = length.parse::<usize>().expect("Unable to parse length");
        Ok(WireDirection { direction, length })
    }
}

pub type WirePath = Vec<WireDirection>;
