use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::Point;

type Input = Vec<Machine>;

const COST_A: usize = 3;
const COST_B: usize = 1;

struct Machine {
    /// How far the claw moves when pressing A.
    a: Point,
    /// How far the claw moves when pressing B.
    b: Point,
    /// Location of the prize.
    prize: Point,
}

impl Machine {
    /// Returns the cheapest way to win. Or None if it's not possible.
    #[allow(non_snake_case)]
    fn min_cost_to_win(&self, add_to_prize: isize) -> Option<usize> {
        // R = location of prize
        let Rx = (self.prize.x + add_to_prize) as isize;
        let Ry = (self.prize.y + add_to_prize) as isize;
        let Ax = self.a.x as isize;
        let Ay = self.a.y as isize;
        let Bx = self.b.x as isize;
        let By = self.b.y as isize;
        let top = Bx * Ry - By * Rx;
        let bot = Ay * Bx - Ax * By;
        if top % bot != 0 {
            return None;
        }
        let Pa = top / bot;
        if (Rx - (Pa * Ax)) % Bx != 0 {
            return None;
        }
        let Pb = (Rx - (Pa * Ax)) / Bx;
        assert_eq!(Ry, Pa * Ay + Pb * By);
        Some(Pa as usize * COST_A + Pb as usize * COST_B)
    }
}

fn parse_two_nums(s: &str) -> (&str, &str) {
    s.split_once(": ").unwrap().1.split_once(", ").unwrap()
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|paragraph| {
            let mut lines = paragraph.trim().lines();
            let (ax, ay) = parse_two_nums(lines.next().unwrap());
            let (bx, by) = parse_two_nums(lines.next().unwrap());
            let (px, py) = parse_two_nums(lines.next().unwrap());
            let [ax, ay, bx, by, px, py] =
                [ax, ay, bx, by, px, py].map(|s| s[2..].parse::<isize>().unwrap());
            Machine {
                a: Point { x: ax, y: ay },
                b: Point { x: bx, y: by },
                prize: Point { x: px, y: py },
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn q1(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|machine| machine.min_cost_to_win(0))
        .sum()
}

#[aoc(day13, part2)]
fn q2(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|machine| machine.min_cost_to_win(10000000000000))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
        );

        let expected = 480;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_real() {
        let input = parse(include_str!("../input/2024/day13.txt"));
        assert_eq!(q1(&input), 35574);
        assert_eq!(q2(&input), 80882098756071);
    }
}
