use std::{collections::HashMap, fs::File, io::Read, iter, time::Instant};

use rayon::iter::{ParallelDrainRange, ParallelIterator};

struct Data {
    stones: Vec<u64>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let stones = input
            .lines()
            .next()
            .expect("Empty input")
            .split_ascii_whitespace()
            .map(|s| str::parse(s).expect("Could not parse number"))
            .collect();

        Self { stones }
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let from_str = Data::from_str(&input);
    let mut data = from_str;

    let time = Instant::now();

    part_two(&mut data);

    println!("{:?}", time.elapsed());
}

fn part_one(data: &mut Data) {
    blink_dynamic(data, 25);
}

fn part_two(data: &mut Data) {
    blink_dynamic(data, 75);
}

fn blink_raw(data: &mut Data, n: usize) {
    // This is too slow (and too memory hungry) to calculate any n over ~45, but is was great to validate my dynamic solution
    for _ in (0..n).map(|n| dbg!(n)) {
        // Blink n times

        data.stones = data
            .stones
            .par_drain(..)
            .flat_map_iter(|stone| {
                let len = stone.checked_ilog10().unwrap_or(0) + 1;
                let vals: Box<dyn Iterator<Item = u64>> = if stone == 0 {
                    Box::new(iter::once(1))
                } else if len % 2 == 0 {
                    Box::new(
                        iter::once(stone / 10_u64.pow(len / 2))
                            .chain(iter::once(stone % 10_u64.pow(len / 2))),
                    )
                } else {
                    Box::new(iter::once(stone * 2024))
                };

                vals
            })
            .collect();
    }

    println!("Found {} stones", data.stones.len())
}

fn blink_dynamic(data: &Data, n: usize) {
    // This is slower when using par_iter and RwLock then just doing it single_threaded *shrug*
    let mut lookup = HashMap::new();

    let sum: u64 = data
        .stones
        .iter()
        .map(|stone| count_stones_for_for(*stone, n, &mut lookup))
        .sum();

    println!("Found {sum} stones");
}

fn count_stones_for_for(stone: u64, n: usize, lookup: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(v) = lookup.get(&(stone, n)) {
        return *v;
    }

    if n == 0 {
        lookup.insert((stone, n), 1);
        return 1;
    }

    let len = stone.checked_ilog10().unwrap_or(0) + 1;
    if stone == 0 {
        let res = count_stones_for_for(1, n - 1, lookup);
        lookup.insert((stone, n), res);
        res
    } else if len % 2 == 0 {
        let res = count_stones_for_for(stone / 10_u64.pow(len / 2), n - 1, lookup)
            + count_stones_for_for(stone % 10_u64.pow(len / 2), n - 1, lookup);
        lookup.insert((stone, n), res);
        res
    } else {
        let res = count_stones_for_for(stone * 2024, n - 1, lookup);
        lookup.insert((stone, n), res);
        res
    }
}
