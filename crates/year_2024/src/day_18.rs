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
    GraphMap::from_edges(map.indexed_iter().filter(|(_, valid)| !**valid).flat_map(
        |((y, x), _)| {
            [(0, 1), (1, 0)].iter().filter_map(move |(dy, dx)| {
                let (ny, nx) = (y + dy, x + dx);
                if map.get(ny, nx) == Some(&false) {
                    Some(((x, y), (nx, ny)))
                } else {
                    None
                }
            })
        },
    ))
}

pub fn solution_a(input: &str) -> usize {
    let coords = read_input(input);
    let size = if coords.len() < 1000 { 7 } else { 71 };
    let nbytes = if coords.len() < 1000 { 12 } else { 1024 };
    let map = make_grid(&coords[0..nbytes], size);
    let graph = make_graph(&map);
    let costs = dijkstra(&graph, (0, 0), Some((size - 1, size - 1)), |_| 1);
    *costs.get(&(size - 1, size - 1)).unwrap()
}

pub fn solution_b(input: &str) -> String {
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

pub fn main(_: bool) {
    aoc2024::run("18", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc2024::make_test!("a", "2024/18.txt", 22);
    aoc2024::make_test!("b", "2024/18.txt", "6,1");
}
