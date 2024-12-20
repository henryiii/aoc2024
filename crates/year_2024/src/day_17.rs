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

fn read_input(input: &str) -> ((u64, u64, u64), Vec<OpCode>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        section(
            line("Register A: " u64)
            line("Register B: " u64)
            line("Register C: " u64)
        )
        section("Program: " repeat_sep(a:u8 => OpCode::try_from(a).unwrap(), ","))
    )
    .parse(input)
    .unwrap()
}

fn computer(mut reg: Registers, instructions: &[OpCode]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut instr_ptr = 0;
    while instr_ptr + 1 < instructions.len() {
        let opcode = instructions[instr_ptr];
        let literal_operand = instructions[instr_ptr + 1] as u64;
        let combo_operand = match literal_operand {
            0..4 => literal_operand,
            4 => reg.a,
            5 => reg.b,
            6 => reg.c,
            _ => unreachable!(),
        };
        match opcode {
            OpCode::Adv => {
                reg.a /= 2u64.pow(combo_operand.try_into().unwrap());
            }
            OpCode::Bxl => {
                reg.b = reg.b.bitxor(literal_operand);
            }
            OpCode::Bst => {
                reg.b = combo_operand & 7;
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
                out.push((combo_operand & 7).try_into().unwrap());
            }
            OpCode::Bdv => {
                reg.b = reg.a / 2u64.pow(combo_operand.try_into().unwrap());
            }
            OpCode::Cdv => {
                reg.c = reg.a / 2u64.pow(combo_operand.try_into().unwrap());
            }
        }
        instr_ptr += 2;
    }
    out
}

fn solution_a(input: &str) -> String {
    let (reg, instructions) = read_input(input);
    let reg = Registers::new(reg.0, reg.1, reg.2);
    let out = computer(reg, &instructions);
    out.iter().join(",")
}

fn solution_b(input: &str) -> u64 {
    let (reg, instructions) = read_input(input);
    let expected_out = instructions.iter().map(|x| *x as u8).collect_vec();
    let size: u32 = expected_out.len().try_into().unwrap();

    // Shortcut for simple runs
    if size < 7 {
        return (0..8u64.pow(size))
            .find(|val| computer(Registers::new(*val, reg.1, reg.2), &instructions) == expected_out)
            .unwrap();
    }

    // Try to find the value, assuming each value is locked in place one it prints
    let mut val = 0;
    for i in 0..expected_out.len() {
        val *= 8;
        for _ in 0..8 {
            val += 1;
            let reg = Registers::new(val, reg.1, reg.2);
            let out = computer(reg, &instructions);
            if *out.first().unwrap() == expected_out[expected_out.len() - 1 - i] {
                break;
            }
        }
    }
    // The assumption above is too strict for the final value due to the two
    // 8-bit registers
    for i in (val - 0o77)..=(val + 0o77) {
        let reg = Registers::new(i, reg.1, reg.2);
        let out = computer(reg, &instructions);
        if out == expected_out {
            return i;
        }
    }
    unreachable!();
}

pub fn main(_: bool) {
    aoc::run("17", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/17.txt", "4,6,3,5,6,3,5,2,1,0");
    aoc::make_test!("b", "2024/17b.txt", 117_440);
}
