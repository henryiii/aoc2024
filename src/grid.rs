use grid::Grid;

#[must_use]
pub fn read_char(input: &str) -> Grid<char> {
    let inp: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::<char>::from(inp)
}

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

pub const XDIRECTIONS: [(i64, i64); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
