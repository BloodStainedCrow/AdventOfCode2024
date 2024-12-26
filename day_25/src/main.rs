use std::{fs::File, io::Read, time::Instant};

struct Data {
    keys: Vec<[[bool; 5]; 5]>,
    locks: Vec<[[bool; 5]; 5]>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];

        for thing in input.split("\n\n") {
            let mut lines = thing.lines();

            let first_line = lines.next();

            let content: Vec<[bool; 5]> = lines
                .take(5)
                .map(|line| {
                    line.chars()
                        .map(|c| c == '#')
                        .collect::<Vec<bool>>()
                        .try_into()
                        .expect("Wrong size")
                })
                .collect();

            if first_line == Some("#####") {
                // This is a lock
                locks.push(content.try_into().expect("Wrong size"));
            } else if first_line == Some(".....") {
                // This is a key
                keys.push(content.try_into().expect("Wrong size"));
            } else {
                unreachable!()
            }
        }

        Self { keys, locks }
    }
}

struct BitMapData {
    keys: Vec<u32>,
    locks: Vec<u32>,
}

impl From<Data> for BitMapData {
    fn from(value: Data) -> Self {
        let keys = value
            .keys
            .iter()
            .map(|key| {
                key.iter().fold(0, |acc, line| {
                    line.iter()
                        .fold(acc, |acc, bit| (acc << 1) | u32::from(*bit))
                })
            })
            .collect();

        let locks = value
            .locks
            .iter()
            .map(|key| {
                key.iter().fold(0, |acc, line| {
                    line.iter()
                        .fold(acc, |acc, bit| (acc << 1) | u32::from(*bit))
                })
            })
            .collect();

        Self { keys, locks }
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let data = Data::from_str(&input);
    let bitmap_data = &data.into();

    let start = Instant::now();

    part_one(bitmap_data);

    dbg!(start.elapsed());
}

fn part_one(data: &BitMapData) {
    let mut count = 0;

    for key in &data.keys {
        for lock in &data.locks {
            if key & lock == 0 {
                // They do not intersect
                count += 1;
            }
        }
    }

    dbg!(count);
}
