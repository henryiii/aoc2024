#![doc = include_str!("../README.md")]
/*!
## aoc2024 crate

This contains things that might get used multiple times.
*/

pub mod grid;
pub mod problems;

pub use problems::run;
