/*!
# 2024 Day 9: Disk Fragmenter
## Fragmenter

<https://adventofcode.com/2024/day/9>

Warning: Example does not have side by side movement for part 2.
*/

use derive_new::new;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, new)]
struct Block {
    pub id: usize,
    pub size: usize,
    pub empty: usize,
}

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
        .map(|(i, (a, b))| Block::new(i, *a, *b))
        .collect()
}

fn expand_filesystem(data: &[Block]) -> Vec<Option<usize>> {
    data.iter()
        .flat_map(|block| {
            [Some(block.id)]
                .repeat(block.size)
                .into_iter()
                .chain([None].repeat(block.empty))
        })
        .collect()
}

fn checksum(filesystem_iter: impl Iterator<Item = Option<usize>>) -> usize {
    filesystem_iter
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| v * i))
        .sum()
}

pub fn solution_a(input: &str) -> usize {
    let numbers = read(input);
    let data = mk_data(&numbers);
    let filesystem = expand_filesystem(&data);
    let num_good = filesystem.iter().flatten().count();
    let mut rev_fs = filesystem.iter().rev().flatten();
    let compact = filesystem
        .iter()
        .map(|x| x.map_or_else(|| Some(*rev_fs.next().unwrap()), Some))
        .take(num_good);
    checksum(compact)
}

pub fn solution_b(input: &str) -> usize {
    let numbers = read(input);
    let data = mk_data(&numbers);
    let mut compact = data.clone();
    for block in data.iter().rev() {
        let (orig_pos, &orig) = compact.iter().find_position(|x| x.id == block.id).unwrap();
        if let Some((target_pos, &target)) = compact.iter().find_position(|x| x.empty >= block.size)
        {
            if target_pos < orig_pos {
                compact[orig_pos - 1].empty += orig.size + orig.empty;
                compact[target_pos].empty = 0;
                compact.remove(orig_pos);
                compact.insert(
                    target_pos + 1,
                    Block::new(orig.id, orig.size, target.empty - orig.size),
                );
                if target_pos + 1 == orig_pos {
                    compact[orig_pos].empty += orig.size + orig.empty;
                }
            }
        }
    }
    let filesystem = expand_filesystem(&compact);
    checksum(filesystem.into_iter())
}

pub fn main(_: bool) {
    aoc::run("09", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    aoc::make_test!("a", "2024/09.txt", 1928);
    aoc::make_test!("b", "2024/09.txt", 2858);
}
