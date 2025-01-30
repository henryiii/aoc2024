#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_clone)]

use pyo3::prelude::*;

#[pymodule]
pub mod aoc2024 {
    use super::pyfunction;

    #[pyfunction]
    pub fn day_01_a(input: &str) -> String {
        year_2024::day_01::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_01_b(input: &str) -> String {
        year_2024::day_01::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_02_a(input: &str) -> String {
        year_2024::day_02::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_02_b(input: &str) -> String {
        year_2024::day_02::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_03_a(input: &str) -> String {
        year_2024::day_03::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_03_b(input: &str) -> String {
        year_2024::day_03::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_04_a(input: &str) -> String {
        year_2024::day_04::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_04_b(input: &str) -> String {
        year_2024::day_04::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_05_a(input: &str) -> String {
        year_2024::day_05::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_05_b(input: &str) -> String {
        year_2024::day_05::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_06_a(input: &str) -> String {
        year_2024::day_06::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_06_b(input: &str) -> String {
        year_2024::day_06::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_07_a(input: &str) -> String {
        year_2024::day_07::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_07_b(input: &str) -> String {
        year_2024::day_07::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_08_a(input: &str) -> String {
        year_2024::day_08::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_08_b(input: &str) -> String {
        year_2024::day_08::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_09_a(input: &str) -> String {
        year_2024::day_09::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_09_b(input: &str) -> String {
        year_2024::day_09::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_10_a(input: &str) -> String {
        year_2024::day_10::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_10_b(input: &str) -> String {
        year_2024::day_10::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_11_a(input: &str) -> String {
        year_2024::day_11::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_11_b(input: &str) -> String {
        year_2024::day_11::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_12_a(input: &str) -> String {
        year_2024::day_12::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_12_b(input: &str) -> String {
        year_2024::day_12::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_13_a(input: &str) -> String {
        year_2024::day_13::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_13_b(input: &str) -> String {
        year_2024::day_13::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_14_a(input: &str) -> String {
        year_2024::day_14::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_14_b(input: &str) -> String {
        year_2024::day_14::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_15_a(input: &str) -> String {
        year_2024::day_15::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_15_b(input: &str) -> String {
        year_2024::day_15::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_16_a(input: &str) -> String {
        year_2024::day_16::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_16_b(input: &str) -> String {
        year_2024::day_16::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_17_a(input: &str) -> String {
        year_2024::day_17::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_17_b(input: &str) -> String {
        year_2024::day_17::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_18_a(input: &str) -> String {
        year_2024::day_18::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_18_b(input: &str) -> String {
        year_2024::day_18::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_19_a(input: &str) -> String {
        year_2024::day_19::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_19_b(input: &str) -> String {
        year_2024::day_19::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_20_a(input: &str) -> String {
        year_2024::day_20::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_20_b(input: &str) -> String {
        year_2024::day_20::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_21_a(input: &str) -> String {
        year_2024::day_21::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_21_b(input: &str) -> String {
        year_2024::day_21::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_22_a(input: &str) -> String {
        year_2024::day_22::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_22_b(input: &str) -> String {
        year_2024::day_22::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_23_a(input: &str) -> String {
        year_2024::day_23::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_23_b(input: &str) -> String {
        year_2024::day_23::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_24_a(input: &str) -> String {
        year_2024::day_24::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_24_b(input: &str) -> String {
        year_2024::day_24::solution_b(input).to_string()
    }

    #[pyfunction]
    pub fn day_25_a(input: &str) -> String {
        year_2024::day_25::solution_a(input).to_string()
    }
    #[pyfunction]
    pub fn day_25_b(input: &str) -> String {
        year_2024::day_25::solution_b(input).to_string()
    }
}
