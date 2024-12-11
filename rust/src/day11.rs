use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;

type Num = u64;
type Input = HashMap<Num, usize>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    let mut m = Input::default();
    for num in input.split_whitespace() {
        let num: u64 = num.parse().unwrap();
        *m.entry(num).or_default() += 1;
    }
    m
}

#[aoc(day11, part1)]
fn q1(input: &Input) -> usize {
    solve(input, 25)
}

#[aoc(day11, part2)]
fn q2(input: &Input) -> usize {
    solve(input, 75)
}

fn solve(input: &Input, n: usize) -> usize {
    let mut stones = input.to_owned();
    for _ in 0..n {
        blink(&mut stones);
    }
    stones.values().sum()
}

fn add_to(m: &mut HashMap<Num, usize>, k: u64, v: usize) {
    *m.entry(k).or_default() += v;
}

fn blink(stones: &mut Input) {
    let old_stones: Vec<_> = stones.drain().collect();
    for (number, count) in old_stones {
        if number == 0 {
            *stones.entry(1).or_default() += count;
            add_to(stones, 1, count);
        } else if even_num_of_digits(number) {
            let (l, r) = split(number);
            add_to(stones, l, count);
            add_to(stones, r, count);
        } else {
            add_to(stones, number * 2024, count);
        }
    }
}

fn even_num_of_digits(n: Num) -> bool {
    num_digits(n) % 2 == 0
}

fn num_digits(n: Num) -> u32 {
    n.ilog10() + 1
}

/// Splits a number down the middle of its digits.
/// E.g.
/// ```
/// assert_eq!(aoc::day11::split(1234), (12, 34));
/// ```
pub fn split(n: Num) -> (Num, Num) {
    let tens = 10u64.pow(num_digits(n) / 2);
    (n / tens, n % tens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(111), 3);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(10299234), (1029, 9234));
        assert_eq!(split(102234), (102, 234));
        assert_eq!(split(1034), (10, 34));
    }

    #[test]
    fn test_example() {
        let input = parse("125 17");
        let expected = 55312;
        assert_eq!(q1(&input), expected);
    }
}
