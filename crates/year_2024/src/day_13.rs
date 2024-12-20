/*!
# 2024 Day 13: Claw Contraption
## Reaching a final destination

<https://adventofcode.com/2024/day/13>

This is one where a bit of algebra can greatly simplify the problem. After working out the equation
with the determinant, Copilot just wrote it for me. :shrug: (It didn't get the 0 case right, though.)
*/

use std::ops::RangeInclusive;

use aoc_parse::{parser, prelude::*};
use derive_new::new;

#[derive(Debug, new)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn valid(&self, a: i64, b: i64) -> bool {
        let end_point = (a * self.a.0 + b * self.b.0, a * self.a.1 + b * self.b.1);
        end_point == self.prize
    }

    fn chk_all(&self, rng: RangeInclusive<i64>) -> Option<(i64, i64)> {
        rng.map(|a| (a, (self.prize.1 - (self.a.1 * a)) / self.b.1))
            .filter(|(a, b)| self.valid(*a, *b))
            .min_by_key(|(a, b)| a * 3 + b)
    }
    fn chk_analytical(&self) -> Option<(i64, i64)> {
        let det = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        if det == 0 {
            let min = self.prize.1 / self.b.1;
            let max = self.prize.0 / self.a.0;
            return self.chk_all(min..=max);
        }
        let a = (self.prize.0 * self.b.1 - self.prize.1 * self.b.0) / det;
        let b = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0) / det;
        self.valid(a, b).then_some((a, b))
    }
}

fn read_input(input: &str) -> Vec<Machine> {
    parser!(
        sections(
            a:line("Button A: X+" i64 ", Y+" i64)
            b:line("Button B: X+" i64 ", Y+" i64)
            c:line("Prize: X=" i64 ", Y=" i64)
            => Machine::new(a, b, c)
        )
    )
    .parse(input)
    .unwrap()
}

pub fn solution_a(input: &str) -> i64 {
    read_input(input)
        .iter()
        .filter_map(|m| m.chk_all(0..=100).map(|(a, b)| a * 3 + b))
        .sum()
}

pub fn solution_b(input: &str) -> i64 {
    let add = 10_000_000_000_000;
    read_input(input)
        .iter()
        .map(|m| Machine::new(m.a, m.b, (m.prize.0 + add, m.prize.1 + add)))
        .filter_map(|m| m.chk_analytical().map(|(a, b)| a * 3 + b))
        .sum()
}

pub fn main(_: bool) {
    aoc::run("13", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/13.txt", 480);
    aoc::make_test!("b", "2024/13.txt", 875_318_608_908);
}
