use indextree::{Arena,NodeId};
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (NodeId, Arena<String>) {
    let mut node_map = HashMap::new();
    let mut arena = Arena::new();
    let mut root = None;

    input
        .lines()
        .map(|line| (&line[0..3], &line[4..7]))
        .for_each(|(from, to)| {
            let from_node: NodeId;
            let to_node: NodeId;

            if !node_map.contains_key(from) {
                from_node = arena.new_node(from.to_string());
                node_map.insert(from, from_node);
            } else {
                from_node = *node_map.get(from).unwrap();
            }

            if from == "COM" {
                root = Some(from_node);
            }

            if !node_map.contains_key(to) {
                to_node = arena.new_node(to.to_string());
                node_map.insert(to, to_node);
            } else {
                to_node = *node_map.get(to).unwrap();
            }

            from_node.append(to_node, &mut arena);
        });

    let root = root.expect("No root node detected");
    (root, arena)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &(NodeId, Arena<String>)) -> usize {
    let (root, arena) = input;

    let mut stack = vec![ *root ];
    let mut total = 0;
    loop {
        match stack.pop() {
            Some(node) => {
                let depth = node.ancestors(arena).skip(1).count();
                total += depth;
                node.children(arena).for_each(|c| stack.push(c));
            },
            None => break,
        }
    }

    total
}
