/*!
# 2024 Day 8: Resonant Collinearity
## Map frequencies

<https://adventofcode.com/2024/day/8>

Frequencies on a map.
*/

use std::collections::HashSet;
use std::ops::Range;

use aoc2024::grid::read_char;
use aoc2024::{run, Problem};
use grid::Grid;
use itertools::Itertools;

struct Day08 {}

fn solution(map: &Grid<char>, range: Range<i64>) -> i64 {
    let mut antinodes: Grid<bool> = Grid::new(map.rows(), map.cols());

    // Every char that is a node
    let chars: HashSet<char> = map.iter().filter(|&v| *v != '.').copied().collect();
    for c in chars {
        // Every node of a char
        let locs: Vec<(i64, i64)> = map
            .indexed_iter()
            .filter(|(_, &ch)| ch == c)
            .map(|((x, y), _)| (x.try_into().unwrap(), y.try_into().unwrap()))
            .collect();

        // Every pair of nodes
        locs.iter().combinations(2).for_each(|v| {
            let (&a, &b) = v.iter().collect_tuple().unwrap();
            let dir = (b.0 - a.0, b.1 - a.1);
            for n in range.clone() {
                let pos1 = (a.0 - dir.0 * n, a.1 - dir.1 * n);
                let pos2 = (b.0 + dir.0 * n, b.1 + dir.1 * n);
                if map.get(pos1.0, pos1.1).is_some() {
                    antinodes[(pos1.0.try_into().unwrap(), pos1.1.try_into().unwrap())] = true;
                }
                if map.get(pos2.0, pos2.1).is_some() {
                    antinodes[(pos2.0.try_into().unwrap(), pos2.1.try_into().unwrap())] = true;
                }
            }
        });
    }
    antinodes.iter().filter(|&v| *v).count().try_into().unwrap()
}

impl Problem for Day08 {
    fn solution_a(input: &str) -> i64 {
        let map = read_char(input);
        solution(&map, 1..2)
    }

    fn solution_b(input: &str) -> i64 {
        let map = read_char(input);
        solution(&map, 0..(map.cols().max(map.rows()).try_into().unwrap()))
    }
}

fn main() {
    run::<Day08>("08");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/08.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day08::solution_a(INPUT), 14);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day08::solution_b(INPUT), 34);
    }
}
