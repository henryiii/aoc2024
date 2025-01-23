/*!
# 2024 Day 9: Disk Fragmenter
## Fragmenter

<https://adventofcode.com/2024/day/9>

Warning: Example does not have side by side movement for part 2.

I reworked this to be much faster by holding vectors of `(id, size)` pairs. `VecDeque` is not faster,
due to the fact that removing from the front isn't that common, and the vectors are small.
*/

use derive_new::new;
use itertools::Itertools;

#[derive(Debug, Clone, new)]
struct Block {
    pub id_size: Vec<(usize, usize)>,
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
        .map(|(i, (a, b))| Block::new(vec![(i, *a)], *b))
        .collect()
}

fn expand_filesystem(data: &[Block]) -> Vec<Option<usize>> {
    data.iter()
        .flat_map(|block| {
            block
                .id_size
                .iter()
                .flat_map(|&(id, size)| [Some(id)].repeat(size).into_iter())
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
    let mut data = mk_data(&numbers);
    for i in (0..data.len()).rev() {
        let item = data[i].id_size[0];
        if let Some((pos, _)) = data[0..i].iter().find_position(|x| x.empty >= item.1) {
            data[pos].empty -= item.1;
            data[pos].id_size.push(item);
            data[i].id_size.remove(0);
            if data[i].id_size.is_empty() {
                data[i].empty += item.1;
            } else {
                data[i - 1].empty += item.1;
            }
        }
    }
    let filesystem = expand_filesystem(&data);
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
