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

pub struct DistanceTo<T> {
    pub distance: usize,
    pub item: T,
}

impl <T> DistanceTo<T> {
    pub fn new(item: T, distance: usize) -> DistanceTo<T> {
        DistanceTo { distance, item }
    }
}

#[derive(Clone,Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub struct Span {
    pub from: Point,
    pub to:   Point,
}

impl Span {
    pub fn new(from: Point, to: Point) -> Span {
        Span { from, to }
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
}

impl Span {
    pub fn intersection(&self, other: &Span) -> Option<Point> {
        if self.is_vertical() && other.is_horizontal() {
            let x = self.from.x;
            let y = other.from.y;
            if min(other.from.x, other.to.x) <= x && max(other.from.x, other.to.x) >= x
                    && min(self.from.y, self.to.y) <= y && max(self.from.y, self.to.y) >= y {
                return Some(Point { x, y })
            }
        } else if self.is_horizontal() && other.is_vertical() {
            let x = other.from.x;
            let y = self.from.y;
            if min(self.from.x, self.to.x) <= x && max(self.from.x, self.to.x) >= x
                    && min(other.from.y, other.to.y) <= y && max(other.from.y, other.to.y) >= y {
                return Some(Point { x, y })
            }
        }

        None
    }

    pub fn intersection_distance(&self, other: &Span) -> Option<usize> {
        match self.intersection(other) {
            Some(Point { x, y }) => {
                if self.is_horizontal() {
                    let x_dist = (self.from.x - x).abs() as usize;
                    let y_dist = (other.from.y - y).abs() as usize;
                    Some(x_dist + y_dist)
                } else {
                    let x_dist = (other.from.x - x).abs() as usize;
                    let y_dist = (self.from.y - y).abs() as usize;
                    Some(x_dist + y_dist)
                }
            },
            None => None,
        }
    }
}
