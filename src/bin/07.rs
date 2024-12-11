/*!
# 2024 Day 7: Bridge Repair
## Applying operators

<https://adventofcode.com/2024/day/7>

This could be faster (10x?) by avoiding making vectors in the cartesian product.
I had an impl of that but had a bug in IO, and this version was easier for part
2. This one is pretty readable, and still under a second without parallelism.
*/

use itertools::Itertools;

use aoc2024::par::prelude::*;

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

fn compute(vals: &[(u64, Vec<u64>)], ops: &[Ops]) -> u64 {
    vals.par_iter()
        .filter_map(|(val, inst)| {
            let (first, rest) = inst.split_first().unwrap();
            (0..rest.len())
                .map(|_| ops)
                .multi_cartesian_product()
                .map(|ops| {
                    rest.iter()
                        .zip(ops.iter())
                        .fold(*first, |acc, (val, op)| match op {
                            Ops::Add => acc + val,
                            Ops::Mul => acc * val,
                            Ops::Cat => acc * 10u64.pow(val.checked_ilog10().unwrap() + 1) + val,
                        })
                })
                .find(|&x| x == *val)
        })
        .sum()
}

fn solution_a(input: &str) -> u64 {
    let vals = read_data(input);
    compute(&vals, &[Ops::Add, Ops::Mul])
}

fn solution_b(input: &str) -> u64 {
    let vals = read_data(input);
    compute(&vals, &[Ops::Add, Ops::Mul, Ops::Cat])
}

fn main() {
    aoc2024::run("07", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/07.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 3749);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 11387);
    }
}
