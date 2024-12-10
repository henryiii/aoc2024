/*!
## Parallel utilities

This contains a simple wrapper on Rayon to allow it to be optional. Only the
required bits are wrapped, currently `.into_par_iter()` and `.par_iter()`.

Use by changing `rayon::prelude::*` to `aoc2024::par::prelude::*`.
*/

pub trait IntoParallelIterator {
    type Iter: Iterator<Item = Self::Item>;
    type Item: Send;
    fn into_par_iter(self) -> Self::Iter;
}

pub trait IntoParallelRefIterator<'data> {
    type Iter: Iterator<Item = Self::Item>;
    type Item: 'data;
    fn par_iter(&'data self) -> Self::Iter;
}

impl<'data, T: Sync + 'data> IntoParallelIterator for &'data [T] {
    type Item = &'data T;
    type Iter = std::slice::Iter<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        self.iter()
    }
}

impl<'data, T: Sync + 'data> IntoParallelIterator for &'data Vec<T> {
    type Item = &'data T;
    type Iter = std::slice::Iter<'data, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&[T]>::into_par_iter(self)
    }
}

impl<'data, I: 'data + ?Sized> IntoParallelRefIterator<'data> for I
where
    &'data I: IntoParallelIterator,
{
    type Iter = <&'data I as IntoParallelIterator>::Iter;
    type Item = <&'data I as IntoParallelIterator>::Item;

    fn par_iter(&'data self) -> Self::Iter {
        self.into_par_iter()
    }
}

pub mod prelude {
    #[cfg(feature = "par")]
    pub use rayon::prelude::*;

    #[cfg(not(feature = "par"))]
    pub use super::IntoParallelIterator;

    #[cfg(not(feature = "par"))]
    pub use super::IntoParallelRefIterator;
}
