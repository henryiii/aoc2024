#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::implicit_clone)]

use pyo3::prelude::*;
use seq_macro::seq;

// Generate `day_NN_a` / `day_NN_b` wrappers for every day instead of hand
// writing all 50. `seq!` only pastes the counter as a suffix, so `paste!`
// stitches it into the middle of the identifier.
seq!(N in 01..=25 {
    paste::paste! {
        #[pyfunction]
        fn [< day_~N _a >](input: &str) -> String {
            year_2024::day_~N::solution_a(input).to_string()
        }

        #[pyfunction]
        fn [< day_~N _b >](input: &str) -> String {
            year_2024::day_~N::solution_b(input).to_string()
        }
    }
});

#[pymodule]
fn aoc2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    seq!(N in 01..=25 {
        paste::paste! {
            m.add_function(wrap_pyfunction!([< day_~N _a >], m)?)?;
            m.add_function(wrap_pyfunction!([< day_~N _b >], m)?)?;
        }
    });
    Ok(())
}
