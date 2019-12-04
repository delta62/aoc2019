use std::cmp::{min,max};
use std::str::FromStr;
use super::direction::Direction;
use super::parseerror::ParseError;

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

#[derive(Clone,Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Span {
    pub from: Point,
    pub to: Point,
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
