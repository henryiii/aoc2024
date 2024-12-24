/*!
# 2024 Day 21: Sample
## Simple template

<https://adventofcode.com/2024/day/21>

This is a small example to get started, also functions as a template for new days.

This looks great: <https://www.reddit.com/r/adventofcode/comments/1hkc7h7/2024_day_21_button_mashing_greyed_out_memoized/>
*/

type Int = usize;

use std::iter;

use counter::Counter;
use grid::{grid, Grid};
use itertools::Itertools;

struct KeyPad {
    pad: Grid<char>,
}

impl KeyPad {
    fn new_numeric() -> Self {
        Self {
            pad: grid![
                ['7', '8', '9']
                ['4', '5', '6']
                ['1', '2', '3']
                ['x', '0', 'A']
            ],
        }
    }

    fn new_dir() -> Self {
        Self {
            pad: grid![
                ['x', '^', 'A']
                ['<', 'v', '>']
            ],
        }
    }

    fn indexed_iter(&self) -> impl Iterator<Item = ((usize, usize), &char)> {
        self.pad.indexed_iter().filter(|(_, &c)| c != 'x')
    }

    fn iter(&self) -> impl Iterator<Item = &char> {
        self.pad.iter().filter(|&c| *c != 'x')
    }

    fn pos(&self, current: char) -> (usize, usize) {
        self.indexed_iter().find(|(_, &c)| c == current).unwrap().0
    }

    fn valid_path(&self, current: char, path: &[char]) -> Option<char> {
        path.iter()
            .try_fold(current, |acc, key| self.step(acc, *key))
    }

    fn paths(&self, current: char, target: char) -> Vec<Vec<char>> {
        let cur_pos = self.pos(current);
        let new_pos = self.pos(target);

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
            .filter(|v| self.valid_path(current, v).is_some())
            .cloned()
            .collect()
    }

    fn step(&self, current: char, key: char) -> Option<char> {
        let (x, y) = self.pos(current);
        let new_key = match key {
            '^' => *self.pad.get(x - 1, y)?,
            'v' => *self.pad.get(x + 1, y)?,
            '<' => *self.pad.get(x, y - 1)?,
            '>' => *self.pad.get(x, y + 1)?,
            'A' => current,
            _ => unreachable!(),
        };
        if new_key == 'x' {
            return None;
        }
        Some(new_key)
    }

    fn calc_next(&self, prev: &Counter<(char, char)>) -> Counter<(char, char)> {
        // Compute every possible pair
        self.indexed_iter()
            .flat_map(|((_, _), ki)| {
                self.indexed_iter().map(|((_, _), kf)| {
                    let paths = self.paths(*ki, *kf);
                    let key_presses = paths
                        .into_iter()
                        .map(|path| {
                            iter::once('A')
                                .chain(path)
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
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(a:string(any_char+) => a.chars().collect::<Vec<char>>()))
        .parse(input)
        .unwrap()
}

fn solution(input: &str, robots: usize) -> Int {
    let numeric_keypad = KeyPad::new_numeric();
    let dir_keypad = KeyPad::new_dir();

    let my_presses = dir_keypad
        .iter()
        .flat_map(|&ki| dir_keypad.iter().map(move |&kf| ((ki, kf), 1)))
        .collect();
    let rob_presses = (0..robots).fold(my_presses, |prev, _| dir_keypad.calc_next(&prev));
    let final_presses = numeric_keypad.calc_next(&rob_presses);

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
    aoc::run(
        "21",
        |input| solution(input, 2),
        |input| solution(input, 25),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        assert_eq!(
            super::solution(include_str!(concat!("../../../samples/", "2024/21.txt")), 2),
            126_384
        );
    }
    #[test]
    fn test_part_b() {
        assert_eq!(
            super::solution(
                include_str!(concat!("../../../samples/", "2024/21.txt")),
                25
            ),
            154_115_708_116_294
        );
    }
}
