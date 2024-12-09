/*!
# 2024 Day 9: Disk Fragmenter
## Fragmenter

<https://adventofcode.com/2024/day/9>

Warning: Example does not have side by side movement for part 2.
*/

use aoc2024::run;
use itertools::Itertools;

fn read(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn solution_a(input: &str) -> usize {
    let numbers = read(input);
    let data: Vec<_> = numbers
        .iter()
        .chain(std::iter::once(&0))
        .tuples()
        .enumerate()
        .map(|(i, (a, b))| (i, *a, *b))
        .collect();
    let filesystem: Vec<_> = data
        .iter()
        .flat_map(|(i, a, b)| [Some(*i)].repeat(*a).into_iter().chain([None].repeat(*b)))
        .collect();
    let num_good = filesystem.iter().filter(|x| x.is_some()).count();
    let mut rev_fs = filesystem.iter().rev().filter(|x| x.is_some());
    let compact = filesystem
        .iter()
        .map(|x| x.map_or_else(|| rev_fs.next().unwrap().unwrap(), |v| v))
        .take(num_good);
    compact.enumerate().map(|(i, x)| i * x).sum()
}

fn solution_b(input: &str) -> usize {
    let numbers = read(input);
    let data: Vec<_> = numbers
        .iter()
        .chain(std::iter::once(&0))
        .tuples()
        .enumerate()
        .map(|(i, (a, b))| (i, *a, *b))
        .collect();
    let mut compact = data.clone();
    for (i, a, _) in data.iter().rev() {
        let (orig_pos, &orig_val) = compact.iter().find_position(|(ii, _, _)| i == ii).unwrap();
        if let Some((target_pos, &target_val)) = compact.iter().find_position(|(_, _, bb)| bb >= a)
        {
            if target_pos < orig_pos {
                compact[orig_pos - 1].2 += orig_val.1 + orig_val.2;
                compact[target_pos].2 = 0;
                compact.remove(orig_pos);
                compact.insert(
                    target_pos + 1,
                    (orig_val.0, orig_val.1, target_val.2 - orig_val.1),
                );
                if target_pos + 1 == orig_pos {
                    compact[orig_pos].2 += orig_val.1 + orig_val.2;
                }
            }
        }
    }
    let filesystem: Vec<_> = compact
        .iter()
        .flat_map(|(i, a, b)| [Some(*i)].repeat(*a).into_iter().chain([None].repeat(*b)))
        .collect();
    filesystem
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| v * i))
        .sum()
}

fn main() {
    run("09", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/09.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 1928);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 2858);
    }
}
