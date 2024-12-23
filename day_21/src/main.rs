use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn into_offsets(self) -> (isize, isize) {
        let (y, x) = match self {
            Dir::U => (-1, 0),
            Dir::R => (0, 1),
            Dir::D => (1, 0),
            Dir::L => (0, -1),
        };

        (x, y)
    }

    fn get_opposite(self) -> Self {
        match self {
            Dir::U => Dir::D,
            Dir::R => Dir::L,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::U => Dir::R,
            Dir::R => Dir::D,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
        }
    }

    fn turn_left(self) -> Self {
        self.turn_right().get_opposite()
    }
}

struct Data {
    codes: Vec<(Vec<KeyVal>, usize)>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let codes = input
            .lines()
            .map(|line| {
                (
                    line.chars()
                        .map(|c| match c {
                            c if c.is_numeric() => {
                                KeyVal::Num(c.to_digit(10).unwrap().try_into().unwrap())
                            }
                            'A' => KeyVal::A,
                            _ => unreachable!(),
                        })
                        .collect(),
                    line.split('A')
                        .next()
                        .expect("No seqment found")
                        .parse()
                        .expect("Could not parse into usize"),
                )
            })
            .collect();

        Self { codes }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum KeyVal {
    Num(u8),
    A,
}

impl KeyVal {
    fn into_pos(self) -> (usize, usize) {
        match self {
            KeyVal::Num(0) => (1, 3),
            KeyVal::A => (2, 3),
            KeyVal::Num(1) => (0, 2),
            KeyVal::Num(2) => (1, 2),
            KeyVal::Num(3) => (2, 2),
            KeyVal::Num(4) => (0, 1),
            KeyVal::Num(5) => (1, 1),
            KeyVal::Num(6) => (2, 1),
            KeyVal::Num(7) => (0, 0),
            KeyVal::Num(8) => (1, 0),
            KeyVal::Num(9) => (2, 0),

            KeyVal::Num(_) => unreachable!(),
        }
    }

    fn try_from_pos(x: usize, y: usize) -> Option<Self> {
        match (x, y) {
            (3, 0) => None,
            (1, 3) => Some(KeyVal::Num(0)),
            (0, 2) => Some(KeyVal::Num(1)),
            (1, 2) => Some(KeyVal::Num(2)),
            (2, 2) => Some(KeyVal::Num(3)),
            (0, 1) => Some(KeyVal::Num(4)),
            (1, 1) => Some(KeyVal::Num(5)),
            (2, 1) => Some(KeyVal::Num(6)),
            (0, 0) => Some(KeyVal::Num(7)),
            (1, 0) => Some(KeyVal::Num(8)),
            (2, 0) => Some(KeyVal::Num(9)),
            (2, 3) => Some(KeyVal::A),

            (_, _) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirVal {
    Dir(Dir),
    A,
}

impl DirVal {
    fn try_from_pos(x: usize, y: usize) -> Option<Self> {
        match (x, y) {
            (0, 0) => None,
            (1, 0) => Some(DirVal::Dir(Dir::U)),
            (2, 0) => Some(DirVal::A),
            (0, 1) => Some(DirVal::Dir(Dir::L)),
            (1, 1) => Some(DirVal::Dir(Dir::D)),
            (2, 1) => Some(DirVal::Dir(Dir::R)),

            (_, _) => None,
        }
    }

    fn into_pos(self) -> (usize, usize) {
        match self {
            DirVal::Dir(Dir::U) => (1, 0),
            DirVal::A => (2, 0),
            DirVal::Dir(Dir::L) => (0, 1),
            DirVal::Dir(Dir::D) => (1, 1),
            DirVal::Dir(Dir::R) => (2, 1),
        }
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
    let mut lookup_dirs = HashMap::new();

    for i in 0..=4 {
        let start_key = match i {
            0 => DirVal::A,
            1 => DirVal::Dir(Dir::U),
            2 => DirVal::Dir(Dir::R),
            3 => DirVal::Dir(Dir::D),
            4 => DirVal::Dir(Dir::L),
            _ => unreachable!(),
        };

        for i in 0..=4 {
            let end_key = match i {
                0 => DirVal::A,
                1 => DirVal::Dir(Dir::U),
                2 => DirVal::Dir(Dir::R),
                3 => DirVal::Dir(Dir::D),
                4 => DirVal::Dir(Dir::L),
                _ => unreachable!(),
            };

            lookup_dirs.insert((start_key, end_key, 0), 1);
        }
    }

    for num_robots in 1..=2 {
        optimize_dirpad(num_robots, &mut lookup_dirs);
    }

    let lookup_keys = optimize_keypad(&lookup_dirs, 3);

    let mut sum = 0;

    for (code, prefix) in &data.codes {
        let solution = solve_with_lookups(code, &lookup_keys, 3);

        sum += solution * prefix;
    }

    dbg!(sum);
}

fn part_two(data: &Data) {
    let mut lookup_dirs = HashMap::new();

    for i in 0..=4 {
        let start_key = match i {
            0 => DirVal::A,
            1 => DirVal::Dir(Dir::U),
            2 => DirVal::Dir(Dir::R),
            3 => DirVal::Dir(Dir::D),
            4 => DirVal::Dir(Dir::L),
            _ => unreachable!(),
        };

        for i in 0..=4 {
            let end_key = match i {
                0 => DirVal::A,
                1 => DirVal::Dir(Dir::U),
                2 => DirVal::Dir(Dir::R),
                3 => DirVal::Dir(Dir::D),
                4 => DirVal::Dir(Dir::L),
                _ => unreachable!(),
            };

            lookup_dirs.insert((start_key, end_key, 0), 1);
        }
    }

    for num_robots in 1..=25 {
        optimize_dirpad(num_robots, &mut lookup_dirs);
    }

    let lookup_keys = optimize_keypad(&lookup_dirs, 26);

    let mut sum = 0;

    for (code, prefix) in &data.codes {
        let solution = solve_with_lookups(code, &lookup_keys, 26);

        sum += solution * prefix;
    }

    dbg!(sum);
}

fn solve_with_lookups(
    code: &[KeyVal],
    lookup_key: &HashMap<(KeyVal, KeyVal, usize), usize>,
    num_robots: usize,
) -> usize {
    let mut current_pos = KeyVal::A;
    let mut sum_cost = 0;

    for goal in code {
        let cost_to = lookup_key.get(&(current_pos, *goal, num_robots)).unwrap();

        current_pos = *goal;

        sum_cost += cost_to;
    }

    sum_cost
}

fn optimize_keypad(
    lookup_dir: &HashMap<(DirVal, DirVal, usize), usize>,
    num_robots: usize,
) -> HashMap<(KeyVal, KeyVal, usize), usize> {
    let mut lookup = HashMap::new();

    for i in 0..11 {
        let start_key = match i {
            10 => KeyVal::A,
            i @ 0..10 => KeyVal::Num(i),
            _ => unreachable!(),
        };

        for i in 0..11 {
            let end_key = match i {
                10 => KeyVal::A,
                i @ 0..10 => KeyVal::Num(i),
                _ => unreachable!(),
            };

            // find fastest path

            let best_path = cost_to_press_from_to(
                start_key.into_pos(),
                end_key.into_pos(),
                |a, b| *lookup_dir.get(&(a, b, num_robots - 1)).unwrap(),
                |(x, y)| KeyVal::try_from_pos(x, y).is_none(),
            )
            .expect("A path should be possible");

            lookup.insert((start_key, end_key, num_robots), best_path);
        }
    }

    lookup
}

fn optimize_dirpad(
    num_robots: usize,
    cost_to_press_button: &mut HashMap<(DirVal, DirVal, usize), usize>,
) {
    for i in 0..=4 {
        let start_key = match i {
            0 => DirVal::A,
            1 => DirVal::Dir(Dir::U),
            2 => DirVal::Dir(Dir::R),
            3 => DirVal::Dir(Dir::D),
            4 => DirVal::Dir(Dir::L),
            _ => unreachable!(),
        };

        for i in 0..=4 {
            let end_key = match i {
                0 => DirVal::A,
                1 => DirVal::Dir(Dir::U),
                2 => DirVal::Dir(Dir::R),
                3 => DirVal::Dir(Dir::D),
                4 => DirVal::Dir(Dir::L),
                _ => unreachable!(),
            };

            // find fastest path

            let cost_to_press_goal = cost_to_press_from_to(
                start_key.into_pos(),
                end_key.into_pos(),
                |start_key, goal_key| {
                    *cost_to_press_button
                        .get(&(start_key, goal_key, num_robots - 1))
                        .unwrap()
                },
                |(x, y)| DirVal::try_from_pos(x, y).is_none(),
            )
            .expect("A path should be possible");

            cost_to_press_button.insert((start_key, end_key, num_robots), cost_to_press_goal);
        }
    }
}

fn cost_to_press_from_to(
    start: (usize, usize),
    end: (usize, usize),
    cost_to_press_button_from_start_pos: impl Fn(DirVal, DirVal) -> usize,
    reject: impl Fn((usize, usize)) -> bool,
) -> Option<usize> {
    let mut current_best_len = usize::MAX;
    let mut current_best_path = None;

    let mut current_len = 0;
    loop {
        let mut directions = vec![Dir::U; current_len];

        'current_len: loop {
            // Test options
            let mut pos = start;

            for dir in &directions {
                match (
                    pos.0.checked_add_signed(dir.into_offsets().0),
                    pos.1.checked_add_signed(dir.into_offsets().1),
                ) {
                    (None, None) => continue,
                    (None, Some(_)) => continue,
                    (Some(_), None) => continue,
                    (Some(x), Some(y)) => {
                        if reject((x, y)) {
                            break;
                        }
                        pos = (x, y);
                    }
                }
            }

            if end == pos {
                // This is path a correct option

                let path_len = if !directions.is_empty() {
                    let mut mover_current_pos = DirVal::A;

                    let mut path_len = 0;

                    for direction_to_press in &directions {
                        path_len += cost_to_press_button_from_start_pos(
                            mover_current_pos,
                            DirVal::Dir(*direction_to_press),
                        );

                        mover_current_pos = DirVal::Dir(*direction_to_press);
                    }

                    path_len += cost_to_press_button_from_start_pos(mover_current_pos, DirVal::A);

                    path_len
                } else {
                    cost_to_press_button_from_start_pos(DirVal::A, DirVal::A)
                };

                if path_len < current_best_len {
                    current_best_len = path_len;
                    current_best_path = Some(directions.clone());
                }
            }

            if directions.iter().all(|v| *v == Dir::L) {
                break 'current_len;
            }

            for i in 0..current_len {
                if directions[i] == Dir::L {
                    directions[i] = Dir::U;
                } else {
                    directions[i] = directions[i].turn_right();
                    break;
                }
            }
        }

        if current_best_path.is_some() {
            break;
        }

        current_len += 1;
    }

    Some(current_best_len)
}
