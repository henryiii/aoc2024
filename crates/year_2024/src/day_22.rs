/*!
# 2024 Day 22: Sample
## Simple template

<https://adventofcode.com/2024/day/22>

This is a small example to get started, also functions as a template for new days.
*/

// 2284 is too high

use itertools::Itertools;

use aoc::par::prelude::*;

fn read_input(input: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(u64)).parse(input).unwrap()
}

const fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

const fn prune(a: u64) -> u64 {
    a % 16_777_216
}

const fn evolve(secret: u64) -> u64 {
    let step_a = prune(mix(secret, secret * 64));
    let step_b = prune(mix(step_a, step_a / 32));
    prune(mix(step_b, step_b * 2048))
}

fn from_sequence(vals: &[Vec<i8>], a: i8, b: i8, c: i8, d: i8) -> u64 {
    vals.iter()
        .map(|vv| -> u64 {
            vv.iter()
                .tuple_windows()
                .find_map(|(aa, bb, cc, dd, ee)| {
                    let aaa = bb - aa;
                    let bbb = cc - bb;
                    let ccc = dd - cc;
                    let ddd = ee - dd;
                    #[allow(clippy::cast_sign_loss)]
                    (aaa == a && bbb == b && ccc == c && ddd == d).then_some(*ee as u64)
                })
                .unwrap_or_default()
        })
        .sum()
}

fn expand(vals: &[u64]) -> Vec<Vec<i8>> {
    vals.iter()
        .map(|&v| {
            (0..2000)
                .scan(v, |acc, _| {
                    let before = *acc;
                    *acc = evolve(*acc);
                    Some(i8::try_from(before % 10).unwrap())
                })
                .collect()
        })
        .collect()
}

fn solution_a(input: &str) -> u64 {
    let nums = read_input(input);
    nums.into_iter()
        .map(|v| (0..2000).fold(v, |acc, _| evolve(acc)))
        .sum()
}

fn solution_b(input: &str) -> u64 {
    let nums = read_input(input);
    let vals = expand(&nums);
    (0..19u64.pow(4))
        .into_par_iter()
        .progress_count(19u64.pow(4))
        .map(|i| {
            let a = i8::try_from(i / 19u64.pow(3)).unwrap() - 9;
            let b = i8::try_from((i % 19u64.pow(3)) / 19u64.pow(2)).unwrap() - 9;
            let c = i8::try_from((i % 19u64.pow(2)) / 19u64.pow(1)).unwrap() - 9;
            let d = i8::try_from(i % 19u64.pow(1)).unwrap() - 9;
            from_sequence(&vals, a, b, c, d)
        })
        .max()
        .unwrap()
}

pub fn main(_: bool) {
    aoc::run("22", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/22.txt", 37_327_623);
    aoc::make_test!("b", "2024/22b.txt", 23);

    #[test]
    fn one_step() {
        assert_eq!(super::evolve(123), 15_887_950);
    }

    #[test]
    fn check_best() {
        let input = include_str!("../../../samples/2024/22b.txt");
        let vals = super::read_input(input);
        let expanded = super::expand(&vals);
        assert_eq!(super::from_sequence(&expanded, -2, 1, -1, 3), 23);
    }

    #[test]
    fn check_not_better() {
        let input = include_str!("../../../samples/2024/22b.txt");
        let vals = super::read_input(input);
        let expanded = super::expand(&vals);
        assert_eq!(super::from_sequence(&expanded, -2, 2, -1, -1), 16);
    }

    #[test]
    fn check_simple() {
        let vals = vec![123];
        let expanded = super::expand(&vals);
        let expanded_1 = vec![expanded[0][0..10].to_vec()];
        assert_eq!(super::from_sequence(&expanded_1, -1, -1, 0, 2), 6);
    }
}
