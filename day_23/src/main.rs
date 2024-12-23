use std::{fs::File, io::Read};

use itertools::Itertools;
use petgraph::{algo::maximal_cliques, prelude::GraphMap, Undirected};

struct Data {
    network: GraphMap<Computer, (), Undirected>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut network = GraphMap::new();

        for line in input.lines() {
            let (a, b) = line.split_once('-').expect("Could not parse connection");

            assert_eq!(a.len(), 2);
            assert_eq!(b.len(), 2);

            network.add_edge(
                (a.chars().next().unwrap(), a.chars().last().unwrap()),
                (b.chars().next().unwrap(), b.chars().last().unwrap()),
                (),
            );
        }

        Self { network }
    }
}

type Computer = (char, char);

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let data = Data::from_str(&input);

    part_two(&data);
}

fn part_one(data: &Data) {
    let mut sets: Vec<Vec<Computer>> = data
        .network
        .nodes()
        .flat_map(|node| traverse_recursive(data, node, 3, |end| end == node).into_iter())
        .collect();

    // Deduplicate
    sets.retain(|set| set.is_sorted());

    dbg!(sets
        .iter()
        .filter(|set| set.iter().any(|comp| comp.0 == 't'))
        .count());
}

fn part_two(data: &Data) {
    let maximal = maximal_cliques(&data.network);

    let mut participants: Vec<Computer> = maximal
        .into_iter()
        .max_by_key(|a| a.len())
        .unwrap()
        .into_iter()
        .collect();

    participants.sort();

    println!(
        "{}",
        participants
            .iter()
            .map(|(a, b)| format!("{}{}", a, b))
            .join(",")
    )
}

fn traverse_recursive(
    data: &Data,
    start_node: Computer,
    max_depth: usize,
    accept: impl Fn(Computer) -> bool + Clone,
) -> Vec<Vec<Computer>> {
    if max_depth == 0 {
        if accept(start_node) {
            return vec![vec![]];
        } else {
            return vec![];
        }
    }

    let ret = data
        .network
        .neighbors(start_node)
        .flat_map(|neighbor| {
            traverse_recursive(data, neighbor, max_depth - 1, accept.clone())
                .into_iter()
                .map(|mut v| {
                    v.push(start_node);
                    v
                })
        })
        .collect();

    ret
}
