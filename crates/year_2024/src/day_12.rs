/*!
# 2024 Day 12: Garden Groups
## Finding regions

<https://adventofcode.com/2024/day/12>

Building up `Vec`'s, then converting to a `HashSet` was faster than building
`HashSet`.

The key here is that an "edge" is a position plus the direction from "filled" square
to unfilled square (surface normal, basically).
*/

use std::collections::HashSet;

use grid::Grid;
use strum::IntoEnumIterator;

use aoc::grid::{Direction, read_char};

fn find_region(seen: &mut Grid<bool>, map: &Grid<char>, start: (i64, i64)) -> Vec<(i64, i64)> {
    let ch = map.get(start.0, start.1).unwrap();
    let mut region = Vec::new();
    let mut stack = vec![start];
    *seen.get_mut(start.0, start.1).unwrap() = true;
    while let Some(pos) = stack.pop() {
        region.push(pos);
        for dir in Direction::iter() {
            let next = pos + dir;
            if seen.get(next.0, next.1) == Some(&false) && map.get(next.0, next.1) == Some(ch) {
                *seen.get_mut(next.0, next.1).unwrap() = true;
                stack.push(next);
            }
        }
    }
    region
}

fn get_edges(
    region: &'_ HashSet<(i64, i64)>,
) -> impl Iterator<Item = (Direction, i64, i64)> + use<'_> {
    region.iter().flat_map(|(x, y)| {
        Direction::iter()
            .filter(|dir| !region.contains(&((*x, *y) + *dir)))
            .map(|dir| (dir, *x, *y))
    })
}

fn remove_contiguous(sides: &mut HashSet<(Direction, i64, i64)>, start: (Direction, i64, i64)) {
    let mut stack = vec![start];
    while let Some(edge) = stack.pop() {
        let pos = (edge.1, edge.2);
        for along in [edge.0.clockwise(), edge.0.counter_clockwise()] {
            let next = pos + along;
            let neighbor = (edge.0, next.0, next.1);
            if sides.remove(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
}

fn get_sides(region: &HashSet<(i64, i64)>) -> usize {
    let mut sides: HashSet<_> = get_edges(region).collect();
    let mut i = 0;
    while !sides.is_empty() {
        i += 1;
        let start = *sides.iter().next().unwrap();
        sides.remove(&start);
        remove_contiguous(&mut sides, start);
    }
    i
}

/// Sum `area * perimeter` over every region, with `perimeter` measuring either
/// fence segments (part a) or straight sides (part b).
fn solve(input: &str, perimeter: impl Fn(&HashSet<(i64, i64)>) -> usize) -> usize {
    let map = read_char(input);
    let mut seen = Grid::new(map.rows(), map.cols());
    map.indexed_iter()
        .filter_map(|((x, y), _)| {
            if seen[(x, y)] {
                return None;
            }
            let start = (x.try_into().unwrap(), y.try_into().unwrap());
            let region: HashSet<_> = find_region(&mut seen, &map, start).into_iter().collect();
            Some(region.len() * perimeter(&region))
        })
        .sum()
}

pub fn solution_a(input: &str) -> usize {
    solve(input, |region| get_edges(region).count())
}

pub fn solution_b(input: &str) -> usize {
    solve(input, get_sides)
}

pub fn main(_: bool) {
    aoc::run("12", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/12.txt", 140);
    aoc::make_test!("b", "2024/12.txt", 80);
}
