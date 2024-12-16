/*!
# Grid helpers


This module contains some helpers for working with grids.
*/

use grid::Grid;

use core::ops::Add;
use strum::EnumIter;

/// Read a grid of characters from a string.
#[must_use]
pub fn read_char(input: &str) -> Grid<char> {
    let inp: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::from(inp)
}

/// Read a grid of ints from a string.
/// # Panics
/// This function will panic if the input contains a non-digit character.
#[must_use]
pub fn read_int(input: &str) -> Grid<u32> {
    let inp: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from(inp)
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
#[repr(u8)]
pub enum Direction {
    Up = 0x01,
    Right = 0x02,
    Down = 0x04,
    Left = 0x08,
}

impl Direction {
    /// This rotates the direction clockwise.
    #[inline]
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
    #[inline]
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

    /// This reverses the direction.
    #[inline]
    #[must_use]
    pub const fn reverse(&self) -> Self {
        use Direction::{Down, Left, Right, Up};

        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
}

impl Add<Direction> for (i64, i64) {
    type Output = Self;

    #[inline]
    fn add(self, dir: Direction) -> Self::Output {
        use Direction::{Down, Left, Right, Up};

        match dir {
            Up => (self.0 - 1, self.1),
            Left => (self.0, self.1 - 1),
            Down => (self.0 + 1, self.1),
            Right => (self.0, self.1 + 1),
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = Self;

    #[inline]
    fn add(self, dir: Direction) -> Self::Output {
        use Direction::{Down, Left, Right, Up};

        match dir {
            Up => (self.0.checked_sub(1).unwrap(), self.1),
            Left => (self.0, self.1.checked_sub(1).unwrap()),
            Down => (self.0 + 1, self.1),
            Right => (self.0, self.1 + 1),
        }
    }
}

pub fn visualize(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}
