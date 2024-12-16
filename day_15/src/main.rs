use std::{borrow::Cow, collections::VecDeque, fs::File, io::Read};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug)]
struct Data {
    map: Vec<Vec<Tile>>,
    robot_pos: (usize, usize),
    instructions: Vec<Dir>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let (map, instructions) = input.split_once("\n\n").expect("Could not find empty line");

        let mut player_pos = None;

        let map = map
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'O' => Tile::Box,
                        '@' => {
                            assert!(player_pos.is_none(), "Found multiple players!");
                            player_pos = Some((x, y));
                            Tile::Empty
                        }
                        '[' => Tile::LeftBoxHalf,
                        ']' => Tile::RightBoxHalf,
                        c => panic!("Unknown char {c}"),
                    })
                    .collect()
            })
            .collect();

        // TODO: Verify there are not half boxes!

        let instructions = instructions
            .chars()
            .filter_map(|c| match c {
                '\n' => None,
                '^' => Some(Dir::U),
                '>' => Some(Dir::R),
                'v' => Some(Dir::D),
                '<' => Some(Dir::L),
                c => panic!("Unknown char {c}"),
            })
            .collect();

        Self {
            map,
            robot_pos: player_pos.expect("Did not find robot position"),
            instructions,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Box,
    Wall,
    LeftBoxHalf,
    RightBoxHalf,
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    part_two(&input);
}

fn part_one(input: &str) {
    let data = &mut Data::from_str(input);

    move_till_stuck(data);

    print_map(&data.map, data.robot_pos);

    println!("GPS sum is {}", calc_gps(data));
}

fn part_two(input: &str) {
    let input = String::from_iter(input.chars().map(|c| match c {
        '#' => Cow::Borrowed("##"),
        '.' => Cow::Borrowed(".."),
        'O' => Cow::Borrowed("[]"),
        '@' => Cow::Borrowed("@."),
        '[' => unreachable!("Long box in input"),
        ']' => unreachable!("Long box in input"),
        c => Cow::Owned(c.to_string()),
    }));

    let data = &mut Data::from_str(&input);

    move_till_stuck(data);

    print_map(&data.map, data.robot_pos);

    println!("GPS sum is {}", calc_gps(data));
}

fn calc_gps(data: &Data) -> usize {
    let mut gps_sum = 0;

    for y in 0..data.map.len() {
        for x in 0..data.map[y].len() {
            if data.map[y][x] == Tile::Box || data.map[y][x] == Tile::LeftBoxHalf {
                gps_sum += 100 * y + x;
            }
        }
    }

    gps_sum
}

fn print_map(map: &[Vec<Tile>], player_pos: (usize, usize)) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let print = match map[y][x] {
                Tile::Empty => {
                    if (x, y) == player_pos {
                        '@'
                    } else {
                        '.'
                    }
                }
                Tile::Box => 'O',
                Tile::Wall => '#',
                Tile::LeftBoxHalf => '[',
                Tile::RightBoxHalf => ']',
            };

            print!("{}", print);
        }
        println!()
    }
}

fn move_till_stuck(data: &mut Data) {
    for dir in &data.instructions {
        match dir {
            Dir::U | Dir::R | Dir::D | Dir::L => {
                let offs = dir.into_offsets();
                let front_pos = (
                    data.robot_pos.0.checked_add_signed(offs.0).unwrap(),
                    data.robot_pos.1.checked_add_signed(offs.1).unwrap(),
                );

                #[cfg(debug_assertions)]
                print_map(&data.map, data.robot_pos);

                if try_move(&mut data.map, *dir, front_pos, false).is_err() {
                    continue;
                } else {
                    assert_eq!(data.map[front_pos.1][front_pos.0], Tile::Empty);

                    data.robot_pos = front_pos;
                }
            }
            _ => unreachable!("Only cardinal directions allowed"),
        }
    }
}

fn try_move(
    map: &mut Vec<Vec<Tile>>,
    dir: Dir,
    pos: (usize, usize),
    dry_run: bool,
) -> Result<(), ()> {
    match map[pos.1][pos.0] {
        Tile::Empty => Ok(()),
        Tile::Box => {
            let offs = dir.into_offsets();
            let front_pos = (
                pos.0.checked_add_signed(offs.0).unwrap(),
                pos.1.checked_add_signed(offs.1).unwrap(),
            );
            if try_move(map, dir, front_pos, false).is_ok() {
                assert_eq!(map[front_pos.1][front_pos.0], Tile::Empty);
                map[front_pos.1][front_pos.0] = Tile::Box;
                map[pos.1][pos.0] = Tile::Empty;
                Ok(())
            } else {
                Err(())
            }
        }
        Tile::Wall => Err(()),
        Tile::LeftBoxHalf => {
            assert_eq!(map[pos.1][pos.0 + 1], Tile::RightBoxHalf);

            let offs = dir.into_offsets();
            let front_pos = (
                pos.0.checked_add_signed(offs.0).unwrap(),
                pos.1.checked_add_signed(offs.1).unwrap(),
            );
            if dir == Dir::U || dir == Dir::D {
                if try_move(map, dir, front_pos, true).is_ok()
                    && try_move(map, dir, (front_pos.0 + 1, front_pos.1), true).is_ok()
                {
                    if dry_run == false {
                        try_move(map, dir, front_pos, false).expect("We checked before");
                        try_move(map, dir, (front_pos.0 + 1, front_pos.1), false)
                            .expect("We checked before");

                        //assert_eq!(map[front_pos.1][front_pos.0], Tile::Empty);
                        //assert_eq!(map[front_pos.1][front_pos.0 + 1], Tile::Empty);
                        map[front_pos.1][front_pos.0] = Tile::LeftBoxHalf;
                        map[front_pos.1][front_pos.0 + 1] = Tile::RightBoxHalf;
                        map[pos.1][pos.0] = Tile::Empty;
                        map[pos.1][pos.0 + 1] = Tile::Empty;
                    }
                    Ok(())
                } else {
                    Err(())
                }
            } else if try_move(map, dir, front_pos, false).is_ok() {
                assert_eq!(map[front_pos.1][front_pos.0], Tile::Empty);
                map[front_pos.1][front_pos.0] = Tile::LeftBoxHalf;
                map[pos.1][pos.0] = Tile::Empty;
                Ok(())
            } else {
                Err(())
            }
        }
        Tile::RightBoxHalf => {
            assert_eq!(map[pos.1][pos.0 - 1], Tile::LeftBoxHalf);

            let offs = dir.into_offsets();
            let front_pos = (
                pos.0.checked_add_signed(offs.0).unwrap(),
                pos.1.checked_add_signed(offs.1).unwrap(),
            );

            if dir == Dir::U || dir == Dir::D {
                if try_move(map, dir, front_pos, true).is_ok()
                    && try_move(map, dir, (front_pos.0 - 1, front_pos.1), true).is_ok()
                {
                    if dry_run == false {
                        try_move(map, dir, front_pos, false).expect("We checked before");
                        try_move(map, dir, (front_pos.0 - 1, front_pos.1), false)
                            .expect("We checked before");

                        assert_eq!(map[front_pos.1][front_pos.0], Tile::Empty);
                        assert_eq!(map[front_pos.1][front_pos.0 - 1], Tile::Empty);
                        map[front_pos.1][front_pos.0] = Tile::RightBoxHalf;
                        map[front_pos.1][front_pos.0 - 1] = Tile::LeftBoxHalf;
                        map[pos.1][pos.0] = Tile::Empty;
                        map[pos.1][pos.0 - 1] = Tile::Empty;
                    }
                    Ok(())
                } else {
                    Err(())
                }
            } else if try_move(map, dir, front_pos, false).is_ok() {
                assert_eq!(map[front_pos.1][front_pos.0], Tile::Empty);
                map[front_pos.1][front_pos.0] = Tile::RightBoxHalf;
                map[pos.1][pos.0] = Tile::Empty;
                Ok(())
            } else {
                Err(())
            }
        }
    }
}
