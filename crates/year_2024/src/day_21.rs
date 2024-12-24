/*!
# 2024 Day 21: Sample
## Simple template

<https://adventofcode.com/2024/day/21>

This is a small example to get started, also functions as a template for new days.

This looks great: <https://www.reddit.com/r/adventofcode/comments/1hkc7h7/2024_day_21_button_mashing_greyed_out_memoized/>
*/

type Int = usize;

use std::iter;
use std::sync::LazyLock;

use counter::Counter;
use grid::{grid, Grid};
use itertools::Itertools;

// 319556127176302 is too high
// 312741267418362 is too high
// 193606477851200 is too high

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

fn calc_next(prev: &Counter<(char, char)>, pad: &Grid<char>) -> Counter<(char, char)> {
    // Compute every possible pair
    pad.indexed_iter()
        .filter(|(_, c)| **c != 'x')
        .flat_map(|((_, _), ki)| {
            pad.indexed_iter()
                .filter(|(_, c)| **c != 'x')
                .map(|((_, _), kf)| {
                    let paths = paths(*ki, *kf, pad);
                    let key_presses = paths
                        .into_iter()
                        .map(|path| {
                            iter::once('A')
                            .chain(path.into_iter())
                                .tuple_windows()
                                .map(|(i, f)| prev[&(i, f)])
                                .sum()
                        })
                        .min()
                        .unwrap();
                    ((*ki, *kf), key_presses)
                })
        })
        .collect()
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
            let val: Int = String::from_iter(&line)
                .strip_suffix("A")
                .unwrap()
                .parse()
                .unwrap();

            let newlines = expand(&line, &NUMERIC_KEYPAD);
            let newlines = newlines.into_iter().flat_map(|v| expand(&v, &DIR_KEYPAD));
            let newlines = newlines.into_iter().flat_map(|v| expand(&v, &DIR_KEYPAD));
            let min = newlines.min_by_key(Vec::len).unwrap().len();
            val * min
        })
        .sum()
}

fn solution_b(input: &str, robots: usize) -> Int {
    let my_presses = DIR_KEYPAD
        .iter()
        .filter(|&k| *k != 'x')
        .flat_map(|&ki| {
            DIR_KEYPAD
                .iter()
                .filter(|&k| *k != 'x')
                .map(move |&kf| ((ki, kf), 1))
        })
        .collect();
    let rob_presses = (0..robots).fold(my_presses, |prev, _| {
        let ret = calc_next(&prev, &DIR_KEYPAD);
        ret
    });
    let final_presses = calc_next(&rob_presses, &NUMERIC_KEYPAD);

    let lines = read_input(input);
    lines
        .into_iter()
        .map(|line| -> usize {
            let val: Int = String::from_iter(&line)
                .strip_suffix("A")
                .unwrap()
                .parse()
                .unwrap();
            iter::once('A')
                .chain(line)
                .tuple_windows()
                .map(|(i, f)| final_presses[&(i, f)])
                .sum::<usize>()
                * val
        })
        .sum()
}

pub fn main(_: bool) {
    aoc::run("21", solution_a, |input| solution_b(input, 25));
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    aoc::make_test!("a", "2024/21.txt", 126_384);

    #[test]
    fn test_part_b() {
        assert_eq!(
            super::solution_b(
                include_str!(concat!("../../../samples/", "2024/21.txt")),
                25
            ),
            154_115_708_116_294
        );
    }

    #[test]
    fn test_part_b_short() {
        assert_eq!(
            super::solution_b(include_str!(concat!("../../../samples/", "2024/21.txt")), 2),
            126_384
        );
    }

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

    #[test]
    fn counter_repeated_elements() {
        let counter: Counter<char> = [('A', 1), ('A', 1), ('A', 2)].into_iter().collect();
        assert_eq!(counter[&'A'], 4);
    }

    #[test]
    fn leg_lengths() {
    let my_presses = DIR_KEYPAD
        .iter()
        .filter(|&k| *k != 'x')
        .flat_map(|&ki| {
            DIR_KEYPAD
                .iter()
                .filter(|&k| *k != 'x')
                .map(move |&kf| ((ki, kf), 1))
        })
        .collect();
    let rob_presses = calc_next(&my_presses, &DIR_KEYPAD);
    assert_eq!(rob_presses[&('A', 'A')], 1);
    assert_eq!(rob_presses[&('<', '>')], 3);
    assert_eq!(rob_presses[&('<', 'A')], 4);
    }
}
