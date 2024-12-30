use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    available: HashSet<String>,
    targets: Vec<String>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let (available, targets) = input.split_once("\n\n").unwrap();
    let available = available
        .split(", ")
        .map(|str| str.chars().collect())
        .collect();
    let targets = targets.lines().map(|str| str.chars().collect()).collect();
    Input { available, targets }
}

fn ways_to_make(target: &str, available: &HashSet<String>) -> usize {
    // Definition: the `n-prefix` of a string is its prefix of length `n`.
    //
    // How many ways can you make the n-prefix of `target`?
    // Only 1 way to make the empty string (0-prefix).
    let mut possible = vec![1];

    // Only 1 way at most to make the 1-prefix.
    possible.push(if available.contains(&target[..1]) {
        1
    } else {
        0
    });

    for n in 2..=target.len() {
        let n_prefix = &target[..n];
        let mut ways_to_make = 0;

        // One easy way to make this n-prefix is to use it directly from our available strings.
        if available.contains(n_prefix) {
            ways_to_make += 1;
        }

        // Other ways to make this n-prefix:
        //  - Make the 1-prefix and find the remaining suffix of length (n-1) in available
        //  - Make the 2-prefix and find the remaining suffix of length (n-2) in available
        //  - etc etc
        //  - Make the (n-1) prefix and find the remaining suffix of length 1 in available
        for suffix_len in 1..n {
            let prefix_len = n - suffix_len;
            let suffix = &n_prefix[prefix_len..];
            if possible[prefix_len] > 0 && available.contains(suffix) {
                // Because there's `k` ways to make the smaller prefix,
                // and one way to make the smaller suffix,
                // there's also `k` ways to make this n-prefix.
                ways_to_make += possible[prefix_len];
            }
        }
        possible.push(ways_to_make);
    }
    possible[target.len()]
}

#[aoc(day19, part1)]
fn q1(input: &Input) -> usize {
    input
        .targets
        .par_iter()
        .filter(|target| ways_to_make(target, &input.available) > 0)
        .count()
}

#[aoc(day19, part2)]
fn q2(input: &Input) -> usize {
    input
        .targets
        .par_iter()
        .map(|target| ways_to_make(target, &input.available))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        assert_eq!(q1(&input), 6);
        assert_eq!(q2(&input), 16);
    }
}
