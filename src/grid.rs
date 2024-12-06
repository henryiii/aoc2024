/*!
# Grid helpers


This module contains some helpers for working with grids.
*/

use grid::Grid;

use core::ops::{Add, Index, IndexMut};
use derive_more::Constructor;

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
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
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

/// This is a helper for a signed position. You can add a direction to step
/// in that direction. You can try convert to a classic (usize, usize)
/// position.
#[derive(Debug, Constructor, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position(isize, isize);

impl Position {
    /// The row of this position, signed.
    #[must_use]
    pub const fn row(&self) -> isize {
        self.0
    }
    /// The column of this position, signed.
    #[must_use]
    pub const fn col(&self) -> isize {
        self.1
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        use Direction::{Down, Left, Right, Up};

        match dir {
            Up => Self(self.0 - 1, self.1),
            Left => Self(self.0, self.1 - 1),
            Down => Self(self.0 + 1, self.1),
            Right => Self(self.0, self.1 + 1),
        }
    }
}
impl Add<Self> for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl TryFrom<Position> for (usize, usize) {
    type Error = std::num::TryFromIntError;

    fn try_from(pos: Position) -> Result<Self, Self::Error> {
        Ok((usize::try_from(pos.0)?, usize::try_from(pos.1)?))
    }
}
/// Panics if the position is out of bounds.
impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        &self[(
            usize::try_from(pos.0).unwrap(),
            usize::try_from(pos.1).unwrap(),
        )]
    }
}

/// Panics if the position is out of bounds.
impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self[(
            usize::try_from(pos.0).unwrap(),
            usize::try_from(pos.1).unwrap(),
        )]
    }
}
