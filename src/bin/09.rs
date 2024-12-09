/*!
# 2024 Day 9: Disk Fragmenter
## Fragmenter

<https://adventofcode.com/2024/day/9>

Warning: Example does not have side by side movement for part 2.
*/

use aoc2024::run;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Block(usize, usize, usize);

fn read(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn mk_data(numbers: &[usize]) -> Vec<Block> {
    numbers
        .iter()
        .chain(std::iter::once(&0))
        .tuples()
        .enumerate()
        .map(|(i, (a, b))| Block(i, *a, *b))
        .collect()
}

fn expand_filesystem(data: &[Block]) -> Vec<Option<usize>> {
    data.iter()
        .flat_map(|Block(i, a, b)| [Some(*i)].repeat(*a).into_iter().chain([None].repeat(*b)))
        .collect()
}

fn checksum(filesystem_iter: impl Iterator<Item = Option<usize>>) -> usize {
    filesystem_iter
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| v * i))
        .sum()
}

fn solution_a(input: &str) -> usize {
    let numbers = read(input);
    let data: Vec<_> = mk_data(&numbers);
    let filesystem = expand_filesystem(&data);
    let num_good = filesystem.iter().filter(|x| x.is_some()).count();
    let mut rev_fs = filesystem.iter().rev().filter(|x| x.is_some());
    let compact = filesystem
        .iter()
        .map(|x| x.map_or_else(|| Some(rev_fs.next().unwrap().unwrap()), Some))
        .take(num_good);
    checksum(compact)
}

fn solution_b(input: &str) -> usize {
    let numbers = read(input);
    let data = mk_data(&numbers);
    let mut compact = data.clone();
    for Block(i, a, _) in data.iter().rev() {
        let (orig_pos, &orig_val) = compact
            .iter()
            .find_position(|Block(ii, _, _)| i == ii)
            .unwrap();
        if let Some((target_pos, &target_val)) =
            compact.iter().find_position(|Block(_, _, bb)| bb >= a)
        {
            if target_pos < orig_pos {
                compact[orig_pos - 1].2 += orig_val.1 + orig_val.2;
                compact[target_pos].2 = 0;
                compact.remove(orig_pos);
                compact.insert(
                    target_pos + 1,
                    Block(orig_val.0, orig_val.1, target_val.2 - orig_val.1),
                );
                if target_pos + 1 == orig_pos {
                    compact[orig_pos].2 += orig_val.1 + orig_val.2;
                }
            }
        }
    }
    let filesystem = expand_filesystem(&compact);
    checksum(filesystem.into_iter())
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
