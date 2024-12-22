/*!
# 2024 Day 21: Sample
## Simple template

<https://adventofcode.com/2024/day/21>

This is a small example to get started, also functions as a template for new days.
*/

type Int = usize;

use std::iter;
use std::sync::LazyLock;

use grid::{grid, Grid};
use itertools::Itertools;

static NUMERIC_KEYPAD: LazyLock<Grid<char>> = LazyLock::new(|| {
    grid![
        ['7', '8', '9']
        ['4', '5', '6']
        ['1', '2', '3']
        ['x', '0', 'A']
    ]
});

static DIR_KEYPAD: LazyLock<Grid<char>> = LazyLock::new(|| {
    grid![
        ['x', '^', 'A']
        ['<', 'v', '>']
    ]
});

fn pos(current: char, pad: &Grid<char>) -> (usize, usize) {
    pad.indexed_iter().find(|(_, &c)| c == current).unwrap().0
}

fn step(current: char, key: char, pad: &Grid<char>) -> Option<char> {
    let (x, y) = pos(current, pad);
    let new_key = match key {
        '^' => *pad.get(x - 1, y)?,
        'v' => *pad.get(x + 1, y)?,
        '<' => *pad.get(x, y - 1)?,
        '>' => *pad.get(x, y + 1)?,
        'A' => current,
        _ => unreachable!(),
    };
    if new_key == 'x' {
        return None;
    }
    Some(new_key)
}

fn valid_path(current: char, path: &[char], pad: &Grid<char>) -> Option<char> {
    path.iter()
        .try_fold(current, |acc, key| step(acc, *key, pad))
}

fn paths(current: char, target: char, pad: &Grid<char>) -> Vec<Vec<char>> {
    let cur_pos = pad.indexed_iter().find(|(_, &c)| c == current).unwrap().0;
    let new_pos = pad.indexed_iter().find(|(_, &c)| c == target).unwrap().0;
    let vert_chars = if cur_pos.0 < new_pos.0 {
        iter::repeat('v').take(new_pos.0 - cur_pos.0)
    } else {
        iter::repeat('^').take(cur_pos.0 - new_pos.0)
    };
    let horiz_chars = if cur_pos.1 < new_pos.1 {
        iter::repeat('>').take(new_pos.1 - cur_pos.1)
    } else {
        iter::repeat('<').take(cur_pos.1 - new_pos.1)
    };
    let vals1: Vec<_> = vert_chars
        .clone()
        .chain(horiz_chars.clone())
        .chain(iter::once('A'))
        .collect();
    let vals2: Vec<_> = horiz_chars
        .chain(vert_chars)
        .chain(iter::once('A'))
        .collect();
    [vals1, vals2]
        .iter()
        .unique()
        .filter(|v| valid_path(current, v, pad).is_some())
        .cloned()
        .collect()
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(a:string(any_char+) => a.chars().collect::<Vec<char>>()))
        .parse(input)
        .unwrap()
}

fn expand(path: &[char], pad: &Grid<char>) -> Vec<Vec<char>> {
    // Add the starting position
    iter::once(&'A')
        .chain(path.iter())
        .tuple_windows()
        .map(|(current, target)| paths(*current, *target, pad))
        .reduce(|a, b| {
            a.iter()
                .cartesian_product(b.iter())
                .map(|(x, y)| x.iter().chain(y).copied().collect())
                .collect()
        })
        .unwrap()
}

fn solution_a(input: &str) -> Int {
    let lines = read_input(input);
    lines
        .into_iter()
        .map(|line| {
            let newlines = expand(&line, &NUMERIC_KEYPAD);
            let newlines = newlines.into_iter().flat_map(|v| expand(&v, &DIR_KEYPAD));
            let newlines = newlines.into_iter().flat_map(|v| expand(&v, &DIR_KEYPAD));
            let min = newlines.min_by_key(Vec::len).unwrap().len();
            let val: Int = String::from_iter(&line)
                .strip_suffix("A")
                .unwrap()
                .parse()
                .unwrap();
            val * min
        })
        .sum()
}

fn solution_b(input: &str) -> Int {
    let lines = read_input(input);
    lines
        .into_iter()
        .map(|line| {
            let newlines = expand(&line, &NUMERIC_KEYPAD);
            let newlines: Vec<Vec<char>> = newlines
                .into_iter()
                .flat_map(|v| expand(&v, &DIR_KEYPAD))
                .collect();
            let newlines: Vec<Vec<char>> = newlines
                .into_iter()
                .flat_map(|v| expand(&v, &DIR_KEYPAD))
                .collect();
            let min = newlines.iter().min_by_key(|v| v.len()).unwrap().len();
            let val: Int = String::from_iter(&line)
                .strip_suffix("A")
                .unwrap()
                .parse()
                .unwrap();
            val * min
        })
        .sum()
}

pub fn main(_: bool) {
    aoc::run("21", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    aoc::make_test!("a", "2024/21.txt", 126_384);
    aoc::make_test!("b", "2024/21.txt", 0);

    #[test]
    fn test_paths_numeric() {
        assert_eq!(paths('A', '3', &NUMERIC_KEYPAD), vec![vec!['^', 'A']]);
        assert_eq!(paths('3', '3', &NUMERIC_KEYPAD), vec![vec!['A']]);
        assert_eq!(paths('3', '1', &NUMERIC_KEYPAD), vec![vec!['<', '<', 'A']]);
        assert_eq!(paths('0', '1', &NUMERIC_KEYPAD), vec![vec!['^', '<', 'A']]);
        assert_eq!(
            paths('3', '7', &NUMERIC_KEYPAD),
            vec![vec!['^', '^', '<', '<', 'A'], vec!['<', '<', '^', '^', 'A']]
        );
    }

    #[test]
    fn test_paths_dir() {
        assert_eq!(paths('A', '>', &DIR_KEYPAD), vec![vec!['v', 'A']]);
        assert_eq!(paths('A', '<', &DIR_KEYPAD), vec![vec!['v', '<', '<', 'A']]);
    }
}
