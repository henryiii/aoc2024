/*!
# 2024 Day 13: Claw Contraption
## Reaching a final destination

<https://adventofcode.com/2024/day/13>

This is one where a bit of algebra can greatly simplify the problem. After working out the equation
with the determinant, Copilot just wrote it for me. :shrug: (It didn't get the 0 case right, though.)
*/

use std::ops::RangeInclusive;

use derive_new::new;
use regex::Regex;

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
    let re = Regex::new(
        r"(?s)Button A: X\+([[:digit:]]+), Y\+([[:digit:]]+)
Button B: X\+([[:digit:]]+), Y\+([[:digit:]]+)
Prize: X=([[:digit:]]+), Y=([[:digit:]]+)",
    )
    .unwrap();
    let cap = re.captures_iter(input);
    cap.map(|val| {
        Machine::new(
            (val[1].parse().unwrap(), val[2].parse().unwrap()),
            (val[3].parse().unwrap(), val[4].parse().unwrap()),
            (val[5].parse().unwrap(), val[6].parse().unwrap()),
        )
    })
    .collect()
}

fn solution_a(input: &str) -> i64 {
    read_input(input)
        .iter()
        .filter_map(|m| m.chk_all(0..=100).map(|(a, b)| a * 3 + b))
        .sum()
}

fn solution_b(input: &str) -> i64 {
    let add = 10_000_000_000_000;
    read_input(input)
        .iter()
        .map(|m| Machine::new(m.a, m.b, (m.prize.0 + add, m.prize.1 + add)))
        .filter_map(|m| m.chk_analytical().map(|(a, b)| a * 3 + b))
        .sum()
}

fn main() {
    aoc2024::run("13", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/13.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 480);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 875_318_608_908);
    }
}
