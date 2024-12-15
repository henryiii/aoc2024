/*!
## Geometry helpers

This contains a point class with basic math ops.
*/

use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use derive_more::{Add, From, Mul};

use grid::Grid;

/// A point in 2D space.
#[derive(Debug, From, Clone, Copy, Add, Mul)]
pub struct Point<I>(pub I, pub I);

impl Point<i32> {
    /// Calculate the Ecuidian remainder
    #[inline]
    #[must_use]
    pub const fn rem_euclid(&self, rhs: (i32, i32)) -> Self {
        Self(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

/// Panics if the position is out of bounds.
impl<T, V> Index<Point<V>> for Grid<T>
where
    usize: TryFrom<V>,
    <usize as TryFrom<V>>::Error: Debug,
{
    type Output = T;

    fn index(&self, pos: Point<V>) -> &Self::Output {
        &self[(
            usize::try_from(pos.1).unwrap(),
            usize::try_from(pos.0).unwrap(),
        )]
    }
}

/// Panics if the position is out of bounds.
impl<T, V> IndexMut<Point<V>> for Grid<T>
where
    usize: TryFrom<V>,
    <usize as TryFrom<V>>::Error: Debug,
{
    fn index_mut(&mut self, pos: Point<V>) -> &mut Self::Output {
        &mut self[(
            usize::try_from(pos.1).unwrap(),
            usize::try_from(pos.0).unwrap(),
        )]
    }
}
