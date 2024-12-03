use winnow::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let q1 = parse(&input, Part::P1);
    println!("Q1: {q1}");

    let q2 = parse(&input, Part::P2);
    println!("Q2: {q2}");
}

#[derive(Eq, PartialEq, Debug)]
struct Mul(u32, u32);

impl Mul {
    fn run(self) -> u32 {
        self.0 * self.1
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Part {
    P1,
    P2,
}

fn parse(input: &str, part: Part) -> u32 {
    /// Parses the string 'mul('
    fn mul_tag(i: &mut &str) -> PResult<()> {
        "mul(".map(|_| ()).parse_next(i)
    }
    /// Parses a sequence of digits into a number.
    fn numbers(i: &mut &str) -> PResult<u32> {
        winnow::ascii::dec_uint.parse_next(i)
    }
    /// Parses something like 123,456
    fn number_pair(i: &mut &str) -> PResult<(u32, u32)> {
        winnow::combinator::separated_pair(numbers, ',', numbers).parse_next(i)
    }
    fn mul(i: &mut &str) -> PResult<Mul> {
        (mul_tag, number_pair, ')')
            .map(|(_, (a, b), _)| Mul(a, b))
            .parse_next(i)
    }
    fn instr_do(i: &mut &str) -> PResult<usize> {
        "do()".map(|_| 4).parse_next(i)
    }
    fn instr_dont(i: &mut &str) -> PResult<usize> {
        "don't()".map(|_| 7).parse_next(i)
    }
    let mut enabled = true;

    let mut i = 0;
    let mut sum_of_product = 0;
    while i < input.len() {
        if part == Part::P2 {
            // Check if we should enable/disable multiplications.
            if let Ok(len) = instr_do(&mut &input[i..]) {
                enabled = true;
                i += len;
                continue;
            }
            if let Ok(len) = instr_dont(&mut &input[i..]) {
                enabled = false;
                i += len;
                continue;
            }
        }
        // Do multiplications, if enabled and one is parsed.
        let mut suffix = &input[i..];
        let n = suffix.len();
        if enabled {
            if let Some(mul) = mul(&mut suffix).ok() {
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
        let input = std::fs::read_to_string("input").unwrap();

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
