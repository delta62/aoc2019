use std::str::FromStr;

use super::cartesian::{Direction,Point};
use super::errors::ParseError;

pub struct WireDirection {
    pub direction: Direction,
    pub length:    i32,
}

impl WireDirection {
    pub fn expand_points(&self, start: &Point) -> Vec<Point> {
        let mut points = Vec::with_capacity(self.length as usize);
        for i in 0..self.length {
            let point = match self.direction {
                Direction::Down  => Point { x: start.x, y: start.y - i },
                Direction::Left  => Point { x: start.x - i, y: start.y },
                Direction::Right => Point { x: start.x + i, y: start.y },
                Direction::Up    => Point { x: start.x, y: start.y + i },
            };
            points.push(point);
        }
        points
    }
}

impl FromStr for WireDirection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, length) = s.split_at(1);
        let direction = direction.parse::<Direction>().expect("Unable to parse direction");
        let length = length.parse::<i32>().expect("Unable to parse length");
        Ok(WireDirection { direction, length })
    }
}

pub type WirePath = Vec<WireDirection>;
