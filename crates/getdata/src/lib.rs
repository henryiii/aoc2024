use thiserror::Error;

const CARGO_AOC_USER_AGENT: &str = "github.com/henryiii/aoc2024 by henryschreineriii@gmail.com";

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Failed to get input: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to read input: {0}")]
    Io(#[from] std::io::Error),
}

/// Get the input for a given day and year
///
/// # Errors
/// Returns an `InputError` if the input could not be fetched or parsed.
pub fn get_input(day: u8, year: u16, session: &str) -> Result<String, InputError> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let resp = client
        .get(&url)
        .header(reqwest::header::COOKIE, format!("session={session}"))
        .header(reqwest::header::USER_AGENT, CARGO_AOC_USER_AGENT)
        .send()?;
    Ok(resp.text()?)
}
