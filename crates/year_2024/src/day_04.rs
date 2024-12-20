/*!
# 2024 Day 4: Ceres Search
##  Word search

<https://adventofcode.com/2024/day/4>

This does a few variations on wordsearch.
*/

use grid::Grid;

use aoc::grid::{read_char, DIRECTIONS, XDIRECTIONS};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];

fn get_xmas(grid: &Grid<char>, a: i64, b: i64) -> usize {
    DIRECTIONS
        .iter()
        .filter(|(x, y)| {
            XMAS.iter()
                .zip(0_i64..)
                .all(|(v, i)| grid.get(a + *x * i, b + *y * i) == Some(v))
        })
        .count()
}

fn get_mas(grid: &Grid<char>, a: i64, b: i64) -> usize {
    XDIRECTIONS
        .iter()
        .filter(|(x, y)| {
            MAS.iter()
                .zip(0_i64..)
                .all(|(v, i)| grid.get(a + *x * (i - 1), b + *y * (i - 1)) == Some(v))
        })
        .count()
}

fn solution_a(input: &str) -> usize {
    let grid = read_char(input);
    grid.indexed_iter()
        .map(|((a, b), _)| get_xmas(&grid, a.try_into().unwrap(), b.try_into().unwrap()))
        .sum()
}

fn solution_b(input: &str) -> usize {
    let grid = read_char(input);
    grid.indexed_iter()
        .filter(|((a, b), _)| {
            get_mas(&grid, (*a).try_into().unwrap(), (*b).try_into().unwrap()) == 2
        })
        .count()
}

pub fn main(_: bool) {
    aoc::run("04", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/04.txt", 18);
    aoc::make_test!("b", "2024/04.txt", 9);
}
