/*!
# 2024 Day 11: Plutonian Pebbles
##  Splitting numbers

<https://adventofcode.com/2024/day/11>

This is the first "easy" problem that is actually only easy if you are clever.
The trick being to bundle repeated numbers, since you don't need to keep or
comute them multiple times.
*/

use counter::Counter;

enum NewStones {
    Single(usize),
    Double(usize, usize),
}

fn read_stones(input: &str) -> Counter<usize, usize> {
    input
        .split_ascii_whitespace()
        .map(|val| val.parse::<usize>().unwrap())
        .collect()
}

fn blink(stone: usize) -> NewStones {
    if let Some(num_digits) = stone.checked_ilog10().map(|x| x + 1) {
        if num_digits % 2 == 0 {
            return NewStones::Double(
                stone / 10usize.pow(num_digits / 2),
                stone % 10usize.pow(num_digits / 2),
            );
        }
        return NewStones::Single(stone * 2024);
    }
    NewStones::Single(1)
}

fn blink_counter(stones: &mut Counter<usize, usize>) {
    for (stone, count) in stones.clone() {
        match blink(stone) {
            NewStones::Single(stone_0) => {
                *stones.entry(stone_0).or_insert(0) += count;
            }
            NewStones::Double(stone_1, stone_2) => {
                *stones.entry(stone_1).or_insert(0) += count;
                *stones.entry(stone_2).or_insert(0) += count;
            }
        }
        stones[&stone] -= count;
    }
}

fn solution_a(input: &str) -> usize {
    let mut stones = read_stones(input);
    for _ in 0..25 {
        blink_counter(&mut stones);
    }
    stones.total()
}

fn solution_b(input: &str) -> usize {
    let mut stones = read_stones(input);
    for _ in 0..75 {
        blink_counter(&mut stones);
    }
    stones.total()
}

fn main() {
    aoc2024::run("11", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../samples/11.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(super::solution_a(INPUT), 55_312);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(super::solution_b(INPUT), 65_601_038_650_482);
    }
}
