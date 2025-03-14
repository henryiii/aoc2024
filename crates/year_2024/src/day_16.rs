/*!
# 2024 Day 16: Reindeer Maze
## Path through maze with turns

<https://adventofcode.com/2024/day/16>

This is a low-cost path traversal, but with turns costing 1000 more than going straight.

This can be solved with a node graph, with two nodes per point, one for each
direction pair, and a 1000 cost to move between them, and since it turned out to be 20x faster,
I rewrote the solution to do exactly that. Part 2 is likely faster if I rewrote it too, but
it's fast enough as is, based on the original solution.
*/

#[cfg(test)]
use aoc::grid::dual_visualize;
use aoc::grid::{Direction, read_char};

use grid::Grid;
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*};

#[cfg(test)]
use inline_colorization::{color_red, color_reset};

type Int = usize;

fn make_graph(grid: &Grid<char>) -> UnGraphMap<(usize, usize, bool), usize> {
    let horiz = grid.iter_rows().enumerate().flat_map(|(row, rows)| {
        rows.enumerate()
            .tuple_windows()
            .filter_map(move |((col_n, &a), (col_m, &b))| {
                (a != '#' && b != '#').then_some(((row, col_n, false), (row, col_m, false)))
            })
    });
    let vert = grid.iter_cols().enumerate().flat_map(|(col, cols)| {
        cols.enumerate()
            .tuple_windows()
            .filter_map(move |((row_n, &a), (row_m, &b))| {
                (a != '#' && b != '#').then_some(((row_n, col, true), (row_m, col, true)))
            })
    });
    let cons = grid
        .indexed_iter()
        .filter(|&(ref pos, &c)| {
            c != '#'
                && (!(grid[(pos.0 + 1, pos.1)] != '#'
                    && grid[(pos.0 - 1, pos.1)] != '#'
                    && grid[(pos.0, pos.1 + 1)] == '#'
                    && grid[(pos.0, pos.1 - 1)] == '#')
                    && !(grid[(pos.0 + 1, pos.1)] == '#'
                        && grid[(pos.0 - 1, pos.1)] == '#'
                        && grid[(pos.0, pos.1 + 1)] != '#'
                        && grid[(pos.0, pos.1 - 1)] != '#'))
        })
        .map(|(pos, &c)| {
            (
                (pos.0, pos.1, false),
                (pos.0, pos.1, true),
                if c == 'E' { 0 } else { 1000 },
            )
        });
    horiz
        .chain(vert)
        .map(|(a, b)| (a, b, 1))
        .chain(cons)
        .collect()
}

fn track(
    grid: &Grid<char>,
    costs: &Grid<usize>,
    mut tracker: Grid<bool>,
    mut start: (i64, i64),
    mut dir: Direction,
    mut cost: Int,
    final_cost: Int,
) -> Grid<bool> {
    let mut res = Grid::new(grid.rows(), grid.cols());
    let mut paths: Vec<_>;

    loop {
        *tracker.get_mut(start.0, start.1).unwrap() = true;

        paths = [
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

        if paths.is_empty() {
            break;
        }
        (dir, start, cost) = paths.pop().unwrap();

        for (dir, start, cost) in paths {
            let nres = track(grid, costs, tracker.clone(), start, dir, cost, final_cost);
            res.iter_mut().zip(nres.iter()).for_each(|(r, t)| *r |= *t);
        }
    }

    res
}

pub fn solution_a(input: &str) -> Int {
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let end = grid.indexed_iter().find(|&(_, c)| *c == 'E').unwrap().0;
    let graph = make_graph(&grid);
    let costs = dijkstra(
        &graph,
        (start.0, start.1, false),
        Some((end.0, end.1, false)),
        |(_, _, c)| *c,
    );
    costs[&(end.0, end.1, false)]
}

pub fn solution_b(input: &str) -> Int {
    let grid = read_char(input);
    let start = grid.indexed_iter().find(|&(_, c)| *c == 'S').unwrap().0;
    let end = grid.indexed_iter().find(|&(_, c)| *c == 'E').unwrap().0;
    let graph = make_graph(&grid);
    let node_costs = dijkstra(&graph, (start.0, start.1, false), None, |(_, _, c)| *c);

    let mut costs = grid.map_ref(|_| usize::MAX);
    for ((row, col, _), cost) in node_costs {
        costs[(row, col)] = cost.min(costs[(row, col)]);
    }

    let tracker = track(
        &grid,
        &costs,
        Grid::new(grid.rows(), grid.cols()),
        (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
        Direction::Right,
        0,
        costs[(end.0, end.1)],
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

pub fn main(_: bool) {
    aoc::run("16", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT1: &str = include_str!("../../../samples/2024/16.txt");
    const INPUT2: &str = include_str!("../../../samples/2024/16-2.txt");

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
