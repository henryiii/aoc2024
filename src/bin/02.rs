/*!
# 2024 Day 2: Red-Nosed Reports
##  Checking lists of numbers

<https://adventofcode.com/2024/day/2>

This validates and counts lists of numbers.
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

fn valid(row: &[i64]) -> bool {
    let range = 1..=3;
    (row.is_sorted() | row.iter().rev().is_sorted())
        & row
            .iter()
            .tuple_windows()
            .map(|(a, b)| range.contains(&(a - b).abs()))
            .all(|x| x)
}

struct Day02 {}

impl Problem for Day02 {
    fn solution_a(input: &str) -> i64 {
        let rows = lists(input);
        rows.iter()
            .map(|row| valid(row))
            .filter(|&x| x)
            .count()
            .try_into()
            .unwrap()
    }

    fn solution_b(input: &str) -> i64 {
        let rows = lists(input);
        rows.iter()
            .map(|row| {
                valid(row)
                    | (0..(row.len()))
                        .map(|i| {
                            let mut nrow = row.clone();
                            nrow.remove(i);
                            valid(&nrow)
                        })
                        .any(|x| x)
            })
            .filter(|&x| x)
            .count()
            .try_into()
            .unwrap()
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
        assert_eq!(Day02::solution_b(INPUT), 4);
    }
}
