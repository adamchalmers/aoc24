use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

struct Equation {
    goal: u64,
    items: Vec<u64>,
}

fn is_solvable(goal: u64, items: &[u64], allow_concat: bool) -> bool {
    // Base case.
    let Some((curr, rest)) = items.split_last() else {
        return goal == 0;
    };

    // Three recursive cases:
    // 1. Using + operation
    is_solvable(goal - curr, rest, allow_concat)
    // 2. Using * operation
    || goal % curr == 0 && is_solvable(goal / curr, rest, allow_concat)
    // 3. Using || operation
    || {
        let new_goal = goal - curr;
        let tens = 10u64.pow(curr.ilog10() + 1);
        allow_concat && new_goal % tens == 0 && is_solvable(new_goal / tens, rest, allow_concat)
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    let equations = input
        .par_lines()
        .map(|line| {
            let (goal, nums) = line.split_once(": ").unwrap();
            let items = nums.split(' ').map(|num| num.parse().unwrap()).collect();
            Equation {
                goal: goal.parse().unwrap(),
                items,
            }
        })
        .collect();
    equations
}

#[aoc(day7, part1, Recursive)]
fn q1(input: &[Equation]) -> u64 {
    input
        .par_iter()
        .filter_map(|e| is_solvable(e.goal, &e.items, false).then_some(e.goal))
        .sum()
}

#[aoc(day7, part2, Recursive)]
fn q2(input: &[Equation]) -> u64 {
    input
        .par_iter()
        .filter_map(|e| is_solvable(e.goal, &e.items, true).then_some(e.goal))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_q1() {
        let input = parse(TEST_INPUT);
        assert_eq!(q1(&input), 3749);
    }

    #[test]
    fn test_q2() {
        let input = parse(TEST_INPUT);
        assert_eq!(q2(&input), 11387);
    }
}
