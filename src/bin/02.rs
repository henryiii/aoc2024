/*!
# 2024 Day 2: Red-Nosed Reports
##  Simple template

<https://adventofcode.com/2024/day/2>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use itertools::Itertools;

fn lists(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub struct Day02 {}

impl Problem for Day02 {
    fn solution_a(input: &str) -> i64 {
        let rows = lists(input);
        let range = 1..=3;
        rows.iter()
            .map(|row| {
                (row.is_sorted() | row.iter().rev().is_sorted())
                    & row
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| range.contains(&(a - b).abs()))
                        .all(|x| x)
            })
            .filter(|&x| x)
            .count()
            .try_into()
            .unwrap()
    }

    fn solution_b(input: &str) -> i64 {
        0
    }
}

fn main() {
    run::<Day02>("02");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/02.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day02::solution_a(INPUT), 2);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day02::solution_b(INPUT), 1);
    }
}
