/*!
# 2024 Day 7: Bridge Repair
## Applying operators

<https://adventofcode.com/2024/day/7>

This could be faster (10x?) by avoiding making vectors in the cartesian product.
I had an impl of that but had a bug in IO, and this version was easier for part
2. This one is pretty readable, and still under a second without parallelism.

Okay, now it's much faster by short circuiting and avoiding the cartesian
product, using recursion instead.
*/

use aoc::par::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Mul,
    Cat,
}

fn read_data(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(val, inst)| {
            (
                val.parse::<u64>().unwrap(),
                inst.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn all_combos(acc: u64, target: u64, rest: &[u64], ops: &[Ops]) -> bool {
    if acc > target {
        return false;
    }
    let (first, rest) = rest.split_first().unwrap();
    for op in ops {
        let next = match *op {
            Ops::Add => acc + first,
            Ops::Mul => acc * first,
            Ops::Cat => acc * 10u64.pow(first.ilog10() + 1) + first,
        };
        if next == target || !rest.is_empty() && all_combos(next, target, rest, ops) {
            return true;
        }
    }
    false
}

fn compute(vals: &[(u64, Vec<u64>)], ops: &[Ops]) -> u64 {
    vals.par_iter()
        .filter_map(|(target, inst)| {
            let (first, rest) = inst.split_first().unwrap();
            all_combos(*first, *target, rest, ops).then_some(*target)
        })
        .sum()
}

pub fn solution_a(input: &str) -> u64 {
    let vals = read_data(input);
    compute(&vals, &[Ops::Add, Ops::Mul])
}

pub fn solution_b(input: &str) -> u64 {
    let vals = read_data(input);
    compute(&vals, &[Ops::Add, Ops::Mul, Ops::Cat])
}

pub fn main(_: bool) {
    aoc::run("07", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/07.txt", 3749);
    aoc::make_test!("b", "2024/07.txt", 11387);
}
