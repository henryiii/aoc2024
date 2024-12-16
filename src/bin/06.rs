/*!
# 2024 Day 6: Guard Gallivant
## Walking with obstacles

<https://adventofcode.com/2024/day/6>

I'm using the direction code from my 2023 solutions. The first solution was
really slow, from using `HashMap`. Switching to the bitflag-like solution sped
it up, but it was still slow due to rerunning iterators. Using a little memory
is okay! Finally it was quite fast. The current solution adds parallel
processing for part 2, which works really well. I've simplifed the 2023
Direction by removing Position, which now only helps very slightly, due to
`.get` being smarter in Grid this year.
*/

use grid::Grid;

use aoc2024::grid::{read_char, Direction};
use aoc2024::par::prelude::*;

enum Result {
    Exited(Grid<u8>),
    Cyclic,
}

fn next_step(map: &Grid<char>, pos: (i64, i64), dir: Direction) -> Direction {
    let next = pos + dir;
    if map.get(next.0, next.1) == Some(&'#') {
        next_step(map, pos, dir.clockwise())
    } else {
        dir
    }
}

fn get_pos<T: TryFrom<usize>>(map: &Grid<char>) -> (T, T) {
    map.indexed_iter()
        .find_map(|((x, y), c)| {
            (*c == '^').then(|| (x.try_into().ok().unwrap(), y.try_into().ok().unwrap()))
        })
        .unwrap()
}

fn solve(map: &Grid<char>) -> Result {
    let mut pos = get_pos(map);
    let mut visited: Grid<u8> = Grid::new(map.rows(), map.cols());
    let mut dir = Direction::Up;
    while let Some(visit) = visited.get_mut(pos.0, pos.1) {
        if *visit & dir as u8 != 0 {
            return Result::Cyclic;
        }
        *visit |= dir as u8;
        dir = next_step(map, pos, dir);
        pos = pos + dir;
    }
    Result::Exited(visited)
}

fn solution_a(input: &str) -> usize {
    let map = read_char(input);
    if let Result::Exited(visited) = solve(&map) {
        visited.iter().filter(|&v| *v > 0).count()
    } else {
        panic!("No solution found");
    }
}

fn solution_b(input: &str) -> usize {
    let orig_map = read_char(input);
    // Only place barriers on the original walking path
    let starting_pos = get_pos(&orig_map);
    if let Result::Exited(solved_map) = solve(&orig_map) {
        solved_map
            .indexed_iter()
            .filter(|(v, &c)| c > 0 && starting_pos != *v)
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|((x, y), _)| {
                let mut map = orig_map.clone();
                map[(*x, *y)] = '#';
                let result = solve(&map);
                matches!(result, Result::Cyclic)
            })
            .count()
    } else {
        panic!("No solution found");
    }
}

fn main() {
    aoc2024::run("06", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/06.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 41);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 6);
    }
}
