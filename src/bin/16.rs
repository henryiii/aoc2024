/*!
# 2024 Day 16: Sample
## Simple template

<https://adventofcode.com/2024/day/16>

This is a small example to get started, also functions as a template for new days.
*/

#[cfg(test)]
use aoc2024::grid::dual_visualize;
use aoc2024::grid::{read_char, Direction};

use grid::Grid;
#[cfg(test)]
use inline_colorization::{color_red, color_reset};
use itertools::Itertools;

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

fn track(
    grid: &Grid<char>,
    costs: &Grid<usize>,
    mut tracker: Grid<bool>,
    start: (i64, i64),
    dir: Direction,
    cost: Int,
    final_cost: Int,
) -> Grid<bool> {
    let mut res = Grid::new(grid.rows(), grid.cols());
    *tracker.get_mut(start.0, start.1).unwrap() = true;

    let paths: Vec<_> = [
        (dir, cost + 1),
        (dir.counter_clockwise(), cost + 1000 + 1),
        (dir.clockwise(), cost + 1000 + 1),
    ]
    .iter()
    .filter_map(|(dir, cost)| {
        let new_pos = start + *dir;
        let char = *grid.get(new_pos.0, new_pos.1)?;
        if *cost > final_cost {
            return None;
        }
        if char == 'E' {
            res.iter_mut()
                .zip(tracker.iter())
                .for_each(|(r, t)| *r |= *t);
            #[cfg(test)]
            dual_visualize(costs, &tracker, |&c, &b| {
                if b {
                    format!("{color_red}{c:6}{color_reset}")
                } else if c == usize::MAX {
                    "  XXXX".to_string()
                } else {
                    format!("{c:6}")
                }
            });
        }

        let current_cost = *costs.get(new_pos.0, new_pos.1)?;
        if char == '.' && (current_cost == *cost || current_cost + 1000 == *cost) {
            return Some((*dir, new_pos, *cost));
        }
        None
    })
    .collect();
    if let Ok(path) = paths.iter().exactly_one() {
        let (dir, start, cost) = path;
        res.iter_mut()
            .zip(track(grid, costs, tracker, *start, *dir, *cost, final_cost).iter())
            .for_each(|(r, t)| *r |= *t);
    } else {
        for (dir, start, cost) in paths {
            let nres = track(grid, costs, tracker.clone(), start, dir, cost, final_cost);
            res.iter_mut().zip(nres.iter()).for_each(|(r, t)| *r |= *t);
        }
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
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let mut costs = Grid::new(grid.rows(), grid.cols());
    costs.fill(usize::MAX);

    let total_cost = walk(
        &grid,
        &mut costs,
        (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
        Direction::Right,
        0,
    )
    .into_iter()
    .min()
    .unwrap();

    let tracker = track(
        &grid,
        &costs,
        Grid::new(grid.rows(), grid.cols()),
        (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
        Direction::Right,
        0,
        total_cost,
    );
    tracker.iter().filter(|&&t| t).count() + 1
}

fn main() {
    aoc2024::run("16", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT1: &str = include_str!("../../samples/16.txt");
    const INPUT2: &str = include_str!("../../samples/16-2.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT1), 7036);
    }

    #[test]
    fn test_sample_a2() {
        assert_eq!(super::solution_a(INPUT2), 11048);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT1), 45);
    }

    #[test]
    fn test_sample_b2() {
        assert_eq!(super::solution_b(INPUT2), 64);
    }
}
