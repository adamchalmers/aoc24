use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;

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

fn can_make(target: &str, available: &HashSet<String>) -> bool {
    // Is it possible to make the prefix of `target` of length n?
    // Trivially true for n=0.
    let mut possible = vec![true];
    // Easy to check for n=1.
    possible.push(available.contains(&target[..1]));

    'n: for n in 2..=target.len() {
        let subtarget = &target[..n];
        assert_eq!(subtarget.len(), n);
        println!("Checking {subtarget}");
        if available.contains(subtarget) {
            println!("\t{subtarget} is available");
            possible.push(true);
            continue;
        }
        // Check all ways to make subtarget.
        // Find them by making sequences of (n-1) <> 1, (n-2) <> 2, ..., 1 <> (n-1)
        for rhs_len in 1..n {
            let lhs_len = n - rhs_len;
            assert_eq!(lhs_len + rhs_len, n);
            let lhs = &subtarget[..lhs_len];
            let rhs = &subtarget[lhs_len..];
            println!("\t{lhs}<>{rhs}");
            if possible[lhs_len] && available.contains(rhs) {
                println!("\tWe can make {lhs} and {rhs} is available!");
                possible.push(true);
                continue 'n;
            }
        }
        println!("\tNot possible");
        possible.push(false);
    }
    possible[target.len()]
}

#[aoc(day19, part1)]
fn q1(input: &Input) -> usize {
    println!("Trying all {}", input.targets.len());
    println!("using {} available patterns", input.available.len());
    input
        .targets
        .iter()
        .enumerate()
        .filter(|(i, target)| {
            println!("{i:3}: {target}");
            let b = can_make(target, &input.available);
            println!("     {b}");
            b
        })
        .count()
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
        let expected = 6;
        assert_eq!(q1(&input), expected);
    }
}
