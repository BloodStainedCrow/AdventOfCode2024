use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    time::Instant,
};

use strum_macros::EnumIter;

use strum::IntoEnumIterator;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<MapPos>>,
    player_pos: (isize, isize),
    player_dir: Dir,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut player_pos = None;
        let mut player_dir = None;

        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '.' => MapPos::Empty,
                        '#' => MapPos::Blocked,
                        '^' => {
                            player_dir = Some(Dir::U);

                            player_pos = Some((x as isize, y as isize));

                            MapPos::Empty
                        }
                        _ => todo!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles,
            player_pos: player_pos.expect("No player found"),
            player_dir: player_dir.expect("No player found"),
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&MapPos> {
        self.tiles.get(y as usize)?.get(x as usize)
    }
}

#[derive(Debug, PartialEq)]
enum MapPos {
    Empty,
    Blocked,
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
        match self {
            Dir::UL => (-1, -1),
            Dir::U => (-1, 0),
            Dir::UR => (-1, 1),
            Dir::R => (0, 1),
            Dir::DR => (1, 1),
            Dir::D => (1, 0),
            Dir::DL => (1, -1),
            Dir::L => (0, -1),
        }
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

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let map = Map::from_str(&input);

    let time = Instant::now();

    part_two_smart(&map);

    println!("{:?}", time.elapsed());
}

fn part_one(map: &Map) {
    let mut visited_positions = HashSet::new();

    visited_positions.insert(map.player_pos);

    let mut player_pos = map.player_pos;
    let mut player_dir = map.player_dir;

    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        map.get(player_pos.0 + go_offset.1, player_pos.1 + go_offset.0)
    } {
        let go_offset = player_dir.into_offsets();
        match pos {
            MapPos::Empty => {
                player_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);
                visited_positions.insert(player_pos);
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
                visited_positions.insert(player_pos);
            }
        }

        // draw_map(map, &visited_positions);
        // sleep(Duration::from_millis(500))
    }

    dbg!(visited_positions.len());
}

fn draw_map(map: &Map, locations: &HashSet<(isize, isize)>) {
    for y in 0..map.tiles[0].len() {
        for x in 0..map.tiles.len() {
            let map_char = match map
                .get(x as isize, y as isize)
                .expect("We iterate over the bounds")
            {
                MapPos::Empty => '.',
                MapPos::Blocked => '#',
            };

            if locations.contains(&(x as isize, y as isize)) {
                print!("X");
            } else {
                print!("{}", map_char);
            }
        }
        println!();
    }

    println!("---------------------------------------------------------------------------------------------------------");
}

fn part_two_simple_bruteforce(map: &Map) {
    // Do a normal run first
    let mut visited_positions = HashSet::new();

    visited_positions.insert(map.player_pos);

    let mut player_pos = map.player_pos;
    let mut player_dir = map.player_dir;

    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        map.get(player_pos.0 + go_offset.1, player_pos.1 + go_offset.0)
    } {
        let go_offset = player_dir.into_offsets();
        match pos {
            MapPos::Empty => {
                player_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);
                visited_positions.insert(player_pos);
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
                visited_positions.insert(player_pos);
            }
        }

        // draw_map(map, &visited_positions);
        // sleep(Duration::from_millis(500))
    }

    let mut num_loops = 0;
    for y in 0..map.tiles[0].len() {
        for x in 0..map.tiles.len() {
            let extra_obstacle_pos = (x as isize, y as isize);
            if extra_obstacle_pos == map.player_pos {
                // The guard is there right now and would notice
                continue;
            }

            if !visited_positions.contains(&extra_obstacle_pos) {
                // We don't hit the obstacle, no need to run
                continue;
            }

            if contains_loop(map, extra_obstacle_pos, None) {
                num_loops += 1;
            }
        }
    }

    // This uses println! instead of dbg! for the first time, since this is slow enough where release mode makes sense xD
    println!("{num_loops} options result in loops");
}

fn part_two_smart_bruteforce(map: &Map) {
    let mut visited_positions = HashMap::new();

    visited_positions.insert((map.player_pos, map.player_dir), 0);

    let mut player_pos = map.player_pos;
    let mut player_dir = map.player_dir;
    let mut step = 0;

    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        let go_to_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        map.get(go_to_pos.0, go_to_pos.1)
    } {
        let go_offset = player_dir.into_offsets();
        match pos {
            MapPos::Empty => {
                player_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);
                // We moved, check if we reached a pos we already had!
                if visited_positions
                    .insert((player_pos, player_dir), step)
                    .is_some()
                {
                    // We were here once and looked in the same direction.
                    // This means this is a loop!
                    unreachable!("Reaching this code means the starting input contains a loop!");
                }
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
            }
        }

        step += 1;
    }

    let mut num_loops = 0;
    for y in 0..map.tiles[0].len() {
        for x in 0..map.tiles.len() {
            let extra_obstacle_pos = (x as isize, y as isize);
            if extra_obstacle_pos == map.player_pos {
                // The guard is there right now and would notice
                continue;
            }

            let mut earliest_step_hit = None;
            for (step, dir) in Dir::cardinals().filter_map(|dir| {
                visited_positions
                    .get(&(extra_obstacle_pos, dir))
                    .map(|step| (step, dir))
            }) {
                if let Some((earliest_step, _)) = earliest_step_hit {
                    if step < earliest_step {
                        earliest_step_hit = Some((step, dir))
                    }
                } else {
                    earliest_step_hit = Some((step, dir))
                }
            }

            if let Some((_, dir)) = earliest_step_hit {
                let start_pos = (
                    extra_obstacle_pos.0 + dir.get_opposite().into_offsets().1,
                    extra_obstacle_pos.1 + dir.get_opposite().into_offsets().0,
                );

                if contains_loop(map, extra_obstacle_pos, Some((start_pos, dir))) {
                    num_loops += 1;
                    continue;
                }
            }
        }
    }

    // This uses println! instead of dbg! for the first time, since this is slow enough where release mode makes sense xD
    println!("{num_loops} options result in loops");
}

// TODO: Test if this is actually faster lol
fn part_two_smart(map: &Map) {
    // Idea: We turn the player around at the start and have it run until it runs into a position a forward run would run into.
    // If there is no such position we cannot get a loop, otherwise place an obstacle such that the player turns onto our backwards path

    let mut num_loops = 0;

    // Do a normal run
    let mut visited_positions = HashSet::new();
    let mut rock_locations = HashSet::new();

    visited_positions.insert((map.player_pos, map.player_dir));

    let mut player_pos = map.player_pos;
    let mut player_dir = map.player_dir;
    let mut step = 0;
    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        let next_pos_if_straight = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        map.get(next_pos_if_straight.0, next_pos_if_straight.1)
    } {
        let go_offset = player_dir.into_offsets();

        let next_pos_if_straight = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        match pos {
            MapPos::Empty => {
                assert!(visited_positions.contains(&(player_pos, player_dir)));

                // The next position is empty, what if is wasn't?
                // Only check each position ONCE
                if rock_locations.insert(next_pos_if_straight) {
                    if test_if_rock_here_means_loop(map, &visited_positions, player_pos, player_dir)
                    {
                        num_loops += 1;

                        assert!(map
                            .get(next_pos_if_straight.0, next_pos_if_straight.1)
                            .is_some());

                        debug_assert!(contains_loop(map, next_pos_if_straight, None));
                    } else {
                        debug_assert!(!contains_loop(map, next_pos_if_straight, None));
                    }
                }

                player_pos = next_pos_if_straight;
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
            }
        }

        // We moved, check if we reached a pos we already had!
        if !visited_positions.insert((player_pos, player_dir)) {
            // We were here once and looked in the same direction.
            // This means this is a loop!
            unreachable!("Reaching this code means the starting input contains a loop!");
        }

        step += 1;
    }

    println!("Found {num_loops} positions resulting in loops")
}

fn test_if_rock_here_means_loop(
    map: &Map,
    visited_positions: &HashSet<((isize, isize), Dir)>,
    current_pos: (isize, isize),
    current_dir: Dir,
) -> bool {
    let mut own_visited_pos = HashSet::new();

    let rock_pos = (
        current_pos.0 + current_dir.into_offsets().1,
        current_pos.1 + current_dir.into_offsets().0,
    );

    own_visited_pos.insert((current_pos, current_dir));

    let mut player_pos = current_pos;
    let mut player_dir = current_dir;

    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        let go_to_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        map.get(go_to_pos.0, go_to_pos.1)
    } {
        let go_offset = player_dir.into_offsets();

        let next_pos_if_straight = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        match pos {
            MapPos::Empty => {
                if next_pos_if_straight == rock_pos {
                    // Act as if there was a rock there
                    player_dir = player_dir.turn_right();
                } else {
                    // Act normally
                    player_pos = next_pos_if_straight;
                }
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
            }
        }

        // We moved, check if we reached a pos we already had!
        if let Some(step) = visited_positions.get(&(player_pos, player_dir)) {
            // We were here once and looked in the same direction.
            // This means this is a loop with the main!

            return true;
        }
        if !own_visited_pos.insert((player_pos, player_dir)) {
            // We were here once and looked in the same direction.
            // This means this is a loop with iself!

            return true;
        }
    }

    false
}

fn how_did_we_get_here(
    map: &Map,
    visited_positions: &mut HashMap<((isize, isize), Dir), i32>,
    current_pos: (isize, isize),
    current_dir: Dir,
    step: i32,
) {
    if Some(&MapPos::Empty) != map.get(current_pos.0, current_pos.1) {
        // We could not stand here
        return;
    }

    if visited_positions
        .insert((current_pos, current_dir), step)
        .is_some()
        && step != 0
    {
        // We were here once and looked in the same direction.
        // This means this is a loop!
        unreachable!("Reaching this code means the starting input contains a loop!");
    }

    // Check if there is a rock that could have turned us
    let rock_dir = current_dir.turn_left();

    if Some(&MapPos::Blocked)
        == map.get(
            current_pos.0 + rock_dir.into_offsets().1,
            current_pos.1 + rock_dir.into_offsets().0,
        )
    {
        // We could have come from the right and turned
        let come_from_dir = current_dir.turn_left();
        let come_from_offset = come_from_dir.get_opposite().into_offsets();

        how_did_we_get_here(
            map,
            visited_positions,
            (
                current_pos.0 + come_from_offset.1,
                current_pos.1 + come_from_offset.0,
            ),
            come_from_dir,
            step - 1,
        );
    }

    // Check if we could have just gone straight
    if Some(&MapPos::Empty)
        == map.get(
            current_pos.0 + current_dir.into_offsets().1,
            current_pos.1 + current_dir.into_offsets().0,
        )
    {
        // We could have come from the back and gone straight
        let come_from_dir = current_dir;
        let come_from_offset = come_from_dir.get_opposite().into_offsets();

        how_did_we_get_here(
            map,
            visited_positions,
            (
                current_pos.0 + come_from_offset.1,
                current_pos.1 + come_from_offset.0,
            ),
            come_from_dir,
            step - 1,
        );
    }
}

fn contains_loop(
    map: &Map,
    extra_obstacle_pos: (isize, isize),
    custom_start: Option<((isize, isize), Dir)>,
) -> bool {
    let mut visited_positions = HashSet::new();

    visited_positions.insert((map.player_pos, map.player_dir));

    let (mut player_pos, mut player_dir) = if let Some((pos, dir)) = custom_start {
        (pos, dir)
    } else {
        (map.player_pos, map.player_dir)
    };

    while let Some(pos) = {
        let go_offset = player_dir.into_offsets();
        // FIXME: Dir::into_offset is a tuple (y, x)!
        let go_to_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);

        if go_to_pos == extra_obstacle_pos {
            Some(&MapPos::Blocked)
        } else {
            map.get(go_to_pos.0, go_to_pos.1)
        }
    } {
        let go_offset = player_dir.into_offsets();
        match pos {
            MapPos::Empty => {
                player_pos = (player_pos.0 + go_offset.1, player_pos.1 + go_offset.0);
                // We moved, check if we reached a pos we already had!
                if !visited_positions.insert((player_pos, player_dir)) {
                    // We were here once and looked in the same direction.
                    // This means this is a loop!
                    return true;
                }
            }
            MapPos::Blocked => {
                player_dir = player_dir.turn_right();
            }
        }
    }

    // We left the map, no loop
    false
}
