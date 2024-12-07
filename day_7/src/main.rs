use std::{
    cmp::{max, min},
    fs::File,
    io::Read,
};

#[derive(Debug)]
struct Data {
    entries: Vec<Entry>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        Self {
            entries: input.lines().map(Entry::from_str).collect(),
        }
    }
}

#[derive(Debug)]
struct Entry {
    goal: u64,
    values: Vec<u64>,
}

impl Entry {
    fn from_str(input: &str) -> Self {
        let (goal, rest) = input.split_once(':').expect("Input malformed (missing :)");

        Self {
            goal: str::parse(goal).expect("could not parse goal"),
            values: rest
                .split_whitespace()
                .map(|v| str::parse(v).expect("Could not parse value"))
                .collect(),
        }
    }
}

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
    let mut sum = 0;

    for entry in &data.entries {
        if is_possible_mul_add(entry.goal, &entry.values) {
            sum += entry.goal;
        }
    }

    dbg!(sum);
}

fn part_two(data: &Data) {
    let mut sum = 0;

    for entry in &data.entries {
        if is_possible_mul_add_concat(entry.goal, &entry.values) {
            sum += entry.goal;
        }
    }

    dbg!(sum);
}

fn is_possible_mul_add(goal: u64, values: &[u64]) -> bool {
    match min_possible_add_mul(values).cmp(&goal) {
        std::cmp::Ordering::Less => {}
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Greater => return false,
    }

    match max_possible_add_mul(values).cmp(&goal) {
        std::cmp::Ordering::Less => return false,
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Greater => {}
    }

    let last_value = values.last().expect("values should not be empty");
    if goal % last_value == 0 {
        // We check division first, since it will reduce the size of ther value faster
        is_possible_mul_add(goal / last_value, &values[0..(values.len() - 1)])
            | is_possible_mul_add(goal - last_value, &values[0..(values.len() - 1)])
    } else {
        is_possible_mul_add(goal - last_value, &values[0..(values.len() - 1)])
    }
}

fn is_possible_mul_add_concat(goal: u64, values: &[u64]) -> bool {
    match min_possible_add_mul_concat(values).cmp(&goal) {
        std::cmp::Ordering::Less => {}
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Greater => return false,
    }

    match max_possible_add_mul_concat(values).cmp(&goal) {
        std::cmp::Ordering::Less => return false,
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Greater => {}
    }

    let last_value = values.last().expect("values should not be empty");
    if let Some(new_goal) = deconcat(goal, *last_value) {
        if goal % last_value == 0 {
            // We check concat first, since it will reduce the size of ther value faster, then division
            is_possible_mul_add_concat(new_goal, &values[0..(values.len() - 1)])
                | is_possible_mul_add_concat(goal / last_value, &values[0..(values.len() - 1)])
                | is_possible_mul_add_concat(goal - last_value, &values[0..(values.len() - 1)])
        } else {
            is_possible_mul_add_concat(new_goal, &values[0..(values.len() - 1)])
                | is_possible_mul_add_concat(goal - last_value, &values[0..(values.len() - 1)])
        }
    } else if goal % last_value == 0 {
        // We check concat first, since it will reduce the size of ther value faster, then division
        is_possible_mul_add_concat(goal / last_value, &values[0..(values.len() - 1)])
            | is_possible_mul_add_concat(goal - last_value, &values[0..(values.len() - 1)])
    } else {
        is_possible_mul_add_concat(goal - last_value, &values[0..(values.len() - 1)])
    }
}

fn min_possible_add_mul(values: &[u64]) -> u64 {
    values[1..]
        .iter()
        .fold(values[0], |acc, v| min(acc * v, acc + v))
}

fn max_possible_add_mul(values: &[u64]) -> u64 {
    values[1..]
        .iter()
        .fold(values[0], |acc, v| max(acc * v, acc + v))
}

fn min_possible_add_mul_concat(values: &[u64]) -> u64 {
    values[1..].iter().fold(values[0], |acc, v| {
        min(concat(acc, *v), min(acc * v, acc + v))
    })
}

fn max_possible_add_mul_concat(values: &[u64]) -> u64 {
    values[1..].iter().fold(values[0], |acc, v| {
        max(concat(acc, *v), max(acc * v, acc + v))
    })
}

fn concat(a: u64, b: u64) -> u64 {
    let b_len = b.checked_ilog10().unwrap_or(0) + 1;

    a * 10_u64.pow(b_len) + b
}

fn deconcat(a: u64, b: u64) -> Option<u64> {
    let b_len = b.checked_ilog10().unwrap_or(0) + 1;

    if a % 10_u64.pow(b_len) == b {
        // a ends with b
        Some(a / 10_u64.pow(b_len))
    } else {
        None
    }
}
