/*!
# 2024 Day 16: Sample
## Simple template

<https://adventofcode.com/2024/day/16>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::grid::{read_char, Direction};

use grid::Grid;

type Int = usize;

fn walk(
    grid: &Grid<char>,
    costs: &mut Grid<usize>,
    start: (i64, i64),
    dir: Direction,
    cost: Int,
) -> Vec<Int> {
    let mut res = vec![];
    *costs.get_mut(start.0, start.1).unwrap() = cost;
    let paths: Vec<_> = [
        (dir, cost + 1),
        (dir.counter_clockwise(), cost + 1000 + 1),
        (dir.clockwise(), cost + 1000 + 1),
    ]
    .iter()
    .filter_map(|(dir, cost)| {
        let new_pos = start + *dir;
        let char = *grid.get(new_pos.0, new_pos.1)?;
        if char == 'E' {
            res.push(*cost);
        }
        if char == '.' && *costs.get(new_pos.0, new_pos.1)? > *cost {
            return Some((*dir, new_pos, *cost));
        }
        None
    })
    .collect();
    for (dir, path, cost) in paths {
        res.extend(walk(grid, costs, path, dir, cost));
    }
    res
}

fn solution_a(input: &str) -> Int {
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let mut costs = Grid::new(grid.rows(), grid.cols());
    costs.fill(usize::MAX);

    walk(
        &grid,
        &mut costs,
        (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
        Direction::Right,
        0,
    )
    .into_iter()
    .min()
    .unwrap()
}

fn solution_b(input: &str) -> Int {
    0
}

fn main() {
    aoc2024::run("16", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/16.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 7036);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 0);
    }
}
