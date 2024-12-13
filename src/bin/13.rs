/*!
# 2024 Day 13: Sample
## Simple template

<https://adventofcode.com/2024/day/13>

This is a small example to get started, also functions as a template for new days.
*/

use derive_new::new;
use regex::Regex;

#[derive(Debug, new)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn valid(&self, a: usize, b: usize) -> bool {
        let end_point = (a * self.a.0 + b * self.b.0, a * self.a.1 + b * self.b.1);
        end_point == self.prize
    }

    fn chk_all(&self) -> Option<(usize, usize)> {
        (0..=100)
            .flat_map(|a| (0..=100).map(move |b| (a, b)))
            .filter(|(a, b)| self.valid(*a, *b))
            .min_by_key(|(a, b)| a * 3 + b)
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

fn solution_a(input: &str) -> usize {
    let machines = read_input(input);
    machines
        .iter()
        .filter_map(|m| m.chk_all().map(|(a, b)| a * 3 + b))
        .sum()
}

fn solution_b(input: &str) -> usize {
    0
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
        assert_eq!(super::solution_b(INPUT), 1);
    }
}
