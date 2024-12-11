/*!
# 2024 Day 11: Plutonian Pebbles
##  Splitting numbers

<https://adventofcode.com/2024/day/11>

This is the first "easy" problem that is actually only easy if you are clever.
The trick being to bundle repeated numbers, since you don't need to keep or
comute them multiple times.
*/

use aoc2024::run;

use counter::Counter;

fn read_stones(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn blink(stones: &[usize]) -> Vec<usize> {
    stones
        .iter()
        .flat_map(|&stone| {
            if stone == 0 {
                return vec![1];
            }
            let num_digits = stone.checked_ilog10().unwrap() + 1;
            if num_digits % 2 == 0 {
                return vec![
                    stone / 10usize.pow(num_digits / 2),
                    stone % 10usize.pow(num_digits / 2),
                ];
            }
            vec![stone * 2024]
        })
        .collect()
}

fn blink_counter(stones: &Counter<usize, usize>) -> Counter<usize, usize> {
    let mut counter = Counter::new();
    for (stone, count) in stones {
        let new_stones = blink(&[*stone]);
        for new_stone in new_stones {
            *counter.entry(new_stone).or_insert(0) += count;
        }
    }
    counter
}

fn solution_a(input: &str) -> usize {
    let mut stones = read_stones(input);
    for _ in 0..25 {
        stones = blink(&stones);
    }
    stones.len()
}

fn solution_b(input: &str) -> usize {
    let mut stones: Counter<usize, usize> = read_stones(input).into_iter().collect();

    for _ in 0..75 {
        stones = blink_counter(&stones);
    }
    stones.total()
}

fn main() {
    run("11", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/11.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 55_312);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 65_601_038_650_482);
    }
}
