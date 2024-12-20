use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::Read,
    iter,
};

use pathfinding::matrix::directions::{E, N};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
struct Data {
    tiles: Vec<Vec<Tile>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Track,
    Wall,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Track,
                        'S' => {
                            start = Some((x, y));
                            Tile::Track
                        }
                        'E' => {
                            end = Some((x, y));
                            Tile::Track
                        }
                        '#' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles,
            start_pos: start.unwrap(),
            end_pos: end.unwrap(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y)?.get(x)
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
    let (base_path, _) = pathfinding::directed::astar::astar(
        &data.start_pos,
        |(x, y)| {
            let x = *x;
            let y = *y;
            Dir::cardinals()
                .map(|dir| dir.into_offsets())
                .map(move |(x_offs, y_offs)| {
                    (x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs))
                })
                .filter(|(x, y)| matches!(data.get(*x, *y), Some(Tile::Track)))
                .map(|pos| (pos, 1usize))
        },
        |(x, y)| {
            x.abs_diff(data.start_pos.0) * x.abs_diff(data.start_pos.0)
                + y.abs_diff(data.start_pos.1) * y.abs_diff(data.start_pos.1)
        },
        |pos| *pos == data.end_pos,
    )
    .expect("Bad input");

    const MIN_SAVED_TIME: usize = 100;

    let count: usize = base_path
        .iter()
        .enumerate()
        .map(|(idx, start_pos)| {
            base_path[idx..]
                .iter()
                .skip(MIN_SAVED_TIME)
                .enumerate()
                .filter_map(|(num_skipped, cheat_end_pos)| {
                    is_reachable_in_exactly_n_steps(*start_pos, 2, *cheat_end_pos)
                        .map(|v| (v, num_skipped))
                })
                .filter(|(num_cheat_steps, num_normal_steps_skipped)| {
                    num_cheat_steps <= num_normal_steps_skipped
                })
                .count()
        })
        .sum();

    println!("{count} cheats save 100 picoseconds");
}

fn part_two(data: &Data) {
    let (base_path, _) = pathfinding::directed::astar::astar(
        &data.start_pos,
        |(x, y)| {
            let x = *x;
            let y = *y;
            Dir::cardinals()
                .map(|dir| dir.into_offsets())
                .map(move |(x_offs, y_offs)| {
                    (x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs))
                })
                .filter(|(x, y)| matches!(data.get(*x, *y), Some(Tile::Track)))
                .map(|pos| (pos, 1usize))
        },
        |(x, y)| {
            x.abs_diff(data.start_pos.0) * x.abs_diff(data.start_pos.0)
                + y.abs_diff(data.start_pos.1) * y.abs_diff(data.start_pos.1)
        },
        |pos| *pos == data.end_pos,
    )
    .expect("Bad input");

    const MIN_SAVED_TIME: usize = 100;

    let count: usize = base_path
        .iter()
        .enumerate()
        .map(|(idx, start_pos)| {
            base_path[idx..]
                .iter()
                .skip(MIN_SAVED_TIME)
                .enumerate()
                .filter_map(|(num_skipped, cheat_end_pos)| {
                    is_reachable_in_max_n_steps(*start_pos, 20, *cheat_end_pos)
                        .map(|v| (v, num_skipped))
                })
                .filter(|(num_cheat_steps, num_normal_steps_skipped)| {
                    num_cheat_steps <= num_normal_steps_skipped
                })
                .count()
        })
        .sum();

    println!("{count} cheats save 100 picoseconds");
}

fn is_reachable_in_exactly_n_steps(
    start: (usize, usize),
    n: usize,
    end: (usize, usize),
) -> Option<usize> {
    if start.0.abs_diff(end.0) + start.1.abs_diff(end.1) == n {
        Some(n)
    } else {
        None
    }
}

fn is_reachable_in_max_n_steps(
    start: (usize, usize),
    n: usize,
    end: (usize, usize),
) -> Option<usize> {
    if start.0.abs_diff(end.0) + start.1.abs_diff(end.1) <= n {
        Some(start.0.abs_diff(end.0) + start.1.abs_diff(end.1))
    } else {
        None
    }
}
