use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use adventofcode2024::parsers::{Parser, Parsers};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Wire {
    name: String,
    value: bool,
}

fn parse_wires(inp: &str) -> Vec<Wire> {
    inp.lines()
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<_>>();
            let value: u8 = parts[1].parse().unwrap();
            Wire {
                name: parts[0].to_string(),
                value: match value {
                    0 => false,
                    1 => true,
                    other => panic!("Unexpected wire value {other}"),
                },
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Op::And => lhs && rhs,
            Op::Or => lhs || rhs,
            Op::Xor => lhs ^ rhs,
        }
    }
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            other => panic!("Unexpected operator {other}"),
        })
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Gate {
    lhs: String,
    rhs: String,
    op: Op,
    output: String,
}

fn parse_gates(inp: &str) -> Vec<Gate> {
    let p = Parsers::r("[a-z0-9]{3}")
        .followed_by(" ")
        .and(Parsers::r("AND|XOR|OR"))
        .followed_by(" ")
        .and(Parsers::r("[a-z0-9]{3}"))
        .followed_by(" -> ")
        .and(Parsers::r("[a-z0-9]{3}"));
    inp.lines()
        .map(|l| p.apply(l).unwrap_value())
        .map(|(((lhs, op), rhs), output)| Gate {
            lhs,
            rhs,
            op: op.parse().unwrap(),
            output,
        })
        .collect()
}

fn parse_inputs(inp: &str, gates: &str) -> (Vec<Wire>, Vec<Gate>) {
    (parse_wires(inp), parse_gates(gates))
}

fn wire_input() -> String {
    std::fs::read_to_string("./inputs/day24_input_wires.txt").unwrap()
}

fn gate_input() -> String {
    std::fs::read_to_string("./inputs/day24_gates.txt").unwrap()
}

fn calculate_score(values: &HashMap<String, bool>, scoring_char: char) -> u64 {
    let mut result: u64 = 0;

    for (k, v) in values {
        if k.starts_with(scoring_char) {
            if *v {
                let num: u64 = k[1..].parse().unwrap();
                result |= 1 << num;
            }
        }
    }

    result
}

#[derive(Clone, Copy, Debug)]
struct CycleDetected;

fn propagate_circuit(
    wires: &[Wire],
    gates: &[Gate],
) -> Result<HashMap<String, bool>, CycleDetected> {
    let mut processing_queue = VecDeque::new();
    let mut values: HashMap<String, bool> = HashMap::new();
    processing_queue.extend(gates);
    for wire in wires {
        values.insert(wire.name.clone(), wire.value);
    }

    let mut queue_hashes = HashMap::new();

    while let Some(item) = processing_queue.pop_front() {
        let result = match (values.get(&item.lhs), values.get(&item.rhs)) {
            (Some(l), Some(r)) => item.op.eval(*l, *r),
            _ => {
                processing_queue.push_back(item);
                let mut hasher = rustc_hash::FxHasher::default();
                processing_queue.hash(&mut hasher);
                let queue_hash = hasher.finish();
                if queue_hashes.get(&queue_hash).copied().unwrap_or(0) > 1 {
                    return Err(CycleDetected);
                } else {
                    queue_hashes
                        .entry(queue_hash)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                continue;
            }
        };
        values.insert(item.output.clone(), result);
    }
    Ok(values)
}

fn part1(wires: &str, gates: &str) -> u64 {
    let (wires, gates) = parse_inputs(wires, gates);
    let values = propagate_circuit(&wires, &gates).unwrap();
    calculate_score(&values, 'z')
}

fn part2(wires: &str, gates: &str) -> String {
    let (wires, gates) = parse_inputs(wires, gates);
    let values = propagate_circuit(&wires, &gates).unwrap();
    // This is the base for a .dot graphvis file I'll use to accompany the python script.
    for node in values.keys() {
        println!("{};", node);
    }
    for gate in gates.iter() {
        println!("{} -> {};", gate.lhs, gate.output);
        println!("{} -> {};", gate.rhs, gate.output);
    }
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(&wire_input(), &gate_input()));
    println!("Part 2: {}", part2(&wire_input(), &gate_input()));
}
