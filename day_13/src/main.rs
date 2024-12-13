struct Data {
    machines: Vec<Machine>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut machines = Vec::new();
        let re = Regex::new(r"Button A: (.*)\nButton B: (.*)\nPrize: (.*)").unwrap();

        for cap in re.captures_iter(input) {
            let button_a = parse_coordinates(&cap[1]);
            let button_b = parse_coordinates(&cap[2]);
            let prize = parse_coordinates(&cap[3]);

            machines.push(Machine {
                button_a,
                button_b,
                prize,
            });
        }

        Self { machines }
    }
}

use std::{
    fs::File,
    io::Read,
    ops::{Add, Mul, Sub},
    time::Instant,
};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i64) -> Self::Output {
        Coordinates {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize: Coordinates,
}

fn parse_coordinates(input: &str) -> Coordinates {
    let re = Regex::new(r"X=*([+-]?\d+),?\s*Y=*([+-]?\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    Coordinates {
        x: caps[1].parse().unwrap(),
        y: caps[2].parse().unwrap(),
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let mut data = Data::from_str(&input);

    let time = Instant::now();

    part_two(&mut data);

    println!("{:?}", time.elapsed());
}

fn part_one(data: &Data) {
    let mut cum_cost = 0;
    for machine in &data.machines {
        dbg!(machine);
        let Some(cost) = solve_bruteforce(
            machine.button_a,
            machine.button_b,
            machine.prize,
            std::cmp::min,
        ) else {
            println!("No solution found for machine {machine:?}");
            continue;
        };
        dbg!(cost);

        cum_cost += cost
    }

    dbg!(&cum_cost);
}

fn part_two(data: &mut Data) {
    let mut cum_cost = 0;
    for machine in &data.machines {
        dbg!(machine);
        let Some((a, b)) = solve(
            machine.button_a,
            machine.button_b,
            machine.prize
                + Coordinates {
                    x: 10000000000000,
                    y: 10000000000000,
                },
        ) else {
            println!("No solution found for machine {machine:?}");
            continue;
        };
        dbg!((a, b));

        cum_cost += 3 * a + b;
    }

    dbg!(&cum_cost);
}

fn solve(a_value: Coordinates, b_value: Coordinates, goal: Coordinates) -> Option<(i64, i64)> {
    if goal.x == 0 && goal.y == 0 {
        return Some((0, 0));
    }

    for a in 0..=9 {
        for b in 0..=9 {
            // Only look at the last digit
            let dest = (a_value * a) + (b_value * b);
            if dest.x % 10 == goal.x % 10 && dest.y % 10 == goal.y % 10 {
                // This is a possible solution for the last digit
                let new_goal = goal - dest;

                assert!(new_goal.x % 10 == 0);
                assert!(new_goal.y % 10 == 0);

                if let Some((next_a, next_b)) = solve(
                    a_value,
                    b_value,
                    Coordinates {
                        x: new_goal.x / 10,
                        y: new_goal.y / 10,
                    },
                ) {
                    // Found A solution
                    return Some((next_a * 10 + a, next_b * 10 + b));
                }
            }
        }
    }

    None
}

// I JUST read that button presses are limited to 100 so we can just bruteforce it lol
fn solve_bruteforce(
    a_value: Coordinates,
    b_value: Coordinates,
    goal: Coordinates,
    min_fn: impl Fn(i64, i64) -> i64,
) -> Option<i64> {
    let mut min = None;

    for a in 0..=100 {
        for b in 0..=100 {
            let dest = (a_value * a) + (b_value * b);

            if dest == goal {
                let cost = a * 3 + b;

                if let Some(old_min) = min {
                    min = Some(min_fn(old_min, cost))
                } else {
                    min = Some(cost)
                }
            }
        }
    }

    min
}
