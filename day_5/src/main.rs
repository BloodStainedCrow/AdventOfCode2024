use std::{cmp, fs::File, io::Read};

#[derive(Debug, Clone)]
struct Data {
    orderings: Vec<Ordering>,

    pages: Vec<Vec<Page>>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let mut orderings = vec![];
        let mut pages = vec![];

        let mut orderings_done = false;
        for line in input.lines() {
            if line.is_empty() {
                orderings_done = true;
                continue;
            }

            if !orderings_done {
                orderings.push(Ordering::from_str(line));
            } else {
                pages.push(line.split(',').flat_map(str::parse).map(Page).collect())
            }
        }

        Self { orderings, pages }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Page(usize);

impl Page {
    fn cmp_with(self, other: Self, orderings: &[Ordering]) -> Option<cmp::Ordering> {
        assert_ne!(self, other);

        let lt = orderings
            .iter()
            .find(|ord| ord.0 == self.0 && ord.1 == other.0);

        let gt = orderings
            .iter()
            .find(|ord| ord.1 == self.0 && ord.0 == other.0);

        assert!(!(lt.is_some() && gt.is_some()));

        if lt.is_some() {
            Some(cmp::Ordering::Less)
        } else if gt.is_some() {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ordering(usize, usize);

impl Ordering {
    fn from_str(input: &str) -> Self {
        let (a, b) = input.split_once('|').expect("Wrong pattern for Ordering");

        Self(
            str::parse(a).expect("ordering not number"),
            str::parse(b).expect("ordering not number"),
        )
    }

    fn satisfies(&self, data: &[Page]) -> bool {
        if !data.contains(&Page(self.0)) || !data.contains(&Page(self.1)) {
            true
        } else {
            data.iter().position(|v| v.0 == self.0) < data.iter().position(|v| v.0 == self.1)
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
    let mut sum = 0;

    for page_list in &data.pages {
        if data.orderings.iter().all(|ord| ord.satisfies(page_list)) {
            dbg!(page_list);

            sum += page_list[page_list.len() / 2].0;
        }
    }

    dbg!(sum);
}

fn part_two(data: &Data) {
    let mut data = data.clone();

    let mut sum = 0;

    for page_list in &mut data.pages {
        if data.orderings.iter().all(|ord| ord.satisfies(page_list)) {
        } else {
            page_list.sort_by(|a, b| {
                a.cmp_with(*b, &data.orderings)
                    .unwrap_or(cmp::Ordering::Equal)
            });

            assert!(data.orderings.iter().all(|ord| ord.satisfies(page_list)));

            sum += page_list[page_list.len() / 2].0;
        }
    }

    dbg!(sum);
}
