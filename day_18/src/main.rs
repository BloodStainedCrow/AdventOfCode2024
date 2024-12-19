use std::{fs::File, io::Read};

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

#[derive(Debug)]
struct Data {
    bytes: Vec<(usize, usize)>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let bytes = input
            .lines()
            .map(|line| line.split_once(',').expect("Could not find comma"))
            .map(|(x, y)| {
                (
                    x.parse().expect("Could not parse"),
                    y.parse().expect("Could not parse"),
                )
            })
            .collect();

        Self { bytes }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    size: usize,
}

impl Map {
    fn from_data(data: &Data, size: usize) -> Self {
        let mut tiles = vec![vec![Tile::PermanentlyFree; size]; size];

        for (time, (x, y)) in data.bytes.iter().enumerate() {
            if tiles[*y][*x] == Tile::PermanentlyFree {
                // The first byte (idx 0) falls after 1 ns
                tiles[*y][*x] = Tile::CorruptedAt(time + 1);
            }
        }

        Self { tiles, size }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y)?.get(x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    PermanentlyFree,
    CorruptedAt(usize),
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
    let map = Map::from_data(data, 71);

    const TIME_PER_MOVE: usize = 0;

    let res = pathfinding::directed::astar::astar(
        &((0usize, 0usize), 1024),
        |((x, y), time)| {
            let x = *x;
            let y = *y;
            let time = *time;

            Dir::cardinals()
                .map(|dir| dir.into_offsets())
                .map(move |(x_offs, y_offs)| {
                    (
                        (x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs)),
                        time + TIME_PER_MOVE,
                    )
                })
                .filter(|((x, y), time)| match map.get(*x, *y) {
                    Some(Tile::PermanentlyFree) => true,
                    Some(Tile::CorruptedAt(time_corrupted)) => time < time_corrupted,
                    None => false,
                })
                .map(|((x, y), time)| (((x, y), time), 1))
        },
        |((x, y), _time)| x * x + y * y,
        |((x, y), _time)| *x == map.size - 1 && *y == map.size - 1,
    );

    let path = res.unwrap().0;

    let num_tiles = path.len();

    let num_steps = num_tiles - 1;

    print_map(&map, 1024, |x, y| {
        path.iter()
            .any(|((x_pos, y_pos), time)| x == *x_pos && y == *y_pos)
    });

    dbg!(num_steps);
}

fn part_two(data: &Data) {
    let map = Map::from_data(data, 71);

    const TIME_PER_MOVE: usize = 0;

    let mut current_time = 1024;

    loop {
        let res = pathfinding::directed::astar::astar(
            &((0usize, 0usize), current_time),
            |((x, y), time)| {
                let x = *x;
                let y = *y;
                let time = *time;

                Dir::cardinals()
                    .map(|dir| dir.into_offsets())
                    .map(move |(x_offs, y_offs)| {
                        (
                            (x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs)),
                            time + TIME_PER_MOVE,
                        )
                    })
                    .filter(|((x, y), time)| match map.get(*x, *y) {
                        Some(Tile::PermanentlyFree) => true,
                        Some(Tile::CorruptedAt(time_corrupted)) => time < time_corrupted,
                        None => false,
                    })
                    .map(|((x, y), time)| (((x, y), time), 1))
            },
            |((x, y), _time)| x * x + y * y,
            |((x, y), _time)| *x == map.size - 1 && *y == map.size - 1,
        );

        dbg!(current_time);

        match res {
            Some(_) => current_time += 1,
            None => break,
        }
    }

    dbg!(data.bytes[current_time - 1]);
}

fn print_map(map: &Map, time_step: usize, step_fn: impl Fn(usize, usize) -> bool) {
    for (y, line) in map.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::PermanentlyFree => {
                    if step_fn(x, y) {
                        print!("O");
                        continue;
                    } else {
                        print!(".");
                    }
                }
                Tile::CorruptedAt(time_corrupted) if time_corrupted <= &time_step => {
                    print!("#");
                    assert!(!step_fn(x, y));
                }
                Tile::CorruptedAt(time) => {
                    if step_fn(x, y) {
                        print!("O");
                        continue;
                    } else {
                        print!(".");
                    }
                }
            }
        }

        println!()
    }
}
