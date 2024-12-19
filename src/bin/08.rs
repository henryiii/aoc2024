/*!
# 2024 Day 8: Resonant Collinearity
## Map frequencies

<https://adventofcode.com/2024/day/8>

Frequencies on a map.
*/

use std::collections::HashSet;
use std::ops::Range;

use grid::Grid;
use itertools::Itertools;

use aoc2024::grid::read_char;

fn solution(map: &Grid<char>, range: Range<usize>) -> usize {
    let mut antinodes: Grid<bool> = Grid::new(map.rows(), map.cols());

    // Every char that is a node
    let chars: HashSet<char> = map.iter().filter(|&v| *v != '.').copied().collect();
    for c in chars {
        // Every node of a char
        let locs: Vec<(i64, i64)> = map
            .indexed_iter()
            .filter(|(_, &ch)| ch == c)
            .map(|((x, y), _)| (x.try_into().unwrap(), y.try_into().unwrap()))
            .collect();

        // Every pair of nodes
        locs.iter().combinations(2).for_each(|v| {
            let (&a, &b) = v.iter().collect_tuple().unwrap();
            let dir = (b.0 - a.0, b.1 - a.1);
            for n in range.clone() {
                let n: i64 = n.try_into().unwrap();
                let pos1 = (a.0 - dir.0 * n, a.1 - dir.1 * n);
                let pos2 = (b.0 + dir.0 * n, b.1 + dir.1 * n);
                if let Some(val) = antinodes.get_mut(pos1.0, pos1.1) {
                    *val = true;
                }
                if let Some(val) = antinodes.get_mut(pos2.0, pos2.1) {
                    *val = true;
                }
            }
        });
    }
    antinodes.iter().filter(|&v| *v).count()
}

fn solution_a(input: &str) -> usize {
    let map = read_char(input);
    solution(&map, 1..2)
}

fn solution_b(input: &str) -> usize {
    let map = read_char(input);
    solution(&map, 0..(map.cols().max(map.rows())))
}

fn main() {
    aoc2024::run("08", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc2024::make_test!("a", "08.txt", 14);
    aoc2024::make_test!("b", "08.txt", 34);
}
