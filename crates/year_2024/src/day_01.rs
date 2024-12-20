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

pub fn main(_: bool) {
    aoc::run("01", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/01.txt", 11);
    aoc::make_test!("b", "2024/01.txt", 31);
}
