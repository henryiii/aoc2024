/*!
# 2024 Day 23: LAN Party
## Directly connected nodes

<https://adventofcode.com/2024/day/23>

This is a simple, standard graph problem. The only issue with part 2 was that
the algorithm I needed was just in a PR to petgraph, so I had to copy the
contents of the PR (which worked perfectly).
*/

use std::collections::HashSet;

use itertools::Itertools;
use petgraph::prelude::*;

use aoc::maximal_cliques::maximal_cliques;

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

fn intercon<'a>(graph: &UnGraphMap<&'a str, ()>) -> HashSet<Vec<&'a str>> {
    graph
        .nodes()
        .flat_map(|node| {
            let neighbors = graph.neighbors(node);
            neighbors.permutations(2).filter_map(move |v| {
                let (a, b) = v.into_iter().collect_tuple().unwrap();
                if graph.contains_edge(a, b) {
                    let mut res = vec![node, a, b];
                    res.sort_unstable();
                    Some(res)
                } else {
                    None
                }
            })
        })
        .collect()
}

fn solution_a(input: &str) -> Int {
    let pairs = read_input(input);
    let graph: UnGraphMap<&str, ()> =
        GraphMap::from_edges(pairs.iter().map(|(a, b)| (&a[..], &b[..])));
    let sets = intercon(&graph);
    sets.into_iter()
        .filter(|scc| scc.len() == 3 && scc.iter().any(|node| node.starts_with('t')))
        .count()
}

fn solution_b(input: &str) -> String {
    let pairs = read_input(input);
    let graph: UnGraphMap<&str, ()> =
        GraphMap::from_edges(pairs.iter().map(|(a, b)| (&a[..], &b[..])));
    let cliques = maximal_cliques(&graph);
    let max_clique = cliques.iter().max_by_key(|clique| clique.len()).unwrap();
    max_clique.iter().sorted().join(",")
}

pub fn main(_: bool) {
    aoc::run("23", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/23.txt", 7);
    aoc::make_test!("b", "2024/23.txt", "co,de,ka,ta");
}
