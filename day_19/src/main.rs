use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
struct Data {
    available_patterns: Vec<Vec<Color>>,

    requested_designs: Vec<Vec<Color>>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let (patterns, designs) = input.split_once("\n\n").expect("Coudl not find empty line");

        let mut available_patterns: Vec<Vec<Color>> = patterns
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.chars().map(Color).collect())
            .collect();

        available_patterns.sort_by_key(|a| a.len());

        let requested_designs = designs
            .lines()
            .map(|line| line.chars().map(Color).collect())
            .collect();

        Self {
            available_patterns,
            requested_designs,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Color(char);

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
    let count = data
        .requested_designs
        .iter()
        .filter(|design| greedy(&data.available_patterns, design).is_some())
        .count();

    dbg!(count);
}

fn part_two(data: &Data) {
    let mut map = HashMap::new();

    let count: u64 = data
        .requested_designs
        .iter()
        .map(|design| greedy_all(&data.available_patterns, design, &mut map))
        .sum();

    dbg!(count);
}

fn greedy(available_patterns: &[Vec<Color>], goal: &[Color]) -> Option<Vec<usize>> {
    if goal.is_empty() {
        return Some(vec![]);
    }

    for (i, pattern) in available_patterns.iter().enumerate() {
        if let Some(rest) = goal.strip_prefix(pattern.as_slice()) {
            if let Some(mut ret) = greedy(available_patterns, rest) {
                ret.push(i);
                return Some(ret);
            }
        }
    }

    None
}

fn greedy_all(
    available_patterns: &[Vec<Color>],
    goal: &[Color],
    lookup: &mut HashMap<Vec<Color>, u64>,
) -> u64 {
    if goal.is_empty() {
        return 1;
    }
    if let Some(val) = lookup.get(goal) {
        return *val;
    }

    let mut cum = 0;

    for pattern in available_patterns.iter() {
        if let Some(rest) = goal.strip_prefix(pattern.as_slice()) {
            let ret = greedy_all(available_patterns, rest, lookup);
            cum += ret;
        }
    }

    lookup.insert(goal.to_vec(), cum);
    cum
}
