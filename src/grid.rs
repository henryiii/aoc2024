/*!
# Grid helpers


This module contains some helpers for working with grids.
*/

use grid::Grid;

/// Read a grid of characters from a string.
#[must_use]
pub fn read_char(input: &str) -> Grid<char> {
    let inp: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::<char>::from(inp)
}

/// All 8 directions.
pub const DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Diagonal directions.
pub const XDIRECTIONS: [(i64, i64); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
