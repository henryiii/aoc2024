/*!
# 2024 Day 6: Sample
##  Simple template

<https://adventofcode.com/2024/day/6>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use aoc2024::grid::{read_char, Direction, Position};
use grid::Grid;

struct Day06 {}

impl Problem for Day06 {
    fn solution_a(input: &str) -> i64 {
        let map = read_char(input);
        let mut pos = map
            .indexed_iter()
            .find_map(|((x, y), c)| {
                if *c == '^' {
                    Some(Position::new(x.try_into().unwrap(), y.try_into().unwrap()))
                } else {
                    None
                }
            })
            .unwrap();
        let mut visited: Grid<bool> = Grid::new(map.rows(), map.cols());
        let mut dir = Direction::Up;
        while map.get(pos.row(), pos.col()) != None {
            visited[pos] = true;
            let mut next = pos + dir;
            if map.get(next.row(), next.col()) == Some(&'#') {
                dir = dir.clockwise();
                next = pos + dir;
                if map.get(next.row(), next.col()) == Some(&'#') {
                    dir = dir.clockwise();
                    next = pos + dir;
                }
            }
            pos = next;
        }
        visited.iter().filter(|&v| *v).count().try_into().unwrap()
    }

    fn solution_b(input: &str) -> i64 {
        0
    }
}

fn main() {
    run::<Day06>("06");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/06.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day06::solution_a(INPUT), 41);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day06::solution_b(INPUT), 1);
    }
}
