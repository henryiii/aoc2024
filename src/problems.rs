/*!
# Problem helpers

This module has some setup to make each file slightly simpler.
*/

use inline_colorization::{color_blue, color_green, color_reset};
use std::time::Instant;

/// The trait that each day must implement.
pub trait Problem {
    fn solution_a(input: &str) -> i64;
    fn solution_b(input: &str) -> i64;
}

/// Read the input from a file.
fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
}

/// Run the problem. Should be the main function of every day.
pub fn run<T: Problem>(name: &str) {
    let now = Instant::now();
    let input = read_input(&format!("data/{name}.txt"));

    let solution_a = T::solution_a(&input);
    println!("Solution A: {color_blue}{solution_a}{color_reset}");

    let solution_b = T::solution_b(&input);
    println!("Solution B: {color_blue}{solution_b}{color_reset}");

    let time_taken = now.elapsed().as_secs_f64() * 1000.0;

    println!("{color_green}Time taken: {time_taken:.3}ms{color_reset}");
}
