use aoc_runner_derive::aoc;
use winnow::{
    ascii::dec_uint,
    combinator::{delimited, separated_pair},
    prelude::*,
};

#[aoc(day3, part1)]
fn q1(input: &str) -> u32 {
    parse(input, Part::P1)
}

#[aoc(day3, part2)]
fn q2(input: &str) -> u32 {
    parse(input, Part::P2)
}

#[derive(Eq, PartialEq, Debug)]
struct Mul((u32, u32));

impl Mul {
    fn run(self) -> u32 {
        self.0 .0 * self.0 .1
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Part {
    P1,
    P2,
}

fn parse(input: &str, part: Part) -> u32 {
    /// Parses something like 123,456
    fn number_pair(i: &mut &str) -> PResult<(u32, u32)> {
        separated_pair(dec_uint, ',', dec_uint).parse_next(i)
    }

    /// Parses a mul instruction like `mul(123, 44)`
    fn mul(i: &mut &str) -> PResult<Mul> {
        delimited("mul(", number_pair, ')').map(Mul).parse_next(i)
    }

    let mut enabled = true;

    let mut i = 0;
    let mut sum_of_product = 0;
    while i < input.len() {
        if part == Part::P2 {
            // Check if we should enable/disable multiplications.
            if input[i..].starts_with("do()") {
                enabled = true;
                i += 4;
                continue;
            }
            if input[i..].starts_with("don't()") {
                enabled = false;
                i += 7;
                continue;
            }
        }
        // Do multiplications, if enabled and one is parsed.
        let mut suffix = &input[i..];
        let n = suffix.len();
        if enabled {
            if let Ok(mul) = mul(&mut suffix) {
                sum_of_product += mul.run();
                let chars_parsed = n - suffix.len();
                i += chars_parsed;
                continue;
            }
        }
        i += 1;
    }
    sum_of_product
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parser() {
        let actual = parse("mul(31,4)", Part::P1);
        assert_eq!(actual, 31 * 4);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../input/2024/day3.txt");

        let actual_q1 = parse(&input, Part::P1);
        let actual_q2 = parse(&input, Part::P2);

        let expected_q1 = 153469856;
        let expected_q2 = 77055967;
        assert_eq!(actual_q1, expected_q1);
        assert_eq!(actual_q2, expected_q2);
    }

    #[test]
    fn test_q1() {
        let actual = parse(TEST_INPUT, Part::P1);
        assert_eq!(actual, 161);
    }

    #[test]
    fn test_q2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let actual = parse(test_input, Part::P2);
        assert_eq!(actual, 48);
    }
}
