use std::cmp::{min,max};
use std::str::FromStr;

use super::errors::ParseError;

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

#[derive(Clone,Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Span {
    pub from: Point,
    pub to:   Point,
}

impl Span {
    pub fn intersection(&self, other: &Span) -> Option<Point> {
        if self.from.x == self.to.x {
            // self is vertical
            // other is horizontal
            let x = self.from.x;
            let y = other.from.y;
            if min(other.from.x, other.to.x) <= x && max(other.from.x, other.to.x) >= x
            && min(self.from.y, self.to.y) <= y && max(self.from.y, self.to.y) >= y {
                return Some(Point { x, y })
            }
            return None
        } else {
            // self is horizontal
            // other is vertical
            let x = other.from.x;
            let y = self.from.y;
            if min(self.from.x, self.to.x) <= x && max(self.from.x, self.to.x) >= x
            && min(other.from.y, other.to.y) <= y && max(other.from.y, other.to.y) >= y {
                return Some(Point { x, y })
            }
            return None
        }
    }
}

