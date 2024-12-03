use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();

    File::open("./input.txt")
        .expect("Could not open File")
        .read_to_string(&mut input)
        .expect("Could not read File");

    part_two(&input);
}

fn part_one(input: &str) {
    // let input_vec: Vec<char> = input.chars().collect();
    //
    // loop {
    //     let mut i = 0;
    //     let mut sum = 0;
    //     match &input_vec.as_slice()[i..] {
    //         ['m', 'u', 'l', '(', a @ .., ',', b @ .., ')'] => {
    //             // Check if a and b are numbers
    //             let Ok(a) = str::parse(a) else {
    //                 i += 1;
    //                 continue;
    //             };
    //             let Ok(b): i64 = str::parse(b);
    //         }
    //         _ => todo!(),
    //     }
    // }

    let mut cum_len = 0;
    let mut sum = 0;
    loop {
        if cum_len >= input.len() {
            break;
        }

        let current_read_head = &input[cum_len..];

        if let Some(((a, b), len)) = try_eat_mul(current_read_head) {
            cum_len += len;
            sum += a * b;

            dbg!((a, b));
        } else {
            cum_len += 1;
        }
    }

    dbg!(sum);
}

fn part_two(input: &str) {
    let mut cum_len = 0;
    let mut sum = 0;
    let mut enabled = true;
    loop {
        if cum_len >= input.len() {
            break;
        }

        let current_read_head = &input[cum_len..];

        if let Some(len) = try_eat_do(current_read_head) {
            cum_len += len;
            enabled = true;
        } else if let Some(len) = try_eat_dont(current_read_head) {
            cum_len += len;
            enabled = false;
        } else if let Some(((a, b), len)) = try_eat_mul(current_read_head) {
            cum_len += len;
            if enabled {
                sum += a * b;
            }

            dbg!((a, b));
            dbg!(enabled);
        } else {
            cum_len += 1;
        }
    }

    dbg!(sum);
}

fn try_eat_do(s: &str) -> Option<usize> {
    if s.starts_with("do()") {
        Some(4)
    } else {
        None
    }
}

fn try_eat_dont(s: &str) -> Option<usize> {
    if s.starts_with("don't()") {
        Some(4)
    } else {
        None
    }
}

fn try_eat_mul(s: &str) -> Option<((i64, i64), usize)> {
    if s.starts_with("mul(") {
        let (a, a_len) = try_eat_number(&s[4..])?;

        if !s[(4 + a_len)..].starts_with(',') {
            return None;
        }

        let (b, b_len) = try_eat_number(&s[(4 + a_len + 1)..])?;

        let end_len = try_eat_end(&s[(4 + a_len + 1 + b_len)..])?;

        Some(((a, b), 4 + a_len + 1 + b_len + end_len))
    } else {
        None
    }
}

fn try_eat_number(input: &str) -> Option<(i64, usize)> {
    let mut s = String::new();

    let mut cum_len = 0;
    while let Some((c, len)) = try_eat_digit(&input[cum_len..]) {
        s.push(c);
        cum_len += len;
    }

    let Ok(value) = str::parse(&s) else {
        return None;
    };

    Some((value, cum_len))
}

fn try_eat_digit(s: &str) -> Option<(char, usize)> {
    let c = s.chars().next()?;
    if c.is_ascii_digit() {
        Some((c, 1))
    } else {
        None
    }
}

fn try_eat_end(s: &str) -> Option<usize> {
    if s.starts_with(')') {
        Some(1)
    } else {
        None
    }
}
