use std::{cmp::max, fs::File, io::Read};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

type RNG = i64;

struct Data {
    seeds: Vec<RNG>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let seeds = input
            .lines()
            .map(|line| line.parse().expect("Could not parse value"))
            .collect();

        Self { seeds }
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

    for seed in &data.seeds {
        let mut seed = *seed;
        for i in 0..2000 {
            rng(&mut seed);
        }

        sum += seed;
    }

    dbg!(sum);
}

fn part_two(data: &Data) {
    let mut sequences = vec![vec![]; data.seeds.len()];

    for (seed, sequence) in data.seeds.iter().zip(sequences.iter_mut()) {
        let mut seed = *seed;
        sequence.push(seed);
        for i in 0..2000 {
            rng(&mut seed);
            sequence.push(seed);
        }
    }

    let sequences_of_diffs: Vec<Vec<i64>> = sequences
        .iter()
        .map(|sequence| {
            sequence
                .iter()
                .zip(sequence.iter().skip(1))
                .map(|(first, second)| (first % 10, second % 10))
                .map(|(first, second)| second - first)
                .collect()
        })
        .collect();

    let max_val: Option<i64> = (-9..=9)
        .into_par_iter()
        .map(|a| {
            (-9..=9)
                .map(|b| {
                    (-9..=9)
                        .map(|c| {
                            (-9..=9)
                                .map(|d| {
                                    sequences_of_diffs
                                        .iter()
                                        .enumerate()
                                        .map(|(sequence_idx, diffs)| {
                                            let sequence = &sequences[sequence_idx];
                                            diffs
                        .iter()
                        .zip(
                            diffs
                                .iter()
                                .skip(1)
                                .zip(diffs.iter().skip(2).zip(diffs.iter().enumerate().skip(3))),
                        )
                        .filter_map(|(first, (second, (third, (sell_idx, fourth))))| {
                            if *first == a && b == *second && c == *third && d == *fourth {
                                Some(sequence[sell_idx + 1] % 10)
                            } else {
                                None
                            }
                        })
                        .next()
                        .unwrap_or(0)
                                        })
                                        .sum()
                                })
                                .max()
                                .unwrap_or(0)
                        })
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0)
        })
        .max();

    dbg!(max_val);
}

fn rng(seed: &mut RNG) {
    let mix_val = *seed * 64;

    mix(seed, mix_val);

    prune(seed);

    let mix_val = *seed / 32;

    mix(seed, mix_val);

    prune(seed);

    let mix_val = *seed * 2048;

    mix(seed, mix_val);

    prune(seed);
}

fn mix(this: &mut RNG, other: RNG) {
    *this ^= other;
}

fn prune(this: &mut RNG) {
    *this %= 16777216;
}
