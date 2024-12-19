/*!
# 2024 Day 10: Hoof It
## Increasing paths on a grid

<https://adventofcode.com/2024/day/10>

This is a simple number of paths on a grid problem.
*/

use std::collections::HashSet;

use grid::Grid;
use strum::IntoEnumIterator;

use aoc2024::grid::{read_int, Direction};

fn find_starts(map: &Grid<u32>) -> Vec<(i64, i64)> {
    map.indexed_iter()
        .filter(|(_, &i)| i == 0)
        .map(|((x, y), _)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect()
}

fn find_path(map: &Grid<u32>, start_pos: (i64, i64)) -> Vec<(i64, i64)> {
    // Could be a number from 0 to 8.
    let start_val = *map.get(start_pos.0, start_pos.1).unwrap();
    Direction::iter()
        .flat_map(|dir| {
            let pos = start_pos + dir;
            if let Some(&val) = map.get(pos.0, pos.1) {
                if val == start_val + 1 {
                    if val == 9 {
                        return vec![pos];
                    }
                    return find_path(map, pos);
                }
            }
            vec![]
        })
        .collect()
}

fn solution_a(input: &str) -> usize {
    let map = read_int(input);
    let starts = find_starts(&map);
    starts
        .into_iter()
        .map(|pos| HashSet::<_>::from_iter(find_path(&map, pos)).len())
        .sum()
}

fn solution_b(input: &str) -> usize {
    let map = read_int(input);
    let starts = find_starts(&map);
    starts
        .into_iter()
        .map(|pos| find_path(&map, pos).len())
        .sum()
}

fn main() {
    aoc2024::run("10", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc2024::make_test!("a", "10.txt", 36);
    aoc2024::make_test!("b", "10.txt", 81);
}
