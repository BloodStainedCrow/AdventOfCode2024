use std::{
    fs::File,
    io::{stdin, Read},
    thread::sleep,
    time::Duration,
};

struct Data {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Data {
    fn from_str(input: &str, width: usize, height: usize) -> Self {
        let robots: Vec<Robot> = input
            .lines()
            .map(|line| line.split_once(' ').expect("could not parse"))
            .map(|(p, v)| (p.split_once('=').unwrap().1, v.split_once('=').unwrap().1))
            .map(|(p, v)| (p.split_once(',').unwrap(), v.split_once(',').unwrap()))
            .map(|((p_x, p_y), (v_x, v_y))| Robot {
                pos: (p_x.parse().unwrap(), p_y.parse().unwrap()),
                velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
            })
            .collect();

        for r in robots.iter() {
            assert!(r.pos.0 < width as isize);
            assert!(r.pos.1 < height as isize);
        }

        Self {
            width,
            height,
            robots,
        }
    }
}

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    let from_str = Data::from_str(&input, 101, 103);
    let mut data = from_str;

    part_two_questionmark(&mut data);
}

fn part_one(data: &mut Data) {
    let factor = simulate_bruteforce(data, Duration::from_secs(100));

    println!("Safety factor is {factor}");
}

fn part_two_questionmark(data: &mut Data) {
    let mut time_passed = Duration::from_secs(0);

    loop {
        simulate_bruteforce(data, Duration::from_secs(1));

        let mut _s = String::new();

        // sleep(Duration::from_millis(100));

        time_passed += Duration::from_secs(1);

        let std_dev = calc_std_dev(data);

        if std_dev < 40.0 {
            dbg!(std_dev);
            print_map(data);
            println!("Time: {time_passed:?}");
            // stdin().read_line(&mut _s).expect("Reading failed!");
            sleep(Duration::from_millis(200));
        }
    }
}

fn calc_std_dev(data: &Data) -> f64 {
    let sum_pos = data
        .robots
        .iter()
        .map(|r| r.pos)
        .fold((0, 0), |(curr_x, curr_y), (x, y)| (curr_x + x, curr_y + y));

    let avg_pos = (
        sum_pos.0 as f64 / data.robots.len() as f64,
        sum_pos.1 as f64 / data.robots.len() as f64,
    );

    (data
        .robots
        .iter()
        .map(|r| r.pos)
        .map(|(x, y)| ((x as f64 - avg_pos.0).abs() + (y as f64 - avg_pos.1).abs()).powi(2))
        .sum::<f64>()
        / data.robots.len() as f64)
        .sqrt()
}

fn print_map(data: &Data) {
    for y in 0..data.height {
        for x in 0..data.width {
            let count = data
                .robots
                .iter()
                .filter(|r| r.pos.0 == x as isize && r.pos.1 == y as isize)
                .count();

            assert!(count < 10);

            if count == 0 {
                print!("..");
            } else {
                print!("{:02}", count);
            }
        }
        println!();
    }
}

fn simulate_bruteforce(data: &mut Data, time: Duration) -> usize {
    for t in 0..time.as_secs() {
        for robot in &mut data.robots {
            let new_pos_pre_mod = (
                robot.pos.0 + robot.velocity.0,
                robot.pos.1 + robot.velocity.1,
            );

            let new_pos = (
                new_pos_pre_mod.0.rem_euclid(data.width as isize),
                new_pos_pre_mod.1.rem_euclid(data.height as isize),
            );

            robot.pos = new_pos;
        }
    }

    // Do the quadrant eval:
    let mut vals = [[0, 0], [0, 0]];

    for robot in &mut data.robots {
        assert!(robot.pos.0 >= 0 && robot.pos.0 < data.width as isize);
        assert!(robot.pos.1 >= 0 && robot.pos.1 < data.height as isize);

        let x_quad = if robot.pos.0 < (data.width / 2) as isize {
            0usize
        } else if robot.pos.0 >= (data.width.div_ceil(2)) as isize {
            1usize
        } else {
            // To reach this, the board must be odd in width!
            assert_eq!(robot.pos.0, (data.width / 2) as isize);
            continue;
        };

        let y_quad = if robot.pos.1 < (data.height / 2) as isize {
            0usize
        } else if robot.pos.1 >= (data.height.div_ceil(2)) as isize {
            1usize
        } else {
            // To reach this, the board must be odd in height!
            assert_eq!(robot.pos.1, (data.height / 2) as isize);
            continue;
        };

        vals[y_quad][x_quad] += 1;
    }

    vals.iter().flatten().product()
}
