use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type List = Vec<u64>;

#[aoc_generator(day1)]
fn parse_lists(input: &str) -> (List, List) {
    let (mut l, mut r): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (parse_num(a), parse_num(b)))
        .unzip();
    l.sort_unstable();
    r.sort_unstable();
    (l, r)
}

#[aoc(day1, part1)]
fn part1((l, r): &(List, List)) -> u64 {
    l.iter().zip(r).map(abs_diff).sum()
}

#[aoc(day1, part2)]
fn part2((l, r): &(List, List)) -> u64 {
    let r = frequencies(r);
    l.iter()
        .map(|item| item * r.get(item).copied().unwrap_or_default())
        .sum()
}

fn frequencies(list: &List) -> HashMap<u64, u64> {
    let mut d = HashMap::with_capacity(list.len());
    for id in list {
        *d.entry(*id).or_insert(0) += 1;
    }
    d
}

fn abs_diff((a, b): (&u64, &u64)) -> u64 {
    (*a as i64 - *b as i64).unsigned_abs()
}

fn parse_num(s: &str) -> u64 {
    s.parse().unwrap()
}
