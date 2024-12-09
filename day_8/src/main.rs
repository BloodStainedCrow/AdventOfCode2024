use std::{collections::HashSet, fs::File, io::Read};

#[derive(Debug)]
struct Data {
    map: Vec<Vec<Tile>>,
    antennas: Vec<(usize, usize)>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut antennas = vec![];

        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        c => {
                            antennas.push((x, y));
                            Tile::Antenna(c)
                        }
                    })
                    .collect()
            })
            .collect();

        Self { map, antennas }
    }

    fn get(&self, x: isize, y: isize) -> Option<&Tile> {
        self.map
            .get(usize::try_from(y).ok()?)?
            .get(usize::try_from(x).ok()?)
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Antenna(char),
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
    let mut antinodes = HashSet::new();

    for start_antenna in &data.antennas {
        for end_antenna in &data.antennas {
            if start_antenna == end_antenna {
                continue;
            }

            if data.get(
                start_antenna.0.try_into().unwrap(),
                start_antenna.1.try_into().unwrap(),
            ) != data.get(
                end_antenna.0.try_into().unwrap(),
                end_antenna.1.try_into().unwrap(),
            ) {
                dbg!((
                    data.get(
                        start_antenna.0.try_into().unwrap(),
                        start_antenna.0.try_into().unwrap(),
                    ),
                    data.get(
                        end_antenna.0.try_into().unwrap(),
                        end_antenna.1.try_into().unwrap(),
                    )
                ));
                continue;
            }

            let final_pos = (
                end_antenna.0 as isize + end_antenna.0 as isize - start_antenna.0 as isize,
                end_antenna.1 as isize + end_antenna.1 as isize - start_antenna.1 as isize,
            );

            dbg!(final_pos);

            if data.get(final_pos.0, final_pos.1).is_some() {
                antinodes.insert(final_pos);
            }
        }
    }

    print_map(data, |x, y| {
        antinodes
            .get(&(x.try_into().unwrap(), y.try_into().unwrap()))
            .map(|_| '#')
    });

    println!("Found {} antinodes", antinodes.len());
}

fn print_map(data: &Data, override_fn: impl Fn(usize, usize) -> Option<char>) {
    let width = data.map[0].len();
    let height = data.map.len();

    for y in 0..height {
        for x in 0..width {
            if let Some(c) = override_fn(x, y) {
                print!("{}", c);
            } else {
                match data
                    .get(x.try_into().unwrap(), y.try_into().unwrap())
                    .expect("Should be in bounds")
                {
                    Tile::Empty => print!("."),
                    Tile::Antenna(c) => print!("{}", c),
                }
            }
        }

        println!()
    }
}

fn part_two(data: &Data) {
    let mut antinodes = HashSet::new();

    for start_antenna in &data.antennas {
        for end_antenna in &data.antennas {
            if start_antenna == end_antenna {
                continue;
            }

            if data.get(
                start_antenna.0.try_into().unwrap(),
                start_antenna.1.try_into().unwrap(),
            ) != data.get(
                end_antenna.0.try_into().unwrap(),
                end_antenna.1.try_into().unwrap(),
            ) {
                // the nodes do not have the same frequency
                continue;
            }

            let offs = (
                end_antenna.0 as isize - start_antenna.0 as isize,
                end_antenna.1 as isize - start_antenna.1 as isize,
            );

            let mut pos = (end_antenna.0 as isize, end_antenna.1 as isize);

            while data.get(pos.0, pos.1).is_some() {
                antinodes.insert(pos);

                pos.0 += offs.0;
                pos.1 += offs.1;
            }
        }
    }

    print_map(data, |x, y| {
        antinodes
            .get(&(x.try_into().unwrap(), y.try_into().unwrap()))
            .map(|_| '#')
    });

    println!("Found {} antinodes", antinodes.len());
}
