/*!
# 2024 Day 5: Print Queue
##  Sorting a print queue with order rules

<https://adventofcode.com/2024/day/5>

The first implmementation was slow (100+ ms); collecting a vector of rules sped
it up 20x. The next implementation is much simpler and faster, using comparison
functions.
*/

use std::collections::HashSet;

use itertools::Itertools;

use aoc_parse::{parser, prelude::*};

fn read_input(input: &str) -> (HashSet<(u64, u64)>, Vec<Vec<u64>>) {
    parser!(
        section(hash_set(lines(u64 "|" u64)))
        section(lines(repeat_sep(u64, ",")))
    )
    .parse(input)
    .unwrap()
}

fn compare_rules(rules: &HashSet<(u64, u64)>, a: u64, b: u64) -> std::cmp::Ordering {
    if rules.contains(&(a, b)) {
        std::cmp::Ordering::Less
    } else if rules.contains(&(b, a)) {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

/// Returns true if the order follows the rules.
fn in_order(rules: &HashSet<(u64, u64)>, order: &[u64]) -> bool {
    order.is_sorted_by(|&a, &b| compare_rules(rules, a, b) == std::cmp::Ordering::Less)
}

/// Reorders the order to follow the rules. Returns a new ordered vector.
fn put_in_order<'a>(rules: &HashSet<(u64, u64)>, order: &'a [u64]) -> Vec<&'a u64> {
    order
        .iter()
        .sorted_by(|&&a, &&b| compare_rules(rules, a, b))
        .collect()
}

fn solution_a(input: &str) -> u64 {
    let (rules, orders) = read_input(input);
    orders
        .iter()
        .filter(|order| in_order(&rules, order))
        .map(|order| order[(order.len() - 1) / 2])
        .sum()
}

fn solution_b(input: &str) -> u64 {
    let (rules, orders) = read_input(input);
    orders
        .iter()
        .filter(|order| !in_order(&rules, order))
        .map(|order| put_in_order(&rules, order)[(order.len() - 1) / 2])
        .sum()
}
fn main() {
    aoc2024::run("05", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/05.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 143);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 123);
    }
}
