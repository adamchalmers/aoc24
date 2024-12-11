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
        apply(&mut stones);
    }
    stones.values().sum()
}

fn apply(stones: &mut Input) {
    let old_stones: Vec<_> = stones.drain().collect();
    for (number, count) in old_stones {
        if number == 0 {
            *stones.entry(1).or_default() += count;
        } else if even_num_of_digits(number) {
            let s = number.to_string();
            let l: Num = s[..s.len() / 2].parse().unwrap();
            let r: Num = s[s.len() / 2..].parse().unwrap();
            *stones.entry(l).or_default() += count;
            *stones.entry(r).or_default() += count;
        } else {
            *stones.entry(number * 2024).or_default() += count;
        }
    }
}

fn even_num_of_digits(n: Num) -> bool {
    num_digits(n) % 2 == 0
}
fn num_digits(n: Num) -> u32 {
    n.ilog10() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(111), 3);
    }

    #[test]
    fn test_example() {
        let input = parse("125 17");
        let expected = 55312;
        assert_eq!(q1(&input), expected);
    }
}
