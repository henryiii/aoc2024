/*!
# 2024 Day 5: Sample
##  Simple template

<https://adventofcode.com/2024/day/5>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::{run, Problem};

use itertools::Itertools;

struct Day05 {}

fn parse_rules(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| line.contains('|'))
        .map(|line| {
            line.split('|')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}
fn parse_orders(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn in_order(rules: &[(i64, i64)], order: &[i64]) -> bool {
    rules
        .iter()
        .filter(|(a, b)| order.contains(a) & order.contains(b))
        .map(|(a, b)| {
            order.iter().position(|x| x == a).unwrap() < order.iter().position(|x| x == b).unwrap()
        })
        .all(|x| x)
}

fn put_in_order<'a>(rules: &[(i64, i64)], order: &'a [i64]) -> Vec<&'a i64> {
    let valid_rules = rules
        .iter()
        .filter(|(a, b)| order.contains(a) & order.contains(b));
    order
        .iter()
        .sorted_by(|&a, &b| {
            let rule = valid_rules
                .clone()
                .filter(|(x, y)| (x == a || x == b) && (y == a || y == b))
                .exactly_one()
                .unwrap();
            if rule.0 == *a {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .collect()
}

impl Problem for Day05 {
    fn solution_a(input: &str) -> i64 {
        let rules = parse_rules(input);
        let orders = parse_orders(input);
        orders
            .iter()
            .filter(|order| in_order(&rules, order))
            .map(|order| order[(order.len() - 1) / 2])
            .sum()
    }

    fn solution_b(input: &str) -> i64 {
        let rules = parse_rules(input);
        let orders = parse_orders(input);
        orders
            .iter()
            .filter(|order| !in_order(&rules, order))
            .map(|order| put_in_order(&rules, order)[(order.len() - 1) / 2])
            .sum()
    }
}

fn main() {
    run::<Day05>("05");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/05.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day05::solution_a(INPUT), 143);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day05::solution_b(INPUT), 123);
    }
}
