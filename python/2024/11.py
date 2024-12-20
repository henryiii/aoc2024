#!/usr/bin/env python3

# This was based on the Rust version.

from collections.abc import Generator
from problems import run, test
from collections import Counter
import math


def parse(input: str) -> Counter[int]:
    return Counter(int(x) for x in input.split())

def blink(stone: int) -> Generator[int, None, None]:
    if stone == 0:
        yield 1
        return

    num_digits = int(math.log10(stone)) + 1
    if num_digits % 2 == 0:
        yield stone // 10**(num_digits // 2)
        yield stone % 10**(num_digits // 2)
    else:
        yield stone * 2024

def blink_counter(stones: Counter[int]) -> None:
    for stone, count in list(stones.items()):
        if count > 0:
            stones[stone] -= count
            for new_stone in blink(stone):
                stones[new_stone] += count

@test("2024/11", 55_312)
def solution_a(input: str) -> int:
    stones = parse(input)
    for _ in range(25):
        blink_counter(stones)
    return sum(stones.values())


@test("2024/11", 65_601_038_650_482)
def solution_b(input: str) -> int:
    stones = parse(input)
    for _ in range(75):
        blink_counter(stones)
    return sum(stones.values())


if __name__ == "__main__":
    run("2024/11", solution_a, solution_b)
