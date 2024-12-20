/*!
# 2024 Day 15: Warehouse Woes
## Pushing blocks around

<https://adventofcode.com/2024/day/15>

I went with the rasterized solution, which is quite fast, but probably harder
than storing the blocks and obsticles and moving them around. Doing that would
require searching through the (moving) blocks, which would might be a bit slow,
but finding and moving the blocks would be easier, I think.
*/

use aoc::grid::{visualize, Direction};

use grid::Grid;
use itertools::Itertools;
use strum::IntoEnumIterator;

type Int = usize;

fn read_input(input: &str) -> (Grid<char>, Vec<Direction>) {
    use aoc_parse::{parser, prelude::*};

    let dirs = Direction::iter().collect::<Vec<_>>();
    parser!(
        a:section(lines(any_char+))
        b:section(lines(char_of("^>v<")+))
        => (Grid::<char>::from(a), b.iter().flatten().map(|&c| dirs[c]).collect())
    )
    .parse(input)
    .unwrap()
}

fn find_end(grid: &Grid<char>, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let mut pos = pos;
    loop {
        pos = pos + dir;
        match grid[pos] {
            '#' => return None,
            '.' => return Some(pos),
            _ => {}
        }
    }
}

fn push_simple(
    grid: &mut Grid<char>,
    pos: (usize, usize),
    dir: Direction,
) -> Option<(usize, usize)> {
    // Check to see if we can push the block
    let end_pos = find_end(grid, pos, dir);
    end_pos.map(|mut new_pos| {
        let rev_dir = dir.reverse();
        while new_pos != pos {
            let next_pos = new_pos + rev_dir;
            grid.swap(new_pos, next_pos);
            new_pos = next_pos;
        }
        pos + dir
    })
}

fn push_vertical(
    grid: &Grid<char>,
    pos: (usize, usize),
    dir: Direction,
) -> Option<Vec<(usize, usize)>> {
    use Direction::{Left, Right};

    let c = grid[pos];
    let next_pos = pos + dir;
    let next_c = grid[next_pos];
    let mut push = vec![pos];
    match (c, next_c) {
        (_, '#') => return None,
        ('@' | ']' | '[', '.') => {}
        ('[', '[') | (']', ']') => push.extend(push_vertical(grid, next_pos, dir)?),
        ('@' | ']', '[') => {
            push.extend(push_vertical(grid, next_pos, dir)?);
            push.extend(push_vertical(grid, next_pos + Right, dir)?);
        }
        ('@' | '[', ']') => {
            push.extend(push_vertical(grid, next_pos, dir)?);
            push.extend(push_vertical(grid, next_pos + Left, dir)?);
        }
        _ => {
            panic!("Unexpected characters: {c} {next_c} going {dir:?} {push:?}");
        }
    };
    let mut ret: Vec<_> = push.into_iter().unique().sorted_unstable().collect();
    if dir == Direction::Down {
        ret.reverse();
    }
    Some(ret)
}

fn solution_a(input: &str, vis: bool) -> Int {
    let (mut grid, dirs) = read_input(input);
    let (mut pos, _) = grid.indexed_iter().find(|(_, &c)| c == '@').unwrap();
    vis.then(|| visualize(&grid, |c| *c));
    for dir in dirs {
        if let Some(new_pos) = push_simple(&mut grid, pos, dir) {
            pos = new_pos;
        }
    }
    vis.then(|| visualize(&grid, |c| *c));
    grid.indexed_iter()
        .filter(|(_, &c)| c == 'O')
        .map(|((r, c), _)| r * 100 + c)
        .sum()
}

fn solution_b(input: &str, vis: bool) -> Int {
    let (grid, dirs) = read_input(input);
    let mut grid = Grid::from_vec(
        grid.iter()
            .flat_map(|c| match *c {
                'O' => vec!['[', ']'],
                '@' => vec!['@', '.'],
                '.' => vec!['.', '.'],
                '#' => vec!['#', '#'],
                _ => unreachable!(),
            })
            .collect(),
        grid.cols() * 2,
    );
    let (mut pos, _) = grid.indexed_iter().find(|(_, &c)| c == '@').unwrap();

    for dir in dirs {
        let new_pos = match dir {
            Direction::Left | Direction::Right => push_simple(&mut grid, pos, dir),
            Direction::Up | Direction::Down => push_vertical(&grid, pos, dir).map(|val| {
                for v in val {
                    grid.swap(v, v + dir);
                }
                pos + dir
            }),
        };
        if let Some(new_pos) = new_pos {
            pos = new_pos;
        }
    }
    vis.then(|| visualize(&grid, |x| *x));

    grid.indexed_iter()
        .filter(|(_, &c)| c == '[')
        .map(|((r, c), _)| r * 100 + c)
        .sum()
}

pub fn main(vis: bool) {
    aoc::run(
        "15",
        |input| solution_a(input, vis),
        |input| solution_b(input, vis),
    );
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../../../samples/2024/15.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT, true), 10092);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT, true), 9021);
    }
}
