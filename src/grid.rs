/*!
# Grid helpers


This module contains some helpers for working with grids.
*/

use grid::Grid;

use core::ops::Add;

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

/// This is a direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    Up = 0x01,
    Down = 0x02,
    Left = 0x04,
    Right = 0x08,
}

impl Direction {
    /// This rotates the direction clockwise.
    #[must_use]
    pub const fn clockwise(&self) -> Self {
        use Direction::{Down, Left, Right, Up};

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// This rotates the direction counter-clockwise.
    #[must_use]
    pub const fn counter_clockwise(&self) -> Self {
        use Direction::{Down, Left, Right, Up};

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

impl Add<Direction> for (i64, i64) {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        use Direction::{Down, Left, Right, Up};

        match dir {
            Up => (self.0 - 1, self.1),
            Left => (self.0, self.1 - 1),
            Down => (self.0 + 1, self.1),
            Right => (self.0, self.1 + 1),
        }
    }
}
