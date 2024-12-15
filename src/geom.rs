/*!
## Geometry helpers

This contains a point class with basic math ops.
*/

use derive_more::{Add, From, Mul};

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
