/*!
# 2024 Day 14: Sample
## Simple template

<https://adventofcode.com/2024/day/14>

This is a small example to get started, also functions as a template for new days.
*/

use clap::Parser;
use counter::Counter;
use grid::Grid;

#[derive(Parser, Debug)]
#[command()]
struct Opts {
    #[clap(long)]
    vis: bool,
}

type Point = (i64, i64);

fn read_input(input: &str) -> Vec<(Point, Point)> {
    use aoc_parse::{parser, prelude::*};

    parser!(
        lines(
            "p=" (i64 "," i64) " v=" (i64 "," i64)
        )
    )
    .parse(input)
    .unwrap()
}

const fn mid_pt(val: i64, edge: i64) -> Option<bool> {
    let double = val * 2;
    if double == edge - 1 {
        None
    } else {
        Some(double > edge)
    }
}

fn to_grid(robots: &[(Point, Point)], size: Point) -> Grid<usize> {
    let mut grid: Grid<usize> = Grid::new(size.1.try_into().unwrap(), size.0.try_into().unwrap());
    for ((x, y), _) in robots {
        *grid.get_mut(*y, *x).unwrap() += 1;
    }
    grid
}

fn vis_grid(grid: &Grid<usize>) {
    for row in grid.iter_rows() {
        for cell in row {
            if *cell == 0 {
                print!(".");
            } else {
                print!("{cell}");
            }
        }
        println!();
    }
}

fn solution_a(input: &str) -> usize {
    let robots = read_input(input);
    let size = if robots.len() < 20 { (11, 7) } else { (101, 103) };

    let new_robots = robots.into_iter().map(|((x, y), (dx, dy))| {
        (
            (
                (x + 100 * dx).rem_euclid(size.0),
                (y + 100 * dy).rem_euclid(size.1),
            ),
            (dx, dy),
        )
    });

    #[cfg(test)]
    vis_grid(&to_grid(&new_robots.clone().collect::<Vec<_>>(), size));

    let counts: Counter<_> = new_robots
        .into_iter()
        .filter_map(|(r, _)| Some((mid_pt(r.0, size.0)?, mid_pt(r.1, size.1)?)))
        .collect();
    counts.values().product()
}

fn solution_b(input: &str, vis: bool) -> usize {
    let mut robots = read_input(input);
    let size = (101, 103);
    for i in 1..=10000 {
        for ((x, y), (dx, dy)) in &mut robots {
            *x = (*x + *dx).rem_euclid(size.0);
            *y = (*y + *dy).rem_euclid(size.1);
        }

        let grid = to_grid(&robots, size);
        if *grid.iter().max().unwrap() == 1 {
            if vis {
                vis_grid(&grid);
            }
            return i;
        }
    }
    0
}

fn main() {
    let opts: Opts = Opts::parse();
    aoc2024::run("14", solution_a, |input| solution_b(input, opts.vis));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/14.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 11);
    }
}
