use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lists = parse_lists(&input);
    assert_eq!(part1(&parse_lists(TEST_INPUT)), 11);
    assert_eq!(part2(parse_lists(TEST_INPUT)), 31);
    println!("{}", part1(&lists));
    println!("{}", part2(lists));
}

type List = Vec<u64>;

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

fn part1((l, r): &(List, List)) -> u64 {
    l.iter().zip(r).map(abs_diff).sum()
}

fn part2((l, r): (List, List)) -> u64 {
    let r = frequencies(r);
    l.into_iter()
        .map(|item| item * r.get(&item).copied().unwrap_or_default())
        .sum()
}

fn frequencies(list: List) -> HashMap<u64, u64> {
    let mut d = HashMap::with_capacity(list.len());
    for id in list {
        *d.entry(id).or_insert(0) += 1;
    }
    d
}

fn abs_diff((a, b): (&u64, &u64)) -> u64 {
    (*a as i64 - *b as i64).unsigned_abs()
}

fn parse_num(s: &str) -> u64 {
    s.parse().unwrap()
}

const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";
