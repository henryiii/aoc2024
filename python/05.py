#!/usr/bin/env uv run --quiet

# /// script
# python-version = ">=3.12"
# ///

# This is based on `cmp_to_key`, which I saw someone mention online.
# I rewrote the Rust solution later to match this. Note there's a huge
# performance gain using a set instead of a list.

from pathlib import Path
from problems import run, test
import functools

DIR = Path(__file__).parent


def parse_rules(input: str) -> frozenset[tuple[int, int]]:
    return frozenset(
        tuple(int(x) for x in line.split("|"))
        for line in input.splitlines()
        if "|" in line
    )


def parse_orders(input: str) -> list[list[int]]:
    return [
        [int(x) for x in line.split(",")] for line in input.splitlines() if "," in line
    ]


def in_order(rules: frozenset[tuple[int, int]], order: list[int]) -> bool:
    return all(
        order.index(rule[0]) < order.index(rule[1])
        for rule in rules
        if rule[0] in order and rule[1] in order
    )


@test("05", 143)
def solution_a(input: str) -> int:
    rules = parse_rules(input)
    orders = parse_orders(input)
    return sum(
        order[(len(order) - 1) // 2] for order in orders if in_order(rules, order)
    )


def put_in_order(rules: frozenset[tuple[int, int]], order: list[int]) -> list[int]:
    return sorted(
        order,
        key=functools.cmp_to_key(
            lambda a, b: -1 if (a, b) in rules else 1 if (b, a) in rules else 0
        ),
    )


@test("05", 123)
def solution_b(input: str) -> int:
    rules = parse_rules(input)
    orders = parse_orders(input)
    return sum(
        put_in_order(rules, order)[(len(order) - 1) // 2]
        for order in orders
        if not in_order(rules, order)
    )


if __name__ == "__main__":
    run("05", solution_a, solution_b)
