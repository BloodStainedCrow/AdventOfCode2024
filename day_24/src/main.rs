use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    mem,
    ops::ControlFlow,
};

use itertools::Itertools;
use logicng::{
    formulas::{EncodedFormula, FormulaFactory},
    operations::predicates::is_tautology,
};

struct Data {
    initial_wires: HashMap<Wire, bool>,
    gates: Vec<Gate>,
}

impl Data {
    fn from_str(input: &str) -> Self {
        let (wires, gates) = input.split_once("\n\n").expect("Could not find empty line");

        let initial_wires = wires
            .lines()
            .map(|line| line.split_once(' ').expect("Malformed wire input"))
            .map(|(wire_name, wire_value)| {
                (
                    wire_from_str(wire_name.strip_suffix(':').unwrap()),
                    wire_value.parse::<u8>().expect("Could not parse bool") > 0,
                )
            })
            .collect();

        let gates = gates
            .lines()
            .map(|line| line.split_ascii_whitespace())
            .map(|mut words| {
                let wire1 = wire_from_str(words.next().unwrap());
                let ty = match words.next().unwrap() {
                    "AND" => GateType::AND {
                        inputs: [wire1, wire_from_str(words.next().unwrap())],
                    },
                    "OR" => GateType::OR {
                        inputs: [wire1, wire_from_str(words.next().unwrap())],
                    },
                    "XOR" => GateType::XOR {
                        inputs: [wire1, wire_from_str(words.next().unwrap())],
                    },
                    _ => unreachable!(),
                };

                assert_eq!(words.next(), Some("->"));

                Gate {
                    ty,
                    output: wire_from_str(words.next().unwrap()),
                }
            })
            .collect();

        Self {
            initial_wires,
            gates,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum GateType {
    AND { inputs: [Wire; 2] },
    OR { inputs: [Wire; 2] },
    XOR { inputs: [Wire; 2] },
}

#[derive(Debug, Clone, PartialEq)]
struct Gate {
    ty: GateType,
    output: Wire,
}

type Wire = String;

fn wire_from_str(input: &str) -> Wire {
    input.to_string()
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
    let mut values = data.initial_wires.clone();

    partial_evaluate(&data.gates, &mut values);

    let mut place = 0;
    let mut final_value = 0;

    dbg!(&values);

    while let Some(val) = values.get(&format!("z{place:02}")) {
        final_value |= u64::from(*val) << place;
        place += 1;
    }

    dbg!(final_value);
}

fn part_two(data: &Data) {
    let ret = solve(data);

    let mut res = vec![];
    for (before, after) in data.gates.iter().zip(ret) {
        if before.output != after.output {
            res.push(after.output);
        }
    }

    res.sort();

    println!("{}", res.iter().join(","))
}

#[derive(Debug)]
enum CheckFailureType {
    S(u64),
    Loop,
}

fn check_is_adder(gates: &[Gate], num_input_digits: usize) -> Result<(), CheckFailureType> {
    let fac = FormulaFactory::new();
    let mut ret = 0;

    for place in 0..num_input_digits {
        let s = format!("z{place:02}");

        let Ok(formula) = formula_for_wire(gates, &s, &fac, &Box::new(&|_| false)) else {
            return Err(CheckFailureType::Loop);
        };

        let correct = adder_for_place(&fac, place);

        let equiv = fac.equivalence(formula, correct);

        if !is_tautology(equiv, &fac) {
            ret |= 1 << place;
        }
    }

    if ret != 0 {
        Err(CheckFailureType::S(ret))
    } else {
        Ok(())
    }
}

fn formula_for_wire(
    gates: &[Gate],
    wire: &Wire,
    factory: &FormulaFactory,
    abort: &Box<&dyn Fn(&Wire) -> bool>,
) -> Result<EncodedFormula, ()> {
    if abort(wire) {
        return Err(());
    }

    Ok(match gates.iter().find(|g| g.output == *wire) {
        Some(gate) => match &gate.ty {
            GateType::AND { inputs } => factory.and(&[
                formula_for_wire(
                    gates,
                    &inputs[0],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
                formula_for_wire(
                    gates,
                    &inputs[1],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
            ]),
            GateType::OR { inputs } => factory.or(&[
                formula_for_wire(
                    gates,
                    &inputs[0],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
                formula_for_wire(
                    gates,
                    &inputs[1],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
            ]),
            GateType::XOR { inputs } => xor(
                factory,
                formula_for_wire(
                    gates,
                    &inputs[0],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
                formula_for_wire(
                    gates,
                    &inputs[1],
                    factory,
                    &Box::new(&move |lower_wire: &String| lower_wire == wire || abort(lower_wire)),
                )?,
            ),
        },
        None => factory.variable(wire),
    })
}

fn adder_for_place(factory: &FormulaFactory, place: usize) -> EncodedFormula {
    if place > 0 {
        xor(
            factory,
            factory.variable(&format!("x{place:02}")),
            xor(
                factory,
                factory.variable(&format!("y{place:02}")),
                cout_for_place(factory, place - 1),
            ),
        )
    } else {
        xor(
            factory,
            factory.variable(&format!("x{place:02}")),
            factory.variable(&format!("y{place:02}")),
        )
    }
}

fn cout_for_place(factory: &FormulaFactory, place: usize) -> EncodedFormula {
    factory.or(&[
        factory.and(&[
            factory.variable(&format!("x{place:02}")),
            factory.variable(&format!("y{place:02}")),
        ]),
        factory.and(&[
            {
                if place == 0 {
                    factory.constant(false)
                } else {
                    cout_for_place(factory, place - 1)
                }
            },
            xor(
                factory,
                factory.variable(&format!("x{place:02}")),
                factory.variable(&format!("y{place:02}")),
            ),
        ]),
    ])
}

fn xor(factory: &FormulaFactory, a: EncodedFormula, b: EncodedFormula) -> EncodedFormula {
    factory.and(&[factory.or(&[a, b]), factory.not(factory.and(&[a, b]))])
}

fn partial_evaluate(gates: &[Gate], state: &mut HashMap<Wire, bool>) {
    let mut gates = gates.to_vec();
    let mut old_len = 0;

    while !gates.is_empty() && old_len != gates.len() {
        old_len = gates.len();
        gates.retain(|gate| {
            match &gate.ty {
                GateType::AND { inputs } => {
                    if let Some(w1) = state.get(&inputs[0]) {
                        if let Some(w2) = state.get(&inputs[1]) {
                            state.insert(gate.output.clone(), *w1 && *w2);
                            return false;
                        }
                    }
                }
                GateType::OR { inputs } => {
                    if let Some(w1) = state.get(&inputs[0]) {
                        if let Some(w2) = state.get(&inputs[1]) {
                            state.insert(gate.output.clone(), *w1 || *w2);
                            return false;
                        }
                    }
                }
                GateType::XOR { inputs } => {
                    if let Some(w1) = state.get(&inputs[0]) {
                        if let Some(w2) = state.get(&inputs[1]) {
                            state.insert(gate.output.clone(), *w1 ^ *w2);
                            return false;
                        }
                    }
                }
            }
            true
        });
    }
}

fn get_influencers(gates: &[Gate], wire: &Wire, include_finals: bool) -> HashSet<Wire> {
    if let Some(gate) = gates.iter().find(|gate| gate.output == *wire) {
        match &gate.ty {
            GateType::AND { inputs } | GateType::XOR { inputs } | GateType::OR { inputs } => {
                let mut ret = get_influencers(gates, &inputs[0], include_finals);
                ret.extend(get_influencers(gates, &inputs[1], include_finals));
                ret.insert(wire.clone());
                ret
            }
        }
    } else if include_finals {
        HashSet::from([wire.clone()])
    } else {
        HashSet::new()
    }
}

fn solve(data: &Data) -> Vec<Gate> {
    let mut num_digits_input = 0;

    while data
        .initial_wires
        .contains_key(&format!("x{num_digits_input:02}"))
    {
        num_digits_input += 1;
    }

    let mut gates = data.gates.clone();

    let mut problems: u64 = (0..num_digits_input).fold(0, |acc, _| (acc << 1) | 1);

    let mut locked_gates: Vec<bool> = vec![false; gates.len()];

    'next_problem: while problems != 0 {
        let res = check_is_adder(&gates, num_digits_input);
        //let res = check_is_ander(&gates, num_digits_input);

        if let Err(err) = res {
            // Do a change

            let problems_left = match err {
                CheckFailureType::S(i) => i,
                CheckFailureType::Loop => unreachable!(),
            };

            println!("{problems_left:b}");

            assert!(problems_left.count_ones() <= problems.count_ones());

            for old_prob in 0..num_digits_input {
                if problems & 1 << old_prob == 0 {
                    continue;
                }

                if (problems_left & (1 << old_prob)) > 0 {
                    continue;
                }

                println!("Locking {old_prob}");
                let output = format!("z{old_prob:02}");
                // these are newly okay
                let influencers = get_influencers(&gates, &output, false);

                for inf in influencers {
                    let idx = gates
                        .iter()
                        .position(|g| g.output == *inf)
                        .expect("Influencer should be the output of a gate");
                    locked_gates[idx] = true;
                }
            }

            problems = problems_left;
        } else {
            break;
        }

        // try and fix the problem
        if problems == 0 {
            break;
        }

        let output = format!("z{:02}", problems.trailing_zeros());
        let current_influencers = get_influencers(&gates, &output, false);

        dbg!(&current_influencers);
        dbg!(&locked_gates);

        for num_swaps in 1..=2 {
            println!("Trying {num_swaps} swaps");
            let ret = try_out_swaps(
                &mut gates,
                &current_influencers,
                &locked_gates,
                num_digits_input,
                problems.trailing_zeros().try_into().unwrap(),
                num_swaps,
                |gates, num_digits_input, _| {
                    // Try if it is better
                    let check = check_is_adder(gates, num_digits_input);

                    match check {
                        Ok(_) => ControlFlow::Break(()),
                        Err(err) => match err {
                            CheckFailureType::S(new_probs) => {
                                if new_probs.count_ones() < problems.count_ones() {
                                    ControlFlow::Break(())
                                } else {
                                    ControlFlow::Continue(())
                                }
                            }
                            CheckFailureType::Loop => ControlFlow::Continue(()),
                        },
                    }
                },
            );

            dbg!(ret);

            match ret {
                ControlFlow::Continue(_) => continue,
                ControlFlow::Break(_) => continue 'next_problem,
            }
        }

        todo!("No solution with x swaps");
    }

    assert!(check_is_adder(&gates, num_digits_input).is_ok());

    gates
}

fn try_out_swaps(
    gates: &mut [Gate],
    current_influencers: &HashSet<Wire>,
    locked_gates: &[bool],
    num_digits_input: usize,
    current_problem: usize,
    num_swaps: usize,
    check: impl Fn(&[Gate], usize, usize) -> ControlFlow<()> + Copy,
) -> ControlFlow<()> {
    if num_swaps == 0 {
        return check(gates, num_digits_input, current_problem);
    }

    let start = gates.to_vec();

    let num_gates = gates.len();

    for influencer in current_influencers {
        let a_idx = gates
            .iter()
            .position(|g| g.output == *influencer)
            .expect("Influencer should be in gates");

        if locked_gates[a_idx] {
            continue;
        }

        dbg!(influencer);
        for j in 0..num_gates {
            if locked_gates[j] {
                continue;
            }

            if a_idx == j {
                continue;
            }

            dbg!(j);

            // Swap them

            if a_idx < j {
                let (a, b) = gates.split_at_mut(j);
                mem::swap(&mut a[a_idx].output, &mut b[0].output);
            } else {
                let (a, b) = gates.split_at_mut(a_idx);
                mem::swap(&mut a[j].output, &mut b[0].output);
            }

            try_out_swaps(
                gates,
                current_influencers,
                locked_gates,
                num_digits_input,
                current_problem,
                num_swaps - 1,
                check,
            )?;

            // Swap back
            if a_idx < j {
                let (a, b) = gates.split_at_mut(j);
                mem::swap(&mut a[a_idx].output, &mut b[0].output);
            } else {
                let (a, b) = gates.split_at_mut(a_idx);
                mem::swap(&mut a[j].output, &mut b[0].output);
            }

            debug_assert_eq!(start, gates);
        }
    }

    debug_assert_eq!(start, gates);

    ControlFlow::Continue(())
}
