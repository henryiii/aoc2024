/*!
# 2024 Day 22: Sample
## Simple template

<https://adventofcode.com/2024/day/22>

This is a small example to get started, also functions as a template for new days.
*/

// 2284 is too high

use itertools::Itertools;

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

fn expand(vals: &[u64]) -> Vec<Vec<u8>> {
    vals.iter()
        .map(|&v| {
            (0..2000)
                .scan(v, |acc, _| {
                    let before = *acc;
                    *acc = evolve(*acc);
                    Some(u8::try_from(before % 10).unwrap())
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

fn solution_b(input: &str) -> u16 {
    let nums = read_input(input);
    let vals = expand(&nums);
    let mut totals = vec![0u16; 19usize.pow(4)];
    let mut seen = vec![false; 19usize.pow(4)];

    for vv in vals {
        seen.fill(false);
        for (aa, bb, cc, dd, ee) in vv.iter().tuple_windows() {
            let a = usize::from(*bb + 9 - *aa);
            let b = usize::from(*cc + 9 - *bb);
            let c = usize::from(*dd + 9 - *cc);
            let d = usize::from(*ee + 9 - *dd);
            let idx = a * 19usize.pow(3) + b * 19usize.pow(2) + c * 19usize.pow(1) + d;
            if !seen[idx] {
                totals[idx] += u16::from(*ee);
                seen[idx] = true;
            }
        }
    }
    totals.into_iter().max().unwrap()
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
}
