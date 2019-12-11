use indextree::{Arena,NodeId};
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Galaxy {
    let mut node_map = HashMap::new();
    let mut planets  = Arena::new();
    let mut root     = None;

    input
        .lines()
        .map(|line| (&line[0..3], &line[4..7]))
        .for_each(|(from, to)| {
            let from_node: NodeId;
            let to_node: NodeId;

            if !node_map.contains_key(from) {
                let planet = Planet::from_str(from);
                let is_root = planet == Planet::COM;
                from_node = planets.new_node(planet);
                node_map.insert(from, from_node);
                if is_root {
                    root = Some(from_node);
                }
            } else {
                from_node = *node_map.get(from).unwrap();
            }

            if !node_map.contains_key(to) {
                to_node = planets.new_node(Planet::from_str(to));
                node_map.insert(to, to_node);
            } else {
                to_node = *node_map.get(to).unwrap();
            }

            from_node.append(to_node, &mut planets);
        });

    let root = root.expect("No root node detected");

    Galaxy { planets, root }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Galaxy) -> usize {
    let root = input.root;
    let arena = &input.planets;

    let mut stack = vec![ root ];
    let mut total = 0;
    loop {
        match stack.pop() {
            Some(node) => {
                let depth = node.ancestors(&arena).skip(1).count();
                total += depth;
                node.children(&arena).for_each(|c| stack.push(c));
            },
            None => break,
        }
    }

    total
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
}

//#[aoc(day6, part2)]
//pub fn solve_part2(input: &(NodeId, Arena<String>)) -> usize {
//    let (root, arena) = input;
//
//    // first find either YOU or SAN
//    let you = find("YOU", *root, arena).expect("Unable to find YOU node");
//
//    // next traverse from there to the other
//    let distance = distance_from(you, "SAN", arena).expect("Unable to find a path");
//    distance
//}

// fn find(data: &str, root: NodeId, arena: &Arena<String>) -> Option<NodeId> {
//     let mut stack = vec![ root ];
//
//     loop {
//         match stack.pop() {
//             Some(node) => {
//                 let d = arena.get(node).unwrap().get();
//                 match d {
//                     s if s == data => return Some(node),
//                     _ => node.children(arena).for_each(|c| stack.push(c)),
//                 }
//             },
//             None => break,
//         }
//     }
//
//     None
// }
//
// fn distance_from(from: NodeId, to: &str, arena: &Arena<String>) -> Option<usize> {
//     None
// }
