#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

use seq_macro::seq;
use wasm_bindgen::prelude::*;

seq!(N in 01..=25 {
    #[wasm_bindgen]
    pub fn day_a_~N(input: &str) -> String {
        #[allow(clippy::redundant_clone)]
        year_2024::day_~N::solution_a(input).to_string()
    }

    #[wasm_bindgen]
    pub fn day_b_~N(input: &str) -> String {
        #[allow(clippy::redundant_clone)]
        year_2024::day_~N::solution_b(input).to_string()
    }
});

#[allow(clippy::missing_const_for_fn)]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
