use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let mut first: Vec<u64> = vec![];
    let mut second: Vec<u64> = vec![];
    for line in input.split('\n') {
        let mut split = line.split_whitespace();

        let first_str = split.next().expect("Line empty?");
        first.push(str::parse(first_str).expect("Not a number"));

        let second_str = split.next().expect("Line empty?");
        second.push(str::parse(second_str).expect("Not a number"));
    }

    first.sort();
    second.sort();

    part_two(&first, &second);
}

fn part_one(first: &[u64], second: &[u64]) {
    let distance: u64 = first
        .iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(*second))
        .sum();

    println!("{distance}");
}

fn part_two(first: &[u64], second: &[u64]) {
    let score: usize = first
        .iter()
        .map(|value| *value as usize * second.iter().filter(|second| *second == value).count())
        .sum();

    println!("{score}");
}
