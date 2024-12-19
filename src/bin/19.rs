/*!
# 2024 Day 19: Sample
## Simple template

<https://adventofcode.com/2024/day/19>

This is a small example to get started, also functions as a template for new days.
*/

use cached::proc_macro::cached;

type Int = usize;

fn read_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        section(line(repeat_sep(alpha+, ", ")))
        section(lines(alpha+))
    )
    .parse(input)
    .unwrap()
}

#[cached(key = "Vec<char>", convert = "{Vec::from(line)}")]
fn has_match(patterns: &[Vec<char>], line: &[char]) -> bool {
    for pattern in patterns {
        if let Some(slice) = line.strip_prefix(&pattern[..]) {
            if slice.is_empty() {
                return true;
            }
            if has_match(patterns, slice) {
                return true;
            }
        }
    }
    false
}

fn solution_a(input: &str) -> Int {
    let (patterns, lines) = read_input(input);
    lines
        .iter()
        .filter(|line| has_match(&patterns, line))
        .count()
}

fn solution_b(input: &str) -> Int {
    todo!();
}

fn main() {
    aoc2024::run("19", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/19.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 6);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 0);
    }
}
