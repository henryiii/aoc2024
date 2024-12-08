/*!
# 2024 Day 2: Red-Nosed Reports
##  Checking lists of numbers

<https://adventofcode.com/2024/day/2>

This validates and counts lists of numbers.
*/

use aoc2024::run;

use itertools::Itertools;

fn lists(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn valid<'a, T: DoubleEndedIterator<Item = &'a u64> + Clone>(row: &T) -> bool {
    let range = 1..=3;
    (row.clone().is_sorted() || row.clone().rev().is_sorted())
        && row
            .clone()
            .tuple_windows()
            .all(|(&a, &b)| range.contains(&a.abs_diff(b)))
}

fn solution_a(input: &str) -> usize {
    let rows = lists(input);
    rows.iter().filter(|&row| valid(&row.iter())).count()
}

fn solution_b(input: &str) -> usize {
    let rows = lists(input);
    rows.iter()
        .filter(|&row| {
            valid(&row.iter())
                || (0..(row.len())).any(|i| {
                    valid(
                        &row.iter()
                            .enumerate()
                            .filter(|(j, _)| i != *j)
                            .map(|(_, x)| x),
                    )
                })
        })
        .count()
}

fn main() {
    run("02", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/02.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 2);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 4);
    }
}
