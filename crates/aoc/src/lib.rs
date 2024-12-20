#![doc = include_str!("../../../README.md")]
/*!
## aoc crate

This contains things that might get used multiple times.
*/

pub mod geom;
pub mod grid;
pub mod macros;
pub mod par;
pub mod problems;

pub use problems::run;
