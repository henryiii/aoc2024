/*!
# 2024 Day 2: Red-Nosed Reports
##  Checking lists of numbers

<https://adventofcode.com/2024/day/2>

This validates and counts lists of numbers.
*/

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

pub fn solution_a(input: &str) -> usize {
    lists(input)
        .into_iter()
        .filter(|row| valid(&row.iter()))
        .count()
}

pub fn solution_b(input: &str) -> usize {
    lists(input)
        .into_iter()
        .filter(|row| {
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

pub fn main(_: bool) {
    aoc::run("02", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/02.txt", 2);
    aoc::make_test!("b", "2024/02.txt", 4);
}
