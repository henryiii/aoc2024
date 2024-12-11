/*!
# 2024 Day 1: Historian Hysteria
##  Lists of numbers

<https://adventofcode.com/2024/day/1>

The key here was IO, as usual for early days. The goal is to cleanly get two
lists from a file. I'm going with two dependencies here, counter and itertools,
for clean code that is simple. Going through a grid, using a regex, etc. would
have been less bare-metal, though maybe more adaptable later.
*/
use counter::Counter;
use itertools::Itertools;

fn lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .unzip()
}

fn solution_a(input: &str) -> usize {
    let (mut row1, mut row2) = lists(input);
    row1.sort_unstable();
    row2.sort_unstable();
    row1.into_iter().zip(row2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn solution_b(input: &str) -> usize {
    let (row1, row2) = lists(input);
    let counts: Counter<_> = row2.iter().collect();
    row1.iter().map(|x| counts[x] * x).sum()
}

fn main() {
    aoc2024::run("01", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/01.txt");

    #[test]
    fn test_day01a() {
        assert_eq!(super::solution_a(INPUT), 11);
    }

    #[test]
    fn test_day01b() {
        assert_eq!(super::solution_b(INPUT), 31);
    }
}
