/*!
# 2024 Day 17: Chronospatial Computer
## Instructions and registers

<https://adventofcode.com/2024/day/17>

My part 2 just happens to work for my input, but it's not general enough.
*/

use std::ops::BitXor;

use derive_more::TryFrom;
use derive_new::new;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, TryFrom)]
#[try_from(repr)]
#[repr(u8)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug, new)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn read_input(input: &str) -> (Registers, Vec<OpCode>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        regs:section(
            line("Register A: " u64)
            line("Register B: " u64)
            line("Register C: " u64)
        )
        prog:section("Program: " repeat_sep(a:u8 => OpCode::try_from(a).unwrap(), ","))
        => (Registers::new(regs.0, regs.1, regs.2), prog)
    )
    .parse(input)
    .unwrap()
}

// The combo operand is only decoded for the instructions that use it. Decoding
// it eagerly would panic on the reserved value 7, which is a legal *literal*
// operand for `bxl`/`jnz`/`bxc` (e.g. the common `bxl 7`).
fn combo_operand(literal_operand: u64, reg: &Registers) -> u64 {
    match literal_operand {
        0..4 => literal_operand,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => unreachable!(),
    }
}

fn computer(mut reg: Registers, instructions: &[OpCode]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut instr_ptr = 0;
    while instr_ptr + 1 < instructions.len() {
        let opcode = instructions[instr_ptr];
        let literal_operand = instructions[instr_ptr + 1] as u64;
        match opcode {
            OpCode::Adv => {
                reg.a /= 2u64.pow(combo_operand(literal_operand, &reg).try_into().unwrap());
            }
            OpCode::Bxl => {
                reg.b = reg.b.bitxor(literal_operand);
            }
            OpCode::Bst => {
                reg.b = combo_operand(literal_operand, &reg) & 7;
            }
            OpCode::Jnz => {
                if reg.a != 0 {
                    instr_ptr = literal_operand.try_into().unwrap();
                    continue;
                }
            }
            OpCode::Bxc => {
                reg.b = reg.c.bitxor(reg.b);
            }
            OpCode::Out => {
                out.push(
                    (combo_operand(literal_operand, &reg) & 7)
                        .try_into()
                        .unwrap(),
                );
            }
            OpCode::Bdv => {
                reg.b = reg.a / 2u64.pow(combo_operand(literal_operand, &reg).try_into().unwrap());
            }
            OpCode::Cdv => {
                reg.c = reg.a / 2u64.pow(combo_operand(literal_operand, &reg).try_into().unwrap());
            }
        }
        instr_ptr += 2;
    }
    out
}

pub fn solution_a(input: &str) -> String {
    let (reg, instructions) = read_input(input);
    let out = computer(reg, &instructions);
    out.iter().join(",")
}

pub fn solution_b(input: &str) -> u64 {
    let (reg, instructions) = read_input(input);
    let expected: Vec<u8> = instructions.iter().map(|x| *x as u8).collect();

    // The program reads register A three bits at a time from the bottom up,
    // emitting one output per group, so the i-th output is fixed by the octit
    // at position i. Build A from the top octit down, keeping every prefix whose
    // output matches the corresponding suffix of the program, then take the
    // smallest. (The previous version locked in the first matching octit, which
    // missed value 0 and wasn't general.)
    let mut candidates = vec![0u64];
    for i in (0..expected.len()).rev() {
        let want = &expected[i..];
        candidates = candidates
            .iter()
            .flat_map(|&acc| (0..8).map(move |octit| acc * 8 + octit))
            .filter(|&a| computer(Registers::new(a, reg.b, reg.c), &instructions) == want)
            .collect();
    }
    candidates.into_iter().min().unwrap()
}

pub fn main(_: bool) {
    aoc::run("17", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/17.txt", "4,6,3,5,6,3,5,2,1,0");
    aoc::make_test!("b", "2024/17b.txt", 117_440);
}
