/*!
# 2024 Day 0: Sample
##  Simple template

<https://adventofcode.com/2024/day/1>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::run;

fn solution_a(input: &str) -> i64 {
    input.lines().map(|line| line.parse::<i64>().unwrap()).sum()
}

fn solution_b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|x| x * x)
        .sum()
}

fn main() {
    run("sample", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/sample.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 6);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 14);
    }
}
