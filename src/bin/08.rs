/*!
# 2024 Day 8: Sample
##  Simple template

<https://adventofcode.com/2024/day/8>

This is a small example to get started, also functions as a template for new days.
*/

use std::collections::HashSet;

use aoc2024::grid::read_char;
use aoc2024::{run, Problem};
use grid::Grid;
use itertools::Itertools;

struct Day08 {}

impl Problem for Day08 {
    fn solution_a(input: &str) -> i64 {
        let map = read_char(input);
        let mut antinodes: Grid<bool> = Grid::new(map.rows(), map.cols());
        let chars: HashSet<char> =
            HashSet::from_iter(map.iter().filter(|&v| *v != '.').map(|&v| v));
        for c in chars {
            let locs: Vec<_> = map
                .indexed_iter()
                .filter(|(_, &ch)| ch == c)
                .map(|((x, y), _)| (x as i64, y as i64))
                .collect();
            locs.iter()
                .cartesian_product(locs.iter())
                .for_each(|(&a, &b)| {
                    if a != b {
                        let dir = (b.0 - a.0, b.1 - a.1);
                        let pos1 = (a.0 - dir.0, a.1 - dir.1);
                        let pos2 = (b.0 + dir.0, b.1 + dir.1);
                        if map.get(pos1.0, pos1.1).is_some() {
                            antinodes[(pos1.0 as usize, pos1.1 as usize)] = true;
                        }
                        if map.get(pos2.0, pos2.1).is_some() {
                            antinodes[(pos2.0 as usize, pos2.1 as usize)] = true;
                        }
                    }
                });
        }
        #[cfg(test)]
        {
            let mut newmap = map.clone();
            newmap.iter_mut().zip(antinodes.iter()).for_each(|(v, &a)| {
                if a {
                    *v = '#';
                }
            });
            dbg!(newmap);
        }
        antinodes.iter().filter(|&v| *v).count().try_into().unwrap()
    }

    fn solution_b(input: &str) -> i64 {
        let map = read_char(input);
        let mut antinodes: Grid<bool> = Grid::new(map.rows(), map.cols());
        let chars: HashSet<char> =
            HashSet::from_iter(map.iter().filter(|&v| *v != '.').map(|&v| v));
        for c in chars {
            let locs: Vec<_> = map
                .indexed_iter()
                .filter(|(_, &ch)| ch == c)
                .map(|((x, y), _)| (x as i64, y as i64))
                .collect();
            locs.iter()
                .cartesian_product(locs.iter())
                .for_each(|(&a, &b)| {
                    if a != b {
                        let dir = (b.0 - a.0, b.1 - a.1);
                        let max_n  = map.cols().max(map.rows()) as i64;
                        for n in 0..max_n {
                            let pos1 = (a.0 - dir.0*n, a.1 - dir.1*n);
                            let pos2 = (b.0 + dir.0*n, b.1 + dir.1*n);
                            if map.get(pos1.0, pos1.1).is_some() {
                                antinodes[(pos1.0 as usize, pos1.1 as usize)] = true;
                            }
                            if map.get(pos2.0, pos2.1).is_some() {
                                antinodes[(pos2.0 as usize, pos2.1 as usize)] = true;
                            }
                        }
                    }
                });
        }
        #[cfg(test)]
        {
            let mut newmap = map.clone();
            newmap.iter_mut().zip(antinodes.iter()).for_each(|(v, &a)| {
                if a {
                    *v = '#';
                }
            });
            dbg!(newmap);
        }
        antinodes.iter().filter(|&v| *v).count().try_into().unwrap()
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
