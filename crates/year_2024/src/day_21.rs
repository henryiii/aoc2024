/*!
# 2024 Day 21: Keypad Conundrum
## Chained keypads

<https://adventofcode.com/2024/day/21>

This was really hard at first, largely because I approached it from the other
direction. Once I flipped the order I was trying to do it in, it became easy.
Switching from a vector of vectors to an enum of vectors, along with no longer
building out the paths one by one but instead checking the corner value, made
this much faster.

This looks great, by the way, though it's not really accurate to the way I solved it:
<https://www.reddit.com/r/adventofcode/comments/1hkc7h7/2024_day_21_button_mashing_greyed_out_memoized/>
*/

use std::iter;

use counter::Counter;
use grid::{grid, Grid};
use itertools::Itertools;

enum Paths {
    Single(Vec<char>),
    Double(Vec<char>, Vec<char>),
}

fn path_to_presses(path: &[char], prev: &Counter<(char, char)>) -> usize {
    iter::once('A')
        .chain(path.iter().copied())
        .chain(iter::once('A'))
        .tuple_windows()
        .map(|(i, f)| prev[&(i, f)])
        .sum()
}

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

    fn paths(&self, current: char, target: char) -> Paths {
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

        if cur_pos.0 == new_pos.0 {
            Paths::Single(horiz_chars.collect())
        } else if cur_pos.1 == new_pos.1 {
            Paths::Single(vert_chars.collect())
        } else if self.pad[(cur_pos.0, new_pos.1)] == 'x' {
            Paths::Single(vert_chars.chain(horiz_chars).collect())
        } else if self.pad[(new_pos.0, cur_pos.1)] == 'x' {
            Paths::Single(horiz_chars.chain(vert_chars).collect())
        } else {
            Paths::Double(
                horiz_chars.clone().chain(vert_chars.clone()).collect(),
                vert_chars.chain(horiz_chars).collect(),
            )
        }
    }

    fn calc_next(&self, prev: &Counter<(char, char)>) -> Counter<(char, char)> {
        // Compute every possible pair
        self.iter()
            .flat_map(|ki| {
                self.iter().map(|kf| {
                    let paths = self.paths(*ki, *kf);
                    let key_presses = match paths {
                        Paths::Single(path) => path_to_presses(&path, prev),
                        Paths::Double(path1, path2) => {
                            usize::min(path_to_presses(&path1, prev), path_to_presses(&path2, prev))
                        }
                    };
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

fn solution(input: &str, robots: usize) -> usize {
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
            let val: usize = String::from_iter(&line)
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
