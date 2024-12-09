use std::{fs::File, io::Read, iter::repeat_n};

#[derive(Debug)]
struct Data {
    disk: Vec<Sector>,
}

#[derive(Debug, Clone, PartialEq)]
enum Sector {
    Empty,
    Full(usize),
}

impl Data {
    fn from_str(input: &str) -> Self {
        let disk = input
            .lines()
            .next()
            .expect("No line")
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let len = c
                    .to_digit(10)
                    .expect("Could not parse digit")
                    .try_into()
                    .unwrap();
                if i % 2 == 0 {
                    // This is a file
                    repeat_n(Sector::Full(i / 2), len)
                } else {
                    repeat_n(Sector::Empty, len)
                }
            })
            .collect();

        Self { disk }
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

    part_two(&mut data);
}

fn part_two(data: &mut Data) {
    for right_idx in (0..data.disk.len()).rev() {
        if let Some(Sector::Full(_)) = data.disk.get(right_idx) {
        } else {
            // This is not a file
            continue;
        }

        if data.disk.get(right_idx - 1).is_some()
            && data.disk.get(right_idx - 1) == data.disk.get(right_idx)
        {
            // We are not at the leftmost position for this file
            continue;
        }

        let file_len = data.disk[right_idx..]
            .iter()
            .take_while(|s| Some(s) == data.disk.get(right_idx).as_ref())
            .count();

        assert!(file_len > 0);

        for left_idx in 0..data.disk.len() {
            if left_idx >= right_idx {
                break;
            }

            // Check if there is enough space for the file
            if data.disk[left_idx..]
                .iter()
                .take(file_len)
                .all(|s| *s == Sector::Empty)
            {
                // Copy the file over
                for offs in 0..file_len {
                    data.disk.swap(left_idx + offs, right_idx + offs);
                }

                println!("changed {left_idx} {right_idx}");
                break;
            }
        }
    }

    // Calc checksum
    let checksum = get_checksum(data);

    println!("Checksum is {checksum}")
}

fn part_one(data: &mut Data) {
    // Start at the end
    let mut left_idx = 0;
    let mut right_idx = data.disk.len() - 1;

    loop {
        while let Sector::Full(_) = data.disk[left_idx] {
            left_idx += 1;
        }

        while Sector::Empty == data.disk[right_idx] {
            right_idx -= 1;
        }

        if left_idx > right_idx {
            break;
        }

        data.disk.swap(left_idx, right_idx);
    }

    // Calc checksum
    let checksum = get_checksum(data);

    println!("Checksum is {checksum}")
}

fn get_checksum(data: &Data) -> usize {
    let mut checksum = 0;

    for (block_idx, block) in data.disk.iter().enumerate() {
        if let Sector::Full(file_id) = block {
            checksum += block_idx * file_id;
        }
    }

    checksum
}
