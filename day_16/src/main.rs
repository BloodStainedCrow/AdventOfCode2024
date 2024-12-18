use std::{fs::File, io::Read};

use itertools::Itertools;
use petgraph::{
    algo::{astar, k_shortest_path},
    data,
    prelude::{DiGraphMap, GraphMap},
    visit::GraphBase,
    Graph,
};
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

struct Data {
    map: Vec<Vec<Tile>>,
    graph_info: GraphInfo<GraphMap<((usize, usize), Dir), i32, petgraph::Directed>>,
}

struct GraphInfo<G: GraphBase> {
    graph: G,
    start: ((usize, usize), Dir),
    end: (usize, usize),
}

impl Data {
    fn from_str(input: &str) -> Self {
        let (mut start, mut end) = (None, None);

        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'S' => {
                            assert!(start.is_none());
                            start = Some(((x, y), Dir::R));
                            Tile::Empty
                        }
                        'E' => {
                            assert!(end.is_none());
                            end = Some((x, y));
                            Tile::Empty
                        }
                        _ => panic!("unexpected char"),
                    })
                    .collect()
            })
            .collect();

        let mut graph = DiGraphMap::new();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == Tile::Wall {
                    continue;
                }

                for dir in Dir::cardinals() {
                    graph.add_edge(((x, y), dir), ((x, y), dir.turn_left()), 1000);
                    graph.add_edge(((x, y), dir), ((x, y), dir.turn_right()), 1000);

                    let offs = dir.into_offsets();

                    // TODO: This will potentially wrap. should be fine since the map is sorrounded by walls
                    let next_pos = (
                        x.checked_add_signed(offs.0).unwrap(),
                        y.checked_add_signed(offs.1).unwrap(),
                    );

                    let tile = map[next_pos.1][next_pos.0];

                    if tile == Tile::Empty {
                        graph.add_edge(((x, y), dir), (next_pos, dir), 1);
                    }
                }
            }
        }

        Self {
            map,
            graph_info: GraphInfo {
                graph,
                start: start.expect("No start found"),
                end: end.expect("No end found"),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
enum Tile {
    Empty,
    Wall,
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
    // println!("{:?}", data.graph_info.graph);

    let Some((cost, _path)) = astar(
        &data.graph_info.graph,
        data.graph_info.start,
        |((x, y), _dir)| (x, y) == data.graph_info.end,
        |(a, b, v)| *v,
        |(pos, _dir)| {
            (data.graph_info.end.0.abs_diff(pos.0) + data.graph_info.end.1.abs_diff(pos.1))
                .try_into()
                .unwrap()
        },
    ) else {
        panic!("No path found!!");
    };

    println!("Cost is {cost}")
}

fn part_two(data: &Data) {
    let mut k = 1;

    let mut all_same_k = 1;

    let mut fine_grained = false;

    loop {
        dbg!(k);

        let ret = pathfinding::directed::yen::yen(
            &data.graph_info.start,
            |node| {
                let cloned = *node;
                data.graph_info.graph.neighbors(*node).map(move |n| {
                    let cost = if n.1 == cloned.1 {
                        // We went straight
                        1
                    } else {
                        1000
                    };
                    (n, cost)
                })
            },
            |node| node.0 == data.graph_info.end,
            k,
        );

        if ret.iter().all(|(_, cost)| *cost == ret[0].1) {
            if fine_grained {
                k += 1;
            } else {
                all_same_k = k;
                k *= 2;
            }
        } else if fine_grained {
            break;
        } else {
            k = all_same_k;
            fine_grained = true;
        }
    }

    // k is not just high enough to include paths with not min cost!
    k -= 1;

    let ret = pathfinding::directed::yen::yen(
        &data.graph_info.start,
        |node| {
            let cloned = *node;
            data.graph_info.graph.neighbors(*node).map(move |n| {
                let cost = if n.1 == cloned.1 {
                    // We went straight
                    1
                } else {
                    1000
                };
                (n, cost)
            })
        },
        |node| node.0 == data.graph_info.end,
        k,
    );

    let count = ret
        .iter()
        .flat_map(|(path, cost)| path.iter())
        .map(|(pos, dir)| pos)
        .unique()
        .count();

    println!("{count} tiles are on a best path");
}
