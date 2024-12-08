/*!
# 2024 Day 3: Mull It Over
##  Picking out simple instructions

<https://adventofcode.com/2024/day/3>

This has us find and run very simple instructions embedded in junk. The second
part has state.
*/

use aoc2024::run;

use regex::Regex;

enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

fn solution_a(input: &str) -> u64 {
    let reg = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)").unwrap();
    reg.captures_iter(input)
        .map(|cap| {
            let a: u64 = cap[1].parse().unwrap();
            let b: u64 = cap[2].parse().unwrap();
            a * b
        })
        .sum()
}

fn solution_b(input: &str) -> u64 {
    let reg = Regex::new(r"mul\(([[:digit:]]+),([[:digit:]]+)\)|do\(\)|don't\(\)").unwrap();
    reg.captures_iter(input)
        .map(|cap| match &cap[0] {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => Instruction::Mul(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        })
        .fold((0, true), |(acc, flag), inst| match inst {
            Instruction::Do => (acc, true),
            Instruction::Dont => (acc, false),
            Instruction::Mul(a, b) => (acc + a * b * u64::from(flag), flag),
        })
        .0
}

fn main() {
    run("03", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/03.txt");
    const INPUT2: &str = include_str!("../../samples/03b.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 161);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT2), 48);
    }
}
