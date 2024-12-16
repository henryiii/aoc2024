/*!
# 2024 Day 16: Reindeer Maze
## Path through maze with turns

<https://adventofcode.com/2024/day/16>

This is a low-cost path traversal, but with turns costing 1000 more than going straight.

This could be solved with a node graph, with two nodes per point, one for each
direction pair, and a 1000 cost to move between them.
*/

#[cfg(test)]
use aoc2024::grid::dual_visualize;
use aoc2024::grid::{read_char, Direction};

use grid::Grid;
#[cfg(test)]
use inline_colorization::{color_red, color_reset};

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
        if *cost > final_cost {
            return None;
        }
        let new_pos = start + *dir;
        let char = *grid.get(new_pos.0, new_pos.1)?;
        if char == 'E' {
            res.iter_mut()
                .zip(tracker.iter())
                .for_each(|(r, t)| *r |= *t);
        }

        // Careful here! We might enter a cell at a 90 degree angle from a
        // cheaper path and end in a tie, so we need to allow for that.
        let current_cost = *costs.get(new_pos.0, new_pos.1)?;
        if char == '.' && (current_cost == *cost || current_cost + 1000 == *cost) {
            return Some((*dir, new_pos, *cost));
        }
        None
    })
    .collect();

    for (dir, start, cost) in paths {
        let nres = track(grid, costs, tracker.clone(), start, dir, cost, final_cost);
        res.iter_mut().zip(nres.iter()).for_each(|(r, t)| *r |= *t);
    }
    res
}

fn solution_a(input: &str) -> Int {
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let start = (start.0.try_into().unwrap(), start.1.try_into().unwrap());
    let mut costs = Grid::new(grid.rows(), grid.cols());
    costs.fill(usize::MAX);

    walk(&grid, &mut costs, start, Direction::Right, 0)
        .into_iter()
        .min()
        .unwrap()
}

fn solution_b(input: &str) -> Int {
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let start = (start.0.try_into().unwrap(), start.1.try_into().unwrap());
    let mut costs = Grid::new(grid.rows(), grid.cols());
    costs.fill(usize::MAX);

    let final_cost = walk(&grid, &mut costs, start, Direction::Right, 0)
        .into_iter()
        .min()
        .unwrap();

    let tracker = track(
        &grid,
        &costs,
        Grid::new(grid.rows(), grid.cols()),
        start,
        Direction::Right,
        0,
        final_cost,
    );

    #[cfg(test)]
    dual_visualize(&costs, &tracker, |&c, &b| {
        if b {
            format!("{color_red}{c:6}{color_reset}")
        } else if c == usize::MAX {
            "  XXXX".to_string()
        } else {
            format!("{c:6}")
        }
    });

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
