/*!
# 2024 All days runner
## Run every day and report timings

Use this to run the entire year's solutions and report timings.
*/

use inline_colorization::{style_bold, style_reset};
use std::process::Command;

fn main() {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to build");

    for day_int in 1..=25 {
        let day = format!("{day_int:02}");
        let prog = format!("target/release/{day}");
        println!("{style_bold}Day {day}{style_reset}:");
        let _ = Command::new(&prog).status();
    }
}
