use indextree::{Arena,NodeId};
use std::collections::{HashMap,HashSet};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Galaxy {
    let mut node_map = HashMap::new();
    let mut planets  = Arena::new();

    let mut root     = None;
    let mut you      = None;

    input
        .lines()
        .map(|line| (&line[0..3], &line[4..7]))
        .for_each(|(from, to)| {
            let from_node: NodeId;
            let to_node: NodeId;

            if !node_map.contains_key(from) {
                let planet = Planet::from_str(from);

                let is_root = planet == Planet::COM;
                let is_you  = planet == Planet::YOU;

                from_node = planets.new_node(planet);
                node_map.insert(from, from_node);

                if is_root {
                    root = Some(from_node);
                }
                if is_you {
                    you = Some(from_node);
                }

            } else {
                from_node = *node_map.get(from).unwrap();
            }

            if !node_map.contains_key(to) {
                let planet = Planet::from_str(to);
                let is_you = planet == Planet::YOU;
                to_node = planets.new_node(planet);
                node_map.insert(to, to_node);

                if is_you {
                    you = Some(to_node);
                }
            } else {
                to_node = *node_map.get(to).unwrap();
            }

            from_node.append(to_node, &mut planets);
        });

    let root = root.expect("No root node detected");
    let you  = you.expect("No YOU node detected");

    Galaxy { planets, root, you, }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Galaxy) -> usize {
    let root = input.root;
    let arena = &input.planets;

    let mut stack = vec![ (root, 0) ];
    let mut total = 0;
    loop {
        match stack.pop() {
            Some((node, depth)) => {
                total += depth;
                node.children(&arena).for_each(|c| stack.push((c, depth + 1)));
            },
            None => break,
        }
    }

    total
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Galaxy) -> usize {
    let you     = input.you;
    let planets = &input.planets;

    let mut visited = HashSet::new();
    let mut stack   = vec![ (you, 0) ];
    let mut paths   = vec![ ];

    loop {
        match stack.pop() {
            Some((node, dist)) => {
                // Skip nodes that have already been visited
                if let Some(_) = visited.get(&node) {
                    continue;
                } else {
                    visited.insert(node);
                }

                let n = planets.get(node).unwrap();
                if *n.get() == Planet::SAN {
                    paths.push(dist);
                } else {
                    node.children(&planets).for_each(|c| stack.push((c, dist + 1)));
                    if let Some(n) = n.parent() {
                        stack.push((n, dist + 1));
                    }
                }
            },
            None => break,
        }
    }

    let min: usize = paths.into_iter().min().expect("No paths found");
    min - 2
}

#[derive(PartialEq)]
pub enum Planet {
    COM,
    YOU,
    SAN,
    UNK,
}

impl Planet {
    fn from_str(s: &str) -> Planet {
        match s {
            "COM" => Planet::COM,
            "YOU" => Planet::YOU,
            "SAN" => Planet::SAN,
            _     => Planet::UNK,
        }
    }
}

pub struct Galaxy {
    pub planets: Arena<Planet>,
    pub root:    NodeId,
    pub you:     NodeId,
}
