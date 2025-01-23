/*!
# 2024 Day 18: RAM Run
## Shortest path on grid

<https://adventofcode.com/2024/day/18>

This was trivial with a graph library.
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
    let graph = make_graph(&map);
    let part_point = coords[nbytes..].partition_point(|(x, y)| {
        let mut new_graph = graph.clone();
        for coord in &coords[nbytes..] {
            new_graph.remove_node((coord.0, coord.1));
            if coord == &(*x, *y) {
                break;
            }
        }
        has_path_connecting(&new_graph, (0, 0), (size - 1, size - 1), None)
    });
    let (x, y) = coords[nbytes + part_point];
    format!("{x},{y}")
}

pub fn main(_: bool) {
    aoc::run("18", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/18.txt", 22);
    aoc::make_test!("b", "2024/18.txt", "6,1");
}
