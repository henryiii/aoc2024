/*!
# 2024 Day 12: Garden Groups
## Finding regions

<https://adventofcode.com/2024/day/12>

Building up `Vec`'s, then converting to a `BTreeSet` was faster than building
`BTreeSet`/`HashSet`.
*/

use std::collections::BTreeSet;

use grid::Grid;
use strum::IntoEnumIterator;

use aoc2024::grid::{read_char, Direction};

fn find_region(seen: &mut Grid<bool>, map: &Grid<char>, start: (i64, i64)) -> Vec<(i64, i64)> {
    *seen.get_mut(start.0, start.1).unwrap() = true;
    let ch = map.get(start.0, start.1).unwrap();
    std::iter::once(start)
        .chain(
            Direction::iter()
                .filter_map(|dir| {
                    let pos = start + dir;
                    if seen.get(pos.0, pos.1) == Some(&false) && map.get(pos.0, pos.1) == Some(ch) {
                        Some(find_region(seen, map, pos))
                    } else {
                        None
                    }
                })
                .flatten(),
        )
        .collect()
}

fn get_edges(
    region: &'_ BTreeSet<(i64, i64)>,
) -> impl Iterator<Item = (Direction, i64, i64)> + use<'_> {
    region.iter().flat_map(|(x, y)| {
        Direction::iter()
            .filter(|dir| !region.contains(&((*x, *y) + *dir)))
            .map(|dir| (dir, *x, *y))
    })
}

fn remove_contiguous(sides: &mut Vec<(Direction, i64, i64)>, start: (Direction, i64, i64)) {
    let a = (start.1, start.2) + start.0.clockwise();
    let b = (start.1, start.2) + start.0.counter_clockwise();
    if let Some(side) = sides.iter().position(|x| x == &(start.0, a.0, a.1)) {
        sides.remove(side);
        remove_contiguous(sides, (start.0, a.0, a.1));
    }
    if let Some(side) = sides.iter().position(|x| x == &(start.0, b.0, b.1)) {
        sides.remove(side);
        remove_contiguous(sides, (start.0, b.0, b.1));
    }
}

fn get_sides(region: &BTreeSet<(i64, i64)>) -> usize {
    let mut sides: Vec<_> = get_edges(region).collect();
    let mut i = 0;
    while !sides.is_empty() {
        i += 1;
        let start = sides.pop().unwrap();
        remove_contiguous(&mut sides, start);
    }
    i
}

fn solution_a(input: &str) -> usize {
    let map = read_char(input);
    let mut seen = Grid::new(map.rows(), map.cols());
    map.indexed_iter()
        .filter_map(|((x, y), _)| {
            if seen[(x, y)] {
                return None;
            }
            let start = (x.try_into().unwrap(), y.try_into().unwrap());
            let region: BTreeSet<_> = find_region(&mut seen, &map, start).into_iter().collect();
            Some((region.len(), get_edges(&region).count()))
        })
        .fold(0, |acc, (a, p)| acc + a * p)
}

fn solution_b(input: &str) -> usize {
    let map = read_char(input);
    let mut seen = Grid::new(map.rows(), map.cols());
    map.indexed_iter()
        .filter_map(|((x, y), _)| {
            if seen[(x, y)] {
                return None;
            }
            let start = (x.try_into().unwrap(), y.try_into().unwrap());
            let region: BTreeSet<_> = find_region(&mut seen, &map, start).into_iter().collect();
            Some((region.len(), get_sides(&region)))
        })
        .fold(0, |acc, (a, p)| acc + a * p)
}

fn main() {
    aoc2024::run("12", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/12.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 140);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 80);
    }
}
