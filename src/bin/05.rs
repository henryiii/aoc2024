/*!
# 2024 Day 5: Print Queue
##  Sorting a print queue with order rules

<https://adventofcode.com/2024/day/5>

The first implmementation was slow (100+ ms); collecting a vector of rules sped
it up 20x.
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
        .filter_map(|(a, b)| {
            Some(order.iter().position(|x| x == a)? < order.iter().position(|x| x == b)?)
        })
        .all(|x| x)
}

fn put_in_order<'a>(rules: &[(i64, i64)], order: &'a [i64]) -> Vec<&'a i64> {
    let valid_rules: Vec<_> = rules
        .iter()
        .filter(|(a, b)| order.contains(a) && order.contains(b))
        .collect();

    order
        .iter()
        .sorted_by(|&a, &b| {
            valid_rules
                .iter()
                .filter_map(|(x, y)| {
                    if x == a && y == b {
                        Some(std::cmp::Ordering::Less)
                    } else if x == b && y == a {
                        Some(std::cmp::Ordering::Greater)
                    } else {
                        None
                    }
                })
                .exactly_one()
                .unwrap()
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
