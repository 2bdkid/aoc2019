use std::collections::HashMap;
use std::collections::HashSet;

pub type Tree = HashMap<String, String>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Tree {
    input
        .lines()
        .map(|l| l.split(')'))
        .map(|mut i| (i.next().unwrap().to_string(), i.next().unwrap().to_string()))
        .map(|(p, c)| (c, p))
        .collect()
}

fn depth(node: &String, tree: &Tree) -> i32 {
    let mut node = node;
    let mut depth = 0;

    while let Some(parent) = tree.get(node) {
        depth += 1;
        node = parent;
    }

    depth
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Tree) -> i32 {
    input.keys().map(|child| depth(child, input)).sum()
}

fn path_to_com(node: &String, tree: &Tree) -> HashSet<String> {
    let mut node = node;
    let mut path = HashSet::new();

    while let Some(parent) = tree.get(node) {
        path.insert(parent.clone());
        node = parent;
    }

    path
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Tree) -> i32 {
    let your_path = path_to_com(&String::from("YOU"), input);
    let santas_path = path_to_com(&String::from("SAN"), input);
    your_path.symmetric_difference(&santas_path).count() as i32
}
