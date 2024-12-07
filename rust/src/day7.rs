use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

struct Input {
    equations: Vec<Equation>,
}

struct Equation {
    goal: i64,
    items: Vec<i64>,
}

fn is_solvable(goal: i64, items: &[i64], allow_concat: bool) -> Option<String> {
    let (curr, rest) = items.split_last().unwrap();

    if rest.is_empty() {
        return (goal == *curr).then_some(format!("{curr}"));
    }
    if let Some(eq) = is_solvable(goal - curr, rest, allow_concat) {
        return Some(format!("{eq} + {curr}"));
    }
    if goal % curr == 0 {
        if let Some(eq) = is_solvable(goal / curr, rest, allow_concat) {
            return Some(format!("{eq} * {curr}"));
        }
    }
    if allow_concat {
        let new_goal = goal - curr;
        let tens = 10i64.pow(curr.ilog10() + 1);
        if new_goal % tens == 0 {
            if let Some(eq) = is_solvable(new_goal / tens, rest, allow_concat) {
                return Some(format!("{eq} || {curr}"));
            }
        }
    }
    None
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
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
    Input { equations }
}

#[aoc(day7, part1, Recursive)]
fn q1(input: &Input) -> i64 {
    input
        .equations
        .par_iter()
        .map(|e| {
            if is_solvable(e.goal, &e.items, false).is_some() {
                e.goal
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day7, part2, Recursive)]
fn q2(input: &Input) -> i64 {
    input
        .equations
        .par_iter()
        .map(|e| {
            if is_solvable(e.goal, &e.items, true).is_some() {
                e.goal
            } else {
                0
            }
        })
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

    #[test]
    fn test_bad1() {
        let input = parse("156: 15 6");
        let _eq = input.equations.first().unwrap();
    }

    #[test]
    fn test_bad2() {
        let input = parse("7290: 6 8 6 15");
        let _eq = input.equations.first().unwrap();
    }

    #[test]
    fn test_bad3() {
        let input = parse("192: 17 8 14");
        let _eq = input.equations.first().unwrap();
    }
}
