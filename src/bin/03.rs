/*!
# 2024 Day 3: Mull It Over
##  Picking out simple instructions

<https://adventofcode.com/2024/day/3>

This has us find and run very simple instructions embedded in junk. The second
part has state.
*/

use aoc2024::{run, Problem};

use regex::Regex;

enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

struct Day03 {}

impl Problem for Day03 {
    fn solution_a(input: &str) -> i64 {
        let reg = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)").unwrap();
        reg.captures_iter(input)
            .map(|cap| {
                let a = cap[1].parse::<i64>().unwrap();
                let b = cap[2].parse::<i64>().unwrap();
                a * b
            })
            .sum()
    }

    fn solution_b(input: &str) -> i64 {
        let reg = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)|do\(\)|don't\(\)").unwrap();
        let instructions = reg.captures_iter(input).map(|cap| match &cap[0] {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => {
                let a = cap[1].parse::<i64>().unwrap();
                let b = cap[2].parse::<i64>().unwrap();
                Instruction::Mul(a, b)
            }
        });
        instructions
            .fold((0, true), |(acc, flag), inst| match inst {
                Instruction::Do => (acc, true),
                Instruction::Dont => (acc, false),
                Instruction::Mul(a, b) => (acc + a * b * i64::from(flag), flag),
            })
            .0
    }
}

fn main() {
    run::<Day03>("03");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/03.txt");
    const INPUT2: &str = include_str!("../../samples/03b.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day03::solution_a(INPUT), 161);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day03::solution_b(INPUT2), 48);
    }
}
