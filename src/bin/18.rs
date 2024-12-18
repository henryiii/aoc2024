/*!
# 2024 Day 18: RAM Run
## Shortest path on grid

<https://adventofcode.com/2024/day/18>

This was trival with a graph library.
*/

use grid::Grid;
use petgraph::{
    algo::{dijkstra, has_path_connecting},
    prelude::*,
};

type Int = usize;

fn read_input(input: &str) -> Vec<(usize, usize)> {
    use aoc_parse::{parser, prelude::*};

    parser!(
        lines(
            usize "," usize
        )
    )
    .parse(input)
    .unwrap()
}

fn make_grid(coords: &[(usize, usize)], size: usize) -> Grid<bool> {
    let mut map = Grid::new(size, size);
    for (x, y) in coords {
        map[(*y, *x)] = true;
    }
    map
}

fn make_graph(map: &Grid<bool>) -> UnGraphMap<(usize, usize), ()> {
    let mut graph = GraphMap::new();
    for ((y, x), valid) in map.indexed_iter() {
        if !*valid {
            graph.add_node((x, y));
        }
    }
    for ((y, x), valid) in map.indexed_iter() {
        if !*valid {
            for (dy, dx) in &[(0, 1), (1, 0)] {
                let (ny, nx) = (y + dy, x + dx);
                if map.get(ny, nx) == Some(&false) {
                    graph.add_edge((x, y), (nx, ny), ());
                }
            }
        }
    }
    graph
}

fn solution_a(input: &str) -> Int {
    let coords = read_input(input);
    let size = if coords.len() < 1000 { 7 } else { 71 };
    let nbytes = if coords.len() < 1000 { 12 } else { 1024 };
    let map = make_grid(&coords[0..nbytes], size);
    let graph = make_graph(&map);
    let costs = dijkstra(&graph, (0, 0), Some((size - 1, size - 1)), |_| 1);
    *costs.get(&(size - 1, size - 1)).unwrap()
}

fn solution_b(input: &str) -> String {
    let coords = read_input(input);
    let size = if coords.len() < 1000 { 7 } else { 71 };
    let nbytes = if coords.len() < 1000 { 12 } else { 1024 };
    let map = make_grid(&coords[0..nbytes], size);
    let mut graph = make_graph(&map);
    for (x, y) in &coords[nbytes..] {
        graph.remove_node((*x, *y));
        if !has_path_connecting(&graph, (0, 0), (size - 1, size - 1), None) {
            return format!("{x},{y}");
        }
    }
    unreachable!();
}

fn main() {
    aoc2024::run("18", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/18.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 22);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), "6,1".to_string());
    }
}
