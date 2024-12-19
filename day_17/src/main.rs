use std::{
    array,
    collections::HashMap,
    fs::File,
    io::Read,
    ops::{BitXorAssign, ShlAssign, ShrAssign},
    sync::{atomic::AtomicBool, Arc},
    thread::{self, sleep},
    time::Duration,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Data {
    inital_state: State,
    program: Vec<Instruction>,
}

#[derive(Debug, Clone)]
struct State {
    ip: usize,

    a: u64,
    b: u64,
    c: u64,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    ADV(ComboOperand),
    BXL(LiteralOperand),
    BST(ComboOperand),
    JNZ(LiteralOperand),
    BXC(LiteralOperand),
    OUT(ComboOperand),
    BDV(ComboOperand),
    CDV(ComboOperand),
}

impl Instruction {
    fn try_from_values(values: (u8, u8)) -> Option<Self> {
        match values.0 {
            0 => Some(Self::ADV(values.1.try_into().ok()?)),
            1 => Some(Self::BXL(values.1.try_into().ok()?)),
            2 => Some(Self::BST(values.1.try_into().ok()?)),
            3 => Some(Self::JNZ(values.1.try_into().ok()?)),
            4 => Some(Self::BXC(values.1.try_into().ok()?)),
            5 => Some(Self::OUT(values.1.try_into().ok()?)),
            6 => Some(Self::BDV(values.1.try_into().ok()?)),
            7 => Some(Self::CDV(values.1.try_into().ok()?)),
            _ => None,
        }
    }

    fn bytecode(&self) -> [u8; 2] {
        match self {
            Instruction::ADV(op) => (0, op.0),
            Instruction::BXL(op) => (1, op.0),
            Instruction::BST(op) => (2, op.0),
            Instruction::JNZ(op) => (3, op.0),
            Instruction::BXC(op) => (4, op.0),
            Instruction::OUT(op) => (5, op.0),
            Instruction::BDV(op) => (6, op.0),
            Instruction::CDV(op) => (7, op.0),
        }
        .into()
    }
}

trait Operand {
    fn evaluate(&self, state: &State) -> u64;
}

#[derive(Debug, Clone, PartialEq)]
struct LiteralOperand(u8);

impl TryFrom<u8> for LiteralOperand {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl Operand for LiteralOperand {
    fn evaluate(&self, _state: &State) -> u64 {
        self.0.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ComboOperand(u8);

impl TryFrom<u8> for ComboOperand {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 6 {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl Operand for ComboOperand {
    fn evaluate(&self, state: &State) -> u64 {
        match self.0 {
            0..=3 => self.0.into(),
            4 => state.a,
            5 => state.b,
            6 => state.c,
            _ => unreachable!("Invalid ComboOperand"),
        }
    }
}

impl Data {
    fn from_str(input: &str) -> Self {
        let (state, instructions) = input.split_once("\n\n").expect("Could not find empty line");

        let (a, (b, c)) = state
            .split_once('\n')
            .map(|(a, bc)| (a, bc.split_once('\n').unwrap()))
            .unwrap();

        let a = a
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .parse()
            .expect("Could not parse value");
        let b = b
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .parse()
            .expect("Could not parse value");
        let c = c
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .parse()
            .expect("Could not parse value");

        let inital_state = State { ip: 0, a, b, c };

        let program = instructions
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(',')
            .map(|s| s.parse().expect("Program is not a number"))
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let opcode = chunk.next().unwrap();
                let operand = chunk.next().unwrap();

                Instruction::try_from_values((opcode, operand))
                    .expect("Could not turn values into instruction")
            })
            .collect();

        Self {
            inital_state,
            program,
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
    let mut output = Vec::new();

    let mut state = data.inital_state.clone();

    while let Some(instruction) = data.program.get(state.ip) {
        if let Some(out) = execute_instruction(instruction, &mut state) {
            output.push(out);
        }
    }

    dbg!(output);
}

fn part_two_bruteforce(data: &Data) {
    const NUM_THREADS: u64 = 10;

    // I just let the bruteforce run while i wrote the better solution lol
    // const STARTVALUE: u64 = 10_586_306_000_000;
    const STARTVALUE: u64 = 212928664708;

    let data = Arc::new(data.clone());

    let goal: Arc<Vec<u8>> = Arc::new(data.program.iter().flat_map(|i| i.bytecode()).collect());

    let mut handles = Vec::new();

    let found = Arc::new(AtomicBool::new(false));

    for thread_idx in 0..NUM_THREADS {
        let inner_goal = goal.clone();
        let inner_data = data.clone();
        let inner_found = found.clone();
        let handle = thread::spawn(move || {
            let mut a_value = STARTVALUE + thread_idx;

            while !inner_found.load(std::sync::atomic::Ordering::Relaxed) {
                if thread_idx == 0 && a_value % 1_000_000 == 0 {
                    dbg!(a_value);
                }

                let mut state = inner_data.inital_state.clone();

                state.a = a_value;

                let mut out_index = 0;
                let mut found_diff = false;

                while let Some(instruction) = inner_data.program.get(state.ip) {
                    if let Some(out) = execute_instruction(instruction, &mut state) {
                        if inner_goal[out_index] != out {
                            found_diff = true;
                            break;
                        }

                        out_index += 1;
                    }
                }

                if found_diff || out_index != inner_goal.len() {
                    a_value += NUM_THREADS;
                } else {
                    inner_found.store(true, std::sync::atomic::Ordering::Relaxed);
                    println!("Found a value: {a_value}");
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn execute_instruction(instruction: &Instruction, state: &mut State) -> Option<u8> {
    state.ip += 1;

    match instruction {
        Instruction::ADV(op) => state.a /= 2_u64.pow(op.evaluate(state).try_into().unwrap()),
        Instruction::BDV(op) => {
            state.b = state.a / 2_u64.pow(op.evaluate(state).try_into().unwrap())
        }
        Instruction::CDV(op) => {
            state.c = state.a / 2_u64.pow(op.evaluate(state).try_into().unwrap())
        }

        Instruction::BXL(op) => state.b ^= op.evaluate(state),
        Instruction::BST(op) => state.b = op.evaluate(state) % 8,
        Instruction::JNZ(op) => {
            if state.a > 0 {
                // my current implementation does not work if we jump to an odd address!
                assert!(op.evaluate(state) % 2 == 0);
                state.ip = usize::try_from(op.evaluate(state)).unwrap() / 2;
            }
        }
        Instruction::BXC(_op) => state.b ^= state.c,
        Instruction::OUT(op) => return Some((op.evaluate(state) % 8).try_into().unwrap()),
    }

    None
}

fn part_two(data: &Data) {
    let goal: Arc<Vec<u8>> = Arc::new(data.program.iter().flat_map(|i| i.bytecode()).collect());

    if let Some(a) = solve(&data.inital_state, &data.program, &goal, 0, goal.len() - 1) {
        println!("Found solution {a}");
    } else {
        println!("Not possible");
    }
}

fn solve(
    initial_state: &State,
    program: &[Instruction],
    goal: &[u8],
    a: u64,
    index: usize,
) -> Option<u64> {
    // try 3 bits at a time
    for i in 0..8 {
        let new_a = (a << 3) | i;

        let mut state = initial_state.clone();
        state.a = new_a;

        while let Some(instruction) = program.get(state.ip) {
            if let Some(out) = execute_instruction(instruction, &mut state) {
                if out == goal[index] {
                    if index == 0 {
                        return Some(new_a);
                    }

                    // Next value
                    if let Some(val) = solve(initial_state, program, goal, new_a, index - 1) {
                        return Some(val);
                    }
                }

                break;
            }
        }
    }

    dbg!("Backtracking");

    None
}
