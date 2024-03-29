mod cartesian;
mod errors;
mod wire;

use cartesian::{Direction,DistanceTo,Point,Span};
use wire::{WireDirection,WirePath};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<WirePath> {
    input
        .lines()
        .map(|line| {
            line
                .split(',')
                .map(|s| s.parse::<WireDirection>().expect("Unable to parse wire direction"))
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[WirePath]) -> i32 {
    let path1 = input.get(0).expect("input data not long enough");
    let path2 = input.get(1).expect("input data not long enough");
    let mut spans = vec![];

    let mut xloc = 0;
    let mut yloc = 0;

    // trace path1 & add to spans
    for d in path1 {
        let from = Point::new(xloc, yloc);
        match d.direction {
            Direction::Down  => yloc -= d.length as i32,
            Direction::Left  => xloc -= d.length as i32,
            Direction::Right => xloc += d.length as i32,
            Direction::Up    => yloc += d.length as i32,
        }
        let to = Point::new(xloc, yloc);
        spans.push(Span::new(from, to));
    }

    // trace path 2 & make a list of intersections
    xloc = 0;
    yloc = 0;
    let mut intersections = vec![];

    for d in path2 {
        let from = Point::new(xloc, yloc);
        match d.direction {
            Direction::Down  => yloc -= d.length as i32,
            Direction::Left  => xloc -= d.length as i32,
            Direction::Right => xloc += d.length as i32,
            Direction::Up    => yloc += d.length as i32,
        }
        let to = Point::new(xloc, yloc);
        let span = Span::new(from, to);

        spans.iter().for_each(|s| {
            let intersection = span.intersection(s);
            if let Some(p) = intersection {
                let distance = p.x.abs() + p.y.abs();
                intersections.push(distance);
            }
        });
    }

    intersections
        .into_iter()
        .filter(|x| *x != 0)
        .min()
        .expect("No intersections found")
}

#[aoc(day3, part2)]
fn solve_part2(input: &[WirePath]) -> usize {
    let path1 = input.get(0).expect("input data not long enough");
    let path2 = input.get(1).expect("input data not long enough");
    let mut spans = vec![];

    let mut xloc = 0;
    let mut yloc = 0;
    let mut distance: usize = 0;

    // trace path1 & add to spans
    for d in path1 {
        let from = Point::new(xloc, yloc);
        match d.direction {
            Direction::Down  => yloc -= d.length as i32,
            Direction::Left  => xloc -= d.length as i32,
            Direction::Right => xloc += d.length as i32,
            Direction::Up    => yloc += d.length as i32,
        }
        let to = Point::new(xloc, yloc);
        spans.push(DistanceTo::new(Span::new(from, to), distance));
        distance += d.length;
    }

    // trace path 2 & make a list of intersections
    xloc = 0;
    yloc = 0;
    distance = 0;
    let mut intersections = vec![];

    for d in path2 {
        let from = Point::new(xloc, yloc);
        match d.direction {
            Direction::Down  => yloc -= d.length as i32,
            Direction::Left  => xloc -= d.length as i32,
            Direction::Right => xloc += d.length as i32,
            Direction::Up    => yloc += d.length as i32,
        }
        let to = Point::new(xloc, yloc);
        let span = Span::new(from, to);

        // Check every span in path 1 for an intersection
        for distance_to in spans.iter() {
            let wire1_dist = distance_to.distance;
            let wire1_span = &distance_to.item;
            let intersection = span.intersection_distance(wire1_span);

            if let Some(dist) = intersection {
                let total_dist = wire1_dist + distance + dist;
                intersections.push(total_dist);
            }
        }
        distance += d.length;
    }

    intersections
        .into_iter()
        .filter(|x| *x != 0)
        .min()
        .expect("No intersections found") as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1ex1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve_part1(&input_generator(input)), 159);
    }

    #[test]
    fn p1ex2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve_part1(&input_generator(input)), 135);
    }

    #[test]
    fn p2ex1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve_part2(&input_generator(input)), 610);
    }

    #[test]
    fn p2ex2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve_part2(&input_generator(input)), 410);
    }
}
