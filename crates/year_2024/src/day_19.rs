/*!
# 2024 Day 19: Linen Layout
## Pattern matching

<https://adventofcode.com/2024/day/19>

The key to this is caching. You could also do it by tracking lengths; once a
specific lencgth has been seen, you don't need to match it again. But I'm using
a cache (like Python's `functools.lru_cache`) to store the results to make it simple.

You could use `count_match` for both parts, which makes part 1 take a long as part 2,
but then part 2 is 60x faster, as it's just reading the cache. But it's not really
acturate for how long it really takes, and this has a faster part 1, so doing it this way.
*/

use cached::proc_macro::cached;

type Int = usize;

fn read_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    use aoc_parse::{parser, prelude::*};

    parser!(
        section(line(repeat_sep(alpha+, ", ")))
        section(lines(alpha+))
    )
    .parse(input)
    .unwrap()
}

#[cached(key = "Vec<char>", convert = "{Vec::from(line)}")]
fn has_match(patterns: &[Vec<char>], line: &[char]) -> bool {
    patterns.iter().any(|pattern| {
        line.strip_prefix(&pattern[..]).map_or(false, |slice| {
            slice.is_empty() || has_match(patterns, slice)
        })
    })
}

#[cached(key = "Vec<char>", convert = "{Vec::from(line)}")]
fn count_match(patterns: &[Vec<char>], line: &[char]) -> Int {
    patterns
        .iter()
        .map(|pattern| {
            line.strip_prefix(&pattern[..]).map_or(0, |slice| {
                if slice.is_empty() {
                    1
                } else {
                    count_match(patterns, slice)
                }
            })
        })
        .sum()
}

pub fn solution_a(input: &str) -> Int {
    let (patterns, lines) = read_input(input);
    lines
        .iter()
        .filter(|line| has_match(&patterns, line))
        .count()
}

pub fn solution_b(input: &str) -> Int {
    let (patterns, lines) = read_input(input);
    lines.iter().map(|line| count_match(&patterns, line)).sum()
}

pub fn main(_: bool) {
    aoc2024::run("19", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc2024::make_test!("a", "2024/19.txt", 6);
    aoc2024::make_test!("b", "2024/19.txt", 16);
}
