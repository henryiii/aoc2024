/*!
# 2024 Day 25: Sample
## Simple template

<https://adventofcode.com/2024/day/25>

This is a small example to get started, also functions as a template for new days.
*/

type Int = usize;

use aoc::grid::read_char;
use grid::Grid;

enum LockKey {
    Lock([usize; 5]),
    Key([usize; 5]),
}

fn read_input(input: &str) -> Vec<Grid<char>> {
    use aoc_parse::{parser, prelude::*};

    let sections = parser!(sections(string(any_char+))).parse(input).unwrap();

    sections.into_iter().map(|s| read_char(&s)).collect()
}

fn convert_input(input: &Grid<char>) -> LockKey {
    let vals: Vec<_> = input
        .iter_cols()
        .map(|col| col.filter(|&c| *c == '#').count() - 1)
        .collect();
    if input[(0, 0)] == '#' {
        LockKey::Lock(vals.try_into().unwrap())
    } else {
        LockKey::Key(vals.try_into().unwrap())
    }
}

fn solution_a(input: &str) -> Int {
    let input = read_input(input);
    let locks_and_keys: Vec<_> = input.iter().map(convert_input).collect();
    let locks: Vec<_> = locks_and_keys
        .iter()
        .filter_map(|lk| {
            if let LockKey::Lock(loc) = lk {
                Some(loc)
            } else {
                None
            }
        })
        .collect();
    let keys: Vec<_> = locks_and_keys
        .iter()
        .filter_map(|lk| {
            if let LockKey::Key(loc) = lk {
                Some(loc)
            } else {
                None
            }
        })
        .collect();
    locks
        .iter()
        .map(|&lock| {
            keys.iter()
                .filter(|&key| lock.iter().zip(key.iter()).all(|(l, k)| *k + *l <= 5))
                .count()
        })
        .sum()
}

const fn solution_b(_: &str) -> Int {
    0
}

pub fn main(_: bool) {
    aoc::run("25", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/25.txt", 3);
    aoc::make_test!("b", "2024/25.txt", 0);
}
