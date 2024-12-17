/*!
# Problem helpers

This module has some setup to make each file slightly simpler.
*/

use inline_colorization::{color_blue, color_green, color_reset};
use std::fmt::Display;
use std::time::Instant;

/// Run the problem. Should be the main function of every day.
///
/// This was originally defined with a `fn(&str) -> T` signture, but
/// traits were introduced as good practice.
///
/// # Panics
/// Panics if the file is not found.
#[allow(clippy::similar_names)]
pub fn run<T1, T2, F1, F2>(name: &str, solution_a: F1, solution_b: F2)
where
    T1: Display,
    T2: Display,
    F1: FnOnce(&str) -> T1,
    F2: FnOnce(&str) -> T2,
{
    let now = Instant::now();
    let input = std::fs::read_to_string(format!("data/{name}.txt")).unwrap();

    let sol_a_time = Instant::now();
    let solution_a = solution_a(&input);
    let sol_a_time = sol_a_time.elapsed().as_secs_f64() * 1000.0;
    println!("Solution A: {color_blue}{solution_a}{color_reset}    ({color_green}{sol_a_time:.3}ms{color_reset})");

    let sol_b_time = Instant::now();
    let solution_b = solution_b(&input);
    let sol_b_time = sol_b_time.elapsed().as_secs_f64() * 1000.0;
    println!("Solution B: {color_blue}{solution_b}{color_reset}    ({color_green}{sol_b_time:.3}ms{color_reset})");

    let time_taken = now.elapsed().as_secs_f64() * 1000.0;

    println!("{color_green}Time taken: {time_taken:.3}ms{color_reset}");
}
