use std::{collections::HashSet, fs::File, io::Read};

use colored::Colorize;
use log::info;
use simple_logger::SimpleLogger;
use strum_macros::EnumIter;

use strum::IntoEnumIterator;

struct Data {
    map: Vec<Vec<u32>>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("char is not a digit"))
                    .collect()
            })
            .collect();

        Self { map }
    }

    fn get(&self, x: isize, y: isize) -> Option<&u32> {
        self.map
            .get(usize::try_from(y).ok()?)?
            .get(usize::try_from(x).ok()?)
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let from_str = Data::from_str(&input);
    let data = from_str;

    part_two(&data);
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    UL,
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
}

impl Dir {
    fn into_offsets(self) -> (isize, isize) {
        let (y, x) = match self {
            Dir::UL => (-1, -1),
            Dir::U => (-1, 0),
            Dir::UR => (-1, 1),
            Dir::R => (0, 1),
            Dir::DR => (1, 1),
            Dir::D => (1, 0),
            Dir::DL => (1, -1),
            Dir::L => (0, -1),
        };

        (x, y)
    }

    fn get_opposite(self) -> Self {
        match self {
            Dir::UL => Dir::DR,
            Dir::U => Dir::D,
            Dir::UR => Dir::DL,
            Dir::R => Dir::L,
            Dir::DR => Dir::UL,
            Dir::D => Dir::U,
            Dir::DL => Dir::UR,
            Dir::L => Dir::R,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::UL => Dir::UR,
            Dir::U => Dir::R,
            Dir::UR => Dir::DR,
            Dir::R => Dir::D,
            Dir::DR => Dir::DL,
            Dir::D => Dir::L,
            Dir::DL => Dir::UL,
            Dir::L => Dir::U,
        }
    }

    fn turn_left(self) -> Self {
        self.turn_right().get_opposite()
    }

    fn diags() -> impl Iterator<Item = Self> {
        Self::iter().filter(|dir| dir.into_offsets().0.abs() + dir.into_offsets().1.abs() == 2)
    }

    fn cardinals() -> impl Iterator<Item = Self> {
        Self::iter().filter(|dir| dir.into_offsets().0.abs() + dir.into_offsets().1.abs() == 1)
    }
}

fn part_one(data: &Data) {
    let mut sum = 0;
    for y in 0..data.map.len() {
        for x in 0..data.map[y].len() {
            if Some(&0) == data.get(x as isize, y as isize) {
                let count = count_hilltops(data, (x as isize, y as isize));
                sum += count;
                let count = format!("{count}");
                print!("{}", count.red());
            } else {
                print!(
                    "{}",
                    data.get(x as isize, y as isize)
                        .expect("Should be in bounds")
                );
            }
        }
        println!()
    }

    println!();
    println!("Scores of all trailheads: {sum}");
}

fn part_two(data: &Data) {
    let mut sum = 0;
    for y in 0..data.map.len() {
        for x in 0..data.map[y].len() {
            if Some(&0) == data.get(x as isize, y as isize) {
                let count = count_paths(data, (x as isize, y as isize));
                sum += count;
                let count = format!("{count:x}");
                print!("{}", count.red());
            } else {
                print!(
                    "{}",
                    data.get(x as isize, y as isize)
                        .expect("Should be in bounds")
                );
            }
        }
        println!()
    }

    println!();
    println!("Scores of all trailheads: {sum}");
}

fn count_hilltops(data: &Data, pos: (isize, isize)) -> usize {
    let current_height = data.get(pos.0, pos.1).expect("Current pos out of bounds");

    let mut positions = HashSet::new();
    positions.insert(pos);

    for next_height in (current_height + 1)..=9 {
        positions = HashSet::from_iter(positions.iter().flat_map(|pos| {
            Dir::cardinals()
                .map(|dir| dir.into_offsets())
                .map(|(x_offs, y_offs)| (pos.0 + x_offs, pos.1 + y_offs))
                .filter(|(x, y)| data.get(*x, *y) == Some(&next_height))
        }));
    }

    positions.len()
}

fn count_paths(data: &Data, current_pos: (isize, isize)) -> usize {
    info!("{:?}", current_pos);
    let current_height = data
        .get(current_pos.0, current_pos.1)
        .expect("Current pos out of bounds");

    if *current_height == 9 {
        info!("{}", current_height);
        return 1;
    }

    let number_paths = Dir::cardinals()
        .map(|dir| dir.into_offsets())
        .map(|(x_offs, y_offs)| (current_pos.0 + x_offs, current_pos.1 + y_offs))
        .filter(|(x, y)| data.get(*x, *y) == Some(&(current_height + 1)))
        .map(|(x, y)| count_paths(data, (x, y)))
        .sum();

    number_paths
}
