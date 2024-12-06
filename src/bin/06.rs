/*!
# 2024 Day 6: Sample
##  Simple template

<https://adventofcode.com/2024/day/6>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use aoc2024::grid::{read_char, Direction, Position};
use grid::Grid;

enum Result {
    Exited(Grid<u8>),
    Cyclic,
}

fn next_step(map: &Grid<char>, pos: Position, dir: Direction) -> Direction {
    let next = pos + dir;
    if map.get(next.row(), next.col()) == Some(&'#') {
        next_step(map, pos, dir.clockwise())
    } else {
        dir
    }
}

fn get_pos(map: &Grid<char>) -> Position {
    map.indexed_iter()
        .find_map(|((x, y), c)| {
            if *c == '^' {
                Some(Position::new(x.try_into().unwrap(), y.try_into().unwrap()))
            } else {
                None
            }
        })
        .unwrap()
}

fn solve(map: &Grid<char>) -> Result {
    let mut pos = get_pos(map);
    let mut visited: Grid<u8> = Grid::new(map.rows(), map.cols());
    let mut dir = Direction::Up;
    while map.get(pos.row(), pos.col()).is_some() {
        visited[pos] |= dir as u8;
        dir = next_step(map, pos, dir);
        pos = pos + dir;
        if let Some(&v) = visited.get(pos.row(), pos.col()) {
            if v & dir as u8 != 0 {
                return Result::Cyclic;
            }
        }
    }
    Result::Exited(visited)
}

struct Day06 {}

impl Problem for Day06 {
    fn solution_a(input: &str) -> i64 {
        let map = read_char(input);
        if let Result::Exited(visited) = solve(&map) {
            visited
                .iter()
                .filter(|&v| *v > 0)
                .count()
                .try_into()
                .unwrap()
        } else {
            panic!("No solution found");
        }
    }

    fn solution_b(input: &str) -> i64 {
        let orig_map = read_char(input);
        let mut map = orig_map.clone();
        orig_map
            .indexed_iter()
            .filter(|(_, &c)| c == '.')
            .filter(|((x, y), _)| {
                map[(*x, *y)] = '#';
                let result = solve(&map);
                map[(*x, *y)] = '.';
                matches!(result, Result::Cyclic)
            })
            .count()
            .try_into()
            .unwrap()
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
        assert_eq!(Day06::solution_b(INPUT), 6);
    }
}