use std::{fs::File, io::Read, time::Instant};

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

struct Data {
    plots: Vec<Vec<char>>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let plots = input.lines().map(|line| line.chars().collect()).collect();

        Self { plots }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.plots.get(y)?.get(x)
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let from_str = Data::from_str(&input);
    let data = from_str;

    let time = Instant::now();

    part_two(&data);

    println!("{:?}", time.elapsed());
}

fn part_one(data: &Data) {
    calc(data, false);
}

fn part_two(data: &Data) {
    calc(data, true);
}

fn calc(data: &Data, sections: bool) {
    let mut fence_counts: Vec<Vec<Vec<Dir>>> = data
        .plots
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(|(x, c)| {
                    Dir::cardinals()
                        .filter(|dir| {
                            let (x_offs, y_offs) = dir.into_offsets();
                            if let Some(neighbor) = data
                                .get(x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs))
                            {
                                c != neighbor
                            } else {
                                true
                            }
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut is_part_of_region: Vec<Vec<bool>> = data
        .plots
        .iter()
        .map(|v| v.iter().map(|_| false).collect())
        .collect();

    let mut sum = 0;

    for y in 0..data.plots.len() {
        for x in 0..data.plots[y].len() {
            let (area, fence) = get_score_for_region(
                (x, y),
                data,
                &mut fence_counts,
                &mut is_part_of_region,
                sections,
            );

            sum += area * fence;
        }
    }

    println!("Total score is {sum}");
}

fn get_score_for_region(
    start_pos: (usize, usize),
    data: &Data,
    fence_dirs: &mut Vec<Vec<Vec<Dir>>>,
    is_part_of_region: &mut Vec<Vec<bool>>,
    count_sections: bool,
) -> (isize, isize) {
    if data.get(start_pos.0, start_pos.1).is_none() {
        return (0, 0);
    } else if is_part_of_region[start_pos.1][start_pos.0] {
        // We already were here
        return (0, 0);
    }

    is_part_of_region[start_pos.1][start_pos.0] = true;

    let mut area_sum = 1;
    let mut fence_sum: isize = 0;
    if count_sections {
        while let Some(dir) = fence_dirs[start_pos.1][start_pos.0].pop() {
            {
                let mut left_pos = start_pos;

                while data.get(start_pos.0, start_pos.1) == data.get(left_pos.0, left_pos.1) {
                    let old_len = fence_dirs[left_pos.1][left_pos.0].len();
                    fence_dirs[left_pos.1][left_pos.0].retain(|left_dir| *left_dir != dir);
                    if old_len == fence_dirs[left_pos.1][left_pos.0].len() && left_pos != start_pos
                    {
                        break;
                    }

                    left_pos = (
                        left_pos
                            .0
                            .wrapping_add_signed(dir.turn_left().into_offsets().0),
                        left_pos
                            .1
                            .wrapping_add_signed(dir.turn_left().into_offsets().1),
                    );
                }
            }

            {
                let mut right_pos = start_pos;

                while data.get(start_pos.0, start_pos.1) == data.get(right_pos.0, right_pos.1) {
                    let old_len = fence_dirs[right_pos.1][right_pos.0].len();
                    fence_dirs[right_pos.1][right_pos.0].retain(|right_dir| *right_dir != dir);
                    if old_len == fence_dirs[right_pos.1][right_pos.0].len()
                        && right_pos != start_pos
                    {
                        break;
                    }

                    right_pos = (
                        right_pos
                            .0
                            .wrapping_add_signed(dir.turn_right().into_offsets().0),
                        right_pos
                            .1
                            .wrapping_add_signed(dir.turn_right().into_offsets().1),
                    );
                }
            }

            fence_sum += 1;
        }
    } else {
        fence_sum = fence_sum.wrapping_add_unsigned(fence_dirs[start_pos.1][start_pos.0].len());
    }

    for dir in Dir::cardinals() {
        let (x_offs, y_offs) = dir.into_offsets();
        let neighbor_pos = (
            start_pos.0.wrapping_add_signed(x_offs),
            start_pos.1.wrapping_add_signed(y_offs),
        );

        if data.get(neighbor_pos.0, neighbor_pos.1) == data.get(start_pos.0, start_pos.1) {
            let (neighbor_area_sum, neightbor_fence_sum) = get_score_for_region(
                neighbor_pos,
                data,
                fence_dirs,
                is_part_of_region,
                count_sections,
            );
            area_sum += neighbor_area_sum;
            fence_sum += neightbor_fence_sum;
        }
    }

    (area_sum, fence_sum)
}

#[cfg(test)]
mod test {
    use proptest::{
        prelude::{prop, ProptestConfig, Strategy},
        prop_assert, proptest,
    };

    use crate::{get_score_for_region, Data, Dir};

    fn rect_vec(max_len: usize) -> impl Strategy<Value = Vec<Vec<char>>> {
        ((0..max_len), (0..max_len)).prop_flat_map(|(height, width)| {
            prop::collection::vec(
                prop::collection::vec(prop::char::range('A', 'Z'), width),
                height,
            )
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100_000))]
        #[test]
        fn test_all_polygons_have_even_number_of_sides(plots in rect_vec(10)) {
            let data = Data { plots };

            let mut fence_counts: Vec<Vec<Vec<Dir>>> = data
                .plots
                .iter()
                .enumerate()
                .map(|(y, v)| {
                    v.iter()
                        .enumerate()
                        .map(|(x, c)| {
                            Dir::cardinals()
                                .filter(|dir| {
                                    let (x_offs, y_offs) = dir.into_offsets();
                                    if let Some(neighbor) = data
                                        .get(x.wrapping_add_signed(x_offs), y.wrapping_add_signed(y_offs))
                                    {
                                        c != neighbor
                                    } else {
                                        true
                                    }
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect();

            let mut is_part_of_region: Vec<Vec<bool>> = data
                .plots
                .iter()
                .map(|v| v.iter().map(|_| false).collect())
                .collect();

            for y in 0..data.plots.len() {
                for x in 0..data.plots[y].len() {
                    let (area, fence) = get_score_for_region(
                        (x, y),
                        &data,
                        &mut fence_counts,
                        &mut is_part_of_region,
                        true,
                    );

                    if area > 0 {
                        prop_assert!(fence % 2 == 0);
                    }
                }
            }
        }
    }
}
