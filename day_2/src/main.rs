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
    let mut num_safe = 0;
    for line in input.lines() {
        if is_report_safe(
            line.split_whitespace()
                .map(str::parse)
                .map(|r| r.expect("Could not convert to u32")),
        ) {
            num_safe += 1;
        }

        assert!(!line.is_empty());
    }

    dbg!(num_safe);
}

fn part_two(input: &str) {
    let mut num_safe = 0;
    'lines: for line in input.lines() {
        for skip_index in 0..line.split_whitespace().count() {
            if is_report_safe(
                line.split_whitespace()
                    .map(str::parse)
                    .map(|r| r.expect("Could not convert to u32"))
                    .enumerate()
                    .filter(|(i, _)| *i != skip_index)
                    .map(|(_, v)| v),
            ) {
                num_safe += 1;
                continue 'lines;
            }
        }

        assert!(!line.is_empty());
    }

    dbg!(num_safe);
}

fn is_report_safe(levels: impl IntoIterator<Item = u32>) -> bool {
    let mut line_dir = None;
    let mut last_value: Option<u32> = None;

    for level in levels {
        if let Some(last_value) = last_value {
            if let Some(dir) = line_dir {
                if dir != last_value.cmp(&level) {
                    // Line changed direction
                    // Unsafe
                    return false;
                }
            } else {
                line_dir = Some(last_value.cmp(&level));
            }

            // Check that the diff is fine
            if last_value.abs_diff(level) < 1 || last_value.abs_diff(level) > 3 {
                // Big jump
                // Unsafe
                return false;
            }
        }

        last_value = Some(level);
    }

    true
}
