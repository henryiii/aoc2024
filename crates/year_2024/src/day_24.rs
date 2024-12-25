/*!
# 2024 Day 24: Crossed Wires
## Digital circuitry

<https://adventofcode.com/2024/day/24>

This is a simple digital circuit.

Part 2 is recrating an adder circuit, so I'm skipping the test, as the same
wasn't an adder circuit.
*/

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Int = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    And,
    Or,
    Xor,
}

#[allow(clippy::type_complexity)]
fn read_input(input: &str) -> (Vec<(String, bool)>, Vec<(String, String, String, String)>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        section(
            lines(
                a:string(alnum+) ": " b:u8 => (a, b == 1)
            )
        )
        section(
            lines(
                string(alnum+) " " string(alpha+) " " string(alnum+) " -> " string(alnum+)
            )
        )
    )
    .parse(input)
    .unwrap()
}

fn lookup_value(
    key: &str,
    cons: &HashMap<&str, (&str, Instruction, &str)>,
    inputs: &[(String, bool)],
) -> bool {
    if let Some((_, val)) = inputs.iter().find(|(k, _)| k == key) {
        return *val;
    }
    let (a, instr, b) = cons[key];
    let a = lookup_value(a, cons, inputs);
    let b = lookup_value(b, cons, inputs);
    match instr {
        Instruction::And => a & b,
        Instruction::Or => a | b,
        Instruction::Xor => a ^ b,
    }
}

fn setup(
    connections: &[(String, String, String, String)],
) -> HashMap<&str, (&str, Instruction, &str)> {
    connections
        .iter()
        .map(|(a, b, c, d)| match &b[..] {
            "AND" => (&d[..], (&a[..], Instruction::And, &c[..])),
            "OR" => (&d[..], (&a[..], Instruction::Or, &c[..])),
            "XOR" => (&d[..], (&a[..], Instruction::Xor, &c[..])),
            _ => unreachable!(),
        })
        .collect()
}

fn compute_inputs(
    cons: &HashMap<&str, (&str, Instruction, &str)>,
    inputs: &[(String, bool)],
) -> Int {
    cons.keys()
        .filter(|k| k.starts_with('z'))
        .sorted()
        .map(|&x| lookup_value(x, cons, inputs))
        .rev()
        .fold(0, |acc, x| acc * 2 + usize::from(x))
}

fn solution_a(input: &str) -> Int {
    let (inputs, connections) = read_input(input);
    let cons = setup(&connections);
    compute_inputs(&cons, &inputs)
}

fn solution_b(input: &str) -> String {
    let (lines, connections) = read_input(input);
    let cons = setup(&connections);
    let max_input = lines
        .iter()
        .map(|(x, _)| x[1..].parse::<usize>().unwrap())
        .max()
        .unwrap();
    let max_output = max_input + 1;
    let mut bad_connections = HashSet::new();
    // Checking rules for a ripple carry adder
    // Every output must be connected via an XOR (except the last one, which is the final carry)
    bad_connections.extend(cons.iter().filter(|&(out, (_, op, _))| {
        out.starts_with('z') && *op != Instruction::Xor && *out != format!("z{max_output}")
    }));
    // Every XOR must connect to an input or an output
    bad_connections.extend(cons.iter().filter(|&(out, (in1, op, in2))| {
        *op == Instruction::Xor
            && !matches!(out.chars().next().unwrap(), 'x' | 'y' | 'z')
            && !matches!(in1.chars().next().unwrap(), 'x' | 'y' | 'z')
            && !matches!(in2.chars().next().unwrap(), 'x' | 'y' | 'z')
    }));
    // Every XOR that is connected to an input can't connect to an output (except the first half adder)
    bad_connections.extend(cons.iter().filter(|&(out, (in1, op, in2))| {
        *op == Instruction::Xor
            && matches!(out.chars().next().unwrap(), 'z')
            && matches!(in1.chars().next().unwrap(), 'x' | 'y')
            && matches!(in2.chars().next().unwrap(), 'x' | 'y')
            && !matches!(*in1, "x00" | "y00")
            && !matches!(*in2, "x00" | "y00")
    }));
    // Make a collection of all OR inputs
    let adder_ors: HashSet<_> = cons
        .iter()
        .filter(|&(_, (_, op, _))| *op == Instruction::Or)
        .flat_map(|(_, (in1, _, in2))| [in1, in2])
        .collect();
    // ANDs can only feed ORs, except for the first half adder
    bad_connections.extend(cons.iter().filter(|&(out, (in1, op, in2))| {
        *op == Instruction::And
            && !matches!(*in1, "x00" | "y00")
            && !matches!(*in2, "x00" | "y00")
            && !adder_ors.contains(out)
    }));

    // Make a collection of all OR inputs
    let all_ors: HashSet<_> = cons
        .iter()
        .filter(|&(_, (_, op, _))| *op == Instruction::Or)
        .flat_map(|(_, (in1, _, in2))| [in1, in2])
        .collect();
    // ORs can only be feed by ANDs
    bad_connections.extend(
        cons.iter()
            .filter(|&(out, (_, op, _))| *op != Instruction::And && all_ors.contains(out)),
    );

    bad_connections
        .into_iter()
        .map(|(out, _)| out)
        .sorted()
        .join(",")
}

pub fn main(_: bool) {
    aoc::run("24", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/24-2.txt", 2024);
}
