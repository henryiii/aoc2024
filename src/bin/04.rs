#![allow(clippy::cast_possible_wrap)]
/*!
# 2024 Day 4: Ceres Search
##  Word search

<https://adventofcode.com/2024/day/4>

This does a few variations on wordsearch.
*/

use aoc2024::grid::{read_char, DIRECTIONS, XDIRECTIONS};
use aoc2024::{run, Problem};
use grid::Grid;

struct Day04 {}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];

fn get_xmas(grid: &Grid<char>, a: i64, b: i64) -> i64 {
    DIRECTIONS
        .iter()
        .filter(|(x, y)| {
            XMAS.iter()
                .enumerate()
                .all(|(i, v)| grid.get(a + *x * (i as i64), b + *y * (i as i64)) == Some(v))
        })
        .count()
        .try_into()
        .unwrap()
}

fn get_mas(grid: &Grid<char>, a: i64, b: i64) -> i64 {
    XDIRECTIONS
        .iter()
        .filter(|(x, y)| {
            MAS.iter()
                .enumerate()
                .all(|(i, v)| grid.get(a + *x * (i as i64 - 1), b + *y * (i as i64 - 1)) == Some(v))
        })
        .count()
        .try_into()
        .unwrap()
}

impl Problem for Day04 {
    fn solution_a(input: &str) -> i64 {
        let grid = read_char(input);
        grid.indexed_iter()
            .map(|((a, b), _)| get_xmas(&grid, a as i64, b as i64))
            .sum()
    }

    fn solution_b(input: &str) -> i64 {
        let grid = read_char(input);
        grid.indexed_iter()
            .filter(|((a, b), _)| get_mas(&grid, *a as i64, *b as i64) == 2)
            .count() as i64
    }
}

fn main() {
    run::<Day04>("04");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/04.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day04::solution_a(INPUT), 18);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day04::solution_b(INPUT), 9);
    }
}
