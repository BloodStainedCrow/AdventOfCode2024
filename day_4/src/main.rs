use std::{fs::File, io::Read};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
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

    fn diags() -> impl Iterator<Item = Self> {
        Self::iter().filter(|dir| dir.into_offsets().0.abs() + dir.into_offsets().1.abs() == 2)
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let mut line_vecs = vec![];
        for line in input.lines() {
            let line_vec = line.chars().collect();
            line_vecs.push(line_vec);
        }

        Grid { grid: line_vecs }
    }

    fn get(&self, x: isize, y: isize) -> Option<&char> {
        match self.grid.get(usize::try_from(x).ok()?) {
            Some(row) => row.get(usize::try_from(y).ok()?),
            None => None,
        }
    }

    fn get_offset(&self, x: isize, y: isize, dir: Dir) -> Option<&char> {
        let (x_offs, y_offs) = dir.into_offsets();

        self.get(x + x_offs, y + y_offs)
    }

    fn num_xmas_at_pos(&self, x: isize, y: isize) -> usize {
        if self.get(x, y) != Some(&'X') {
            return 0;
        }

        let mut count = 0;
        for dir in Dir::iter() {
            let (x_offs, y_offs) = dir.into_offsets();

            if self.get(x + x_offs, y + y_offs) == Some(&'M')
                && self.get(x + 2 * x_offs, y + 2 * y_offs) == Some(&'A')
                && self.get(x + 3 * x_offs, y + 3 * y_offs) == Some(&'S')
            {
                count += 1;
            }
        }

        count
    }

    fn has_mas_cross_at_pos(&self, x: isize, y: isize) -> bool {
        if self.get(x, y) != Some(&'A') {
            return false;
        }

        let mut m_dir = vec![];
        let mut s_count = 0;
        for diag in Dir::diags() {
            if self.get_offset(x, y, diag) == Some(&'M') {
                m_dir.push(diag);
            } else if self.get_offset(x, y, diag) == Some(&'S') {
                s_count += 1;
            }
        }

        m_dir.len() == 2 && s_count == 2 && m_dir[0].get_opposite() != m_dir[1]
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let grid = Grid::from_str(&input);

    part_two(&grid);
}

fn part_one(grid: &Grid) {
    let mut sum = 0;

    for x in 0..grid.grid.len() {
        for y in 0..grid.grid[x].len() {
            sum += grid.num_xmas_at_pos(x as isize, y as isize);
        }
    }

    dbg!(sum);
}

fn part_two(grid: &Grid) {
    let mut sum = 0;

    for x in 0..grid.grid.len() {
        for y in 0..grid.grid[x].len() {
            sum += i32::from(grid.has_mas_cross_at_pos(x as isize, y as isize));
        }
    }

    dbg!(sum);
}
