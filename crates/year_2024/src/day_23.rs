/*!
# 2024 Day 23: Sample
## Simple template

<https://adventofcode.com/2024/day/23>

This is a small example to get started, also functions as a template for new days.
*/

use petgraph::{algo::kosaraju_scc, prelude::*};

type Int = usize;

fn read_input(input: &str) -> Vec<(String, String)> {
    use aoc_parse::{parser, prelude::*};

    parser!(
        lines(
            string(alpha+) "-" string(alpha+)
        )
    )
    .parse(input)
    .unwrap()
}

fn interc

fn solution_a(input: &str) -> Int {
    let pairs = read_input(input);
    let graph: UnGraphMap<&str, ()> = GraphMap::from_edges(pairs.iter().map(|(a, b)| (&a[..], &b[..])));
    let sets = kosaraju_scc(&graph);
    println!("{}", sets.len());
    dbg!(&sets);
    sets.into_iter().filter(|scc| scc.len() == 3 && scc.into_iter().any(|node| node.starts_with('t'))).count()
}

fn solution_b(input: &str) -> Int {
    todo!();
}

pub fn main(_: bool) {
    aoc::run("23", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/23.txt", 7);
    aoc::make_test!("b", "2024/23.txt", 0);
}
