/*!
# 2024 Day 7: Bridge Repair
## Applying operators

<https://adventofcode.com/2024/day/7>

This could be faster (10x?) by avoiding making vectors in the cartesian product.
I had an impl of that but had a bug in IO, and this version was easier for part
2. This one is pretty readable, and still under a second without parallelism.
*/

use aoc2024::par::prelude::*;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

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
            let num_combos = ops.len().pow(rest.len().try_into().unwrap());
            (0..num_combos)
                .map(|opt_int| {
                    rest.iter()
                        .zip(0..)
                        .fold_while(*first, |acc, (x, i)| {
                            let sum = match ops[(opt_int / ops.len().pow(i)) % ops.len()] {
                                Ops::Add => acc + x,
                                Ops::Mul => acc * x,
                                Ops::Cat => acc * 10u64.pow(x.ilog10() + 1) + x,
                            };
                            if sum <= *val {
                                Continue(sum)
                            } else {
                                Done(sum)
                            }
                        })
                        .into_inner()
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
