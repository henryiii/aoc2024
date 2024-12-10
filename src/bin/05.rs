/*!
# 2024 Day 5: Print Queue
##  Sorting a print queue with order rules

<https://adventofcode.com/2024/day/5>

The first implmementation was slow (100+ ms); collecting a vector of rules sped
it up 20x.
*/

use aoc2024::run;

use itertools::Itertools;

/// Parse the rules from the input. Ignores orders.
fn parse_rules(input: &str) -> Vec<(u64, u64)> {
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

/// Parse the orders from the input. Ignores rules.
fn parse_orders(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect()
}

/// Returns true if the order follows the rules.
fn in_order(rules: &[(u64, u64)], order: &[u64]) -> bool {
    rules
        .iter()
        .filter_map(|(a, b)| {
            Some(order.iter().position(|x| x == a)? < order.iter().position(|x| x == b)?)
        })
        .all(|x| x)
}

/// Reorders the order to follow the rules. Returns a new ordered vector.
fn put_in_order<'a>(rules: &[(u64, u64)], order: &'a [u64]) -> Vec<&'a u64> {
    // Collecting this saves a lot of time when working on the order.
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

fn solution_a(input: &str) -> u64 {
    let rules = parse_rules(input);
    let orders = parse_orders(input);
    orders
        .iter()
        .filter(|order| in_order(&rules, order))
        .map(|order| order[(order.len() - 1) / 2])
        .sum()
}

fn solution_b(input: &str) -> u64 {
    let rules = parse_rules(input);
    let orders = parse_orders(input);
    orders
        .iter()
        .filter(|order| !in_order(&rules, order))
        .map(|order| put_in_order(&rules, order)[(order.len() - 1) / 2])
        .sum()
}
fn main() {
    run("05", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/05.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 143);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 123);
    }
}
