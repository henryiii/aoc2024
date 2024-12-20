/*!
# 2024 Day 20: Race Condition
## Shorctutting a maze

<https://adventofcode.com/2024/day/20>

This is pretty trivial for a late puzzle. I'm using a graph for initial setup
mostly because I already have tooling set up. A simple search to make the path
list would be fine too, there's only one unique path through.
*/

use grid::Grid;
use petgraph::{algo::astar, prelude::*};

use aoc::grid::read_char;
use aoc::par::prelude::*;

type Int = usize;

fn make_graph(map: &Grid<char>) -> UnGraphMap<(usize, usize), ()> {
    GraphMap::from_edges(
        map.indexed_iter()
            .filter(|(_, c)| **c == '.' || **c == 'S' || **c == 'E')
            .flat_map(|((y, x), _)| {
                [(0, 1), (1, 0)].iter().filter_map(move |(dy, dx)| {
                    let (ny, nx) = (y + dy, x + dx);
                    if map.get(ny, nx) == Some(&'.')
                        || map.get(ny, nx) == Some(&'S')
                        || map.get(ny, nx) == Some(&'E')
                    {
                        Some(((y, x), (ny, nx)))
                    } else {
                        None
                    }
                })
            }),
    )
}

fn find_costs(path: &[(usize, usize)], cheat: usize, limit: usize) -> usize {
    path[0..path.len() - cheat]
        .par_iter()
        .enumerate()
        .map(|(cur_i, cur_pos)| {
            path[cur_i..]
                .iter()
                .zip(cur_i..)
                .filter_map(move |(pos, i)| {
                    let dist = pos.0.abs_diff(cur_pos.0) + pos.1.abs_diff(cur_pos.1);
                    (dist <= cheat).then_some(i - cur_i - dist)
                })
                .filter(|&dist| dist >= limit)
                .count()
        })
        .sum()
}

fn solution(input: &str, cheat: usize, limit: usize) -> Int {
    let map = read_char(input);
    let graph = make_graph(&map);
    let start = map.indexed_iter().find(|(_, c)| **c == 'S').unwrap().0;
    let end = map.indexed_iter().find(|(_, c)| **c == 'E').unwrap().0;
    let (_, path) = astar::astar(&graph, start, |n| n == end, |_| 1usize, |_| 0).unwrap();
    find_costs(&path, cheat, limit)
}

pub fn solution_a(input: &str) -> Int {
    solution(input, 2, 100)
}

pub fn solution_b(input: &str) -> Int {
    solution(input, 20, 100)
}

pub fn main(_: bool) {
    aoc::run("20", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../../samples/2024/20.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(super::solution(INPUT, 2, 64), 1);
        assert_eq!(super::solution(INPUT, 2, 40), 2);
        assert_eq!(super::solution(INPUT, 2, 38), 3);
        assert_eq!(super::solution(INPUT, 2, 12), 8);
    }
    #[test]
    fn test_part_b() {
        assert_eq!(super::solution(INPUT, 20, 76), 3);
        assert_eq!(super::solution(INPUT, 20, 74), 7);
    }
}
