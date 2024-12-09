use aoc_runner_derive::{aoc, aoc_generator};

type Input = ();

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    ()
}

#[aoc(day9, part1)]
fn q1(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse("");
        let expected = 0;
        assert_eq!(q1(&input), expected);
    }
}
