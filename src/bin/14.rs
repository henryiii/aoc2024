/*!
# 2024 Day 14: Sample
## Simple template

<https://adventofcode.com/2024/day/14>

This is a small example to get started, also functions as a template for new days.
*/

use clap::Parser;
use counter::Counter;
use grid::Grid;

use aoc2024::geom::Point;

#[derive(Parser, Debug)]
#[command()]
struct Opts {
    #[clap(long)]
    vis: bool,
}

type Int = i32;

fn read_input(input: &str) -> Vec<(Point<Int>, Point<Int>)> {
    use aoc_parse::{parser, prelude::*};

    parser!(
        lines(
            "p=" a:(i32 "," i32) " v=" b:(i32 "," i32)
            => (Point::from(a), Point::from(b))
        )
    )
    .parse(input)
    .unwrap()
}

const fn mid_pt(val: Int, edge: Int) -> Option<bool> {
    let double = val * 2;
    if double == edge - 1 {
        None
    } else {
        Some(double > edge)
    }
}

fn to_grid(robots: &[(Point<Int>, Point<Int>)], size: (Int, Int)) -> Grid<usize> {
    let mut grid: Grid<usize> = Grid::new(size.1.try_into().unwrap(), size.0.try_into().unwrap());
    for (Point(x, y), _) in robots {
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
    let size = if robots.len() < 20 {
        (11, 7)
    } else {
        (101, 103)
    };

    let new_robots = robots
        .into_iter()
        .map(|(pos, vel)| ((pos + vel * 100).rem_euclid(size), vel));

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
        for (pos, vel) in &mut robots {
            *pos = (*pos + *vel).rem_euclid(size);
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
        assert_eq!(super::solution_a(INPUT), 12);
    }
}
