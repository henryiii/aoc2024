/*!
# 2024 Day 3: Mull It Over
##  Picking out simple instructions

<https://adventofcode.com/2024/day/3>

This has us find and run very simple instructions embedded in junk. The second
part has state.
*/

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
    aoc2024::run("03", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc2024::make_test!("a", "03.txt", 161);
    aoc2024::make_test!("b", "03b.txt", 48);
}
