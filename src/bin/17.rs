/*!
# 2024 Day 17: Sample
## Simple template

<https://adventofcode.com/2024/day/17>

This is a small example to get started, also functions as a template for new days.
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
    a: u32,
    b: u32,
    c: u32,
}

fn read_input(input: &str) -> ((u32, u32, u32), Vec<OpCode>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        section(
            line("Register A: " u32)
            line("Register B: " u32)
            line("Register C: " u32)
        )
        section("Program: " repeat_sep(a:u8 => OpCode::try_from(a).unwrap(), ","))
    )
    .parse(input)
    .unwrap()
}

fn solution_a(input: &str) -> String {
    let (reg, instructions) = read_input(input);
    let mut reg = Registers::new(reg.0, reg.1, reg.2);
    let mut out = Vec::new();
    let mut instr_ptr = 0;
    while instr_ptr + 1 < instructions.len() {
        let opcode = instructions[instr_ptr];
        let literal_operand = instructions[instr_ptr + 1] as u32;
        let combo_operand = match literal_operand {
            0..4 => literal_operand,
            4 => reg.a,
            5 => reg.b,
            6 => reg.c,
            _ => unreachable!(),
        };
        match opcode {
            OpCode::Adv => {
                reg.a /= 2u32.pow(combo_operand);
            }
            OpCode::Bxl => {
                reg.b = reg.b.bitxor(literal_operand);
            }
            OpCode::Bst => {
                reg.b = combo_operand & 7;
            }
            OpCode::Jnz => {
                if reg.a != 0 {
                    instr_ptr = literal_operand as usize;
                    continue;
                }
            }
            OpCode::Bxc => {
                reg.b = reg.c.bitxor(reg.b);
            }
            OpCode::Out => {
                out.push(combo_operand & 7);
            }
            OpCode::Bdv => {
                reg.b = reg.a / 2u32.pow(combo_operand);
            }
            OpCode::Cdv => {
                reg.c = reg.a / 2u32.pow(combo_operand);
            }
        }
        instr_ptr += 2;
    }

    out.iter().join(",")
}

fn solution_b(input: &str) -> String {
    read_input(input);
    String::new()
}

fn main() {
    aoc2024::run("17", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/17.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), "");
    }
}
