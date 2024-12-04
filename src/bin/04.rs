#![allow(clippy::cast_possible_wrap)]
/*!
# 2024 Day 4: Sample
##  Simple template

<https://adventofcode.com/2024/day/4>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use grid::Grid;

fn read_input(input: &str) -> Grid<char> {
    let inp: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::<char>::from(inp)
}

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const XDIRECTIONS: [(i64, i64); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

struct Day04 {}

impl Problem for Day04 {
    fn solution_a(input: &str) -> i64 {
        let mut word: [Option<char>; 4] = [None, None, None, None];
        let answer = [Some('X'), Some('M'), Some('A'), Some('S')];
        let grid = read_input(input);
        let mut total = 0;
        for a in 0..(grid.size().0) {
            for b in 0..(grid.size().1) {
                for (x, y) in &DIRECTIONS {
                    #[allow(clippy::needless_range_loop)]
                    for i in 0..4 {
                        word[i] = grid
                            .get(a as i64 + *x * i as i64, b as i64 + *y * i as i64)
                            .copied();
                    }
                    if word == answer {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    fn solution_b(input: &str) -> i64 {
        let mut word: [Option<char>; 3] = [None, None, None];
        let answer = [Some('M'), Some('A'), Some('S')];
        let grid = read_input(input);
        let mut total = 0;
        for a in 0..(grid.size().0 as i64) {
            for b in 0..(grid.size().1 as i64) {
                let mut finds = 0;
                for (x, y) in &XDIRECTIONS {
                    #[allow(clippy::needless_range_loop)]
                    for i in 0..3 {
                        word[i] = grid
                            .get(a + *x * (i as i64 - 1), b + *y * (i as i64 - 1))
                            .copied();
                    }
                    if word == answer {
                        finds += 1;
                    }
                }
                if finds == 2 {
                    total += 1;
                }
            }
        }
        total
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
