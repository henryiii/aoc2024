/*!
# 2024 Day 14: Restroom Redoubt
## Movement prediction

<https://adventofcode.com/2024/day/14>

This has the first "Easter Egg" style output I've done, since it was not
included last year. I made an assumption to find the answer, and the assumption
seems to work on my data, not sure if it's always true, though.
*/

use counter::Counter;
use grid::Grid;

use aoc::geom::Point;

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

fn solution_a<const X: Int, const Y: Int>(input: &str) -> usize {
    let robots = read_input(input);
    let size = (X, Y);
    let new_robots = robots
        .into_iter()
        .map(|(pos, vel)| ((pos + vel * 100).rem_euclid(&size), vel));

    #[cfg(test)]
    {
        let mut grid = Grid::new(size.1.try_into().unwrap(), size.0.try_into().unwrap());
        new_robots.clone().for_each(|(pos, _)| grid[pos] += 1);
        vis_grid(&grid);
    }

    let counts: Counter<_> = new_robots
        .into_iter()
        .filter_map(|(r, _)| Some((mid_pt(r.0, size.0)?, mid_pt(r.1, size.1)?)))
        .collect();
    counts.values().product()
}

fn solution_b(input: &str, vis: bool) -> usize {
    let mut robots = read_input(input);
    let size = (101, 103);
    let mut grid = Grid::new(103, 101);
    for i in 1..=10000 {
        for (pos, vel) in &mut robots {
            *pos = (*pos + *vel).rem_euclid(&size);
        }

        grid.fill(0);
        for (pos, _) in &robots {
            grid[*pos] += 1;
        }
        if grid.iter().all(|x| *x <= 1) {
            if vis {
                vis_grid(&grid);
            }
            return i;
        }
    }
    0
}

pub fn main(vis: bool) {
    aoc::run("14", solution_a::<101, 103>, |input| solution_b(input, vis));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../../samples/2024/14.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a::<11, 7>(INPUT), 12);
    }
}
