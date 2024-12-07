/*!
# 2024 Day 7: Sample
##  Simple template

<https://adventofcode.com/2024/day/7>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use itertools::Itertools;

// 12830733360047 is too low
// 12830757847983 is also too low
struct Day07 {}

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Mul,
}

impl Problem for Day07 {
    fn solution_a(input: &str) -> i64 {
        let vals: Vec<(u64, Vec<u64>)> = input
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
            .collect();
        vals.iter()
            .filter_map(|(val, inst)| {
                let (first, rest) = inst.split_first().unwrap();
                (0..rest.len())
                    .map(|_| [Ops::Add, Ops::Mul])
                    .multi_cartesian_product()
                    .map(|ops| {
                        rest.iter()
                            .zip(ops.iter())
                            .fold(*first, |acc, (val, op)| match op {
                                Ops::Add => acc + val,
                                Ops::Mul => acc * val,
                            })
                    })
                    .find(|&x| x == *val)
            })
            .sum::<u64>()
            .try_into()
            .unwrap()
    }

    fn solution_b(input: &str) -> i64 {
        0
    }
}

fn main() {
    run::<Day07>("07");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/07.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day07::solution_a(INPUT), 3749);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day07::solution_b(INPUT), 1);
    }
}
