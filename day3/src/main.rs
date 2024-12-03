use winnow::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let muls = parse(&input, Part::P1);
    let q1: u32 = muls.iter().map(|mul| mul.run()).sum();
    println!("Q1: {q1}");
}

#[derive(Eq, PartialEq, Debug)]
struct Mul(u32, u32);

impl Mul {
    fn run(&self) -> u32 {
        self.0 * self.1
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Part {
    P1,
    P2,
}

fn parse(input: &str, part: Part) -> Vec<Mul> {
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
    fn instr_do(i: &mut &str) -> PResult<()> {
        "do()".map(|_| ()).parse_next(i)
    }
    fn instr_dont(i: &mut &str) -> PResult<()> {
        "don't()".map(|_| ()).parse_next(i)
    }
    let mut enabled = true;
    input
        .chars()
        .enumerate()
        .filter_map(|(i, _char)| {
            // Note, this is inefficient, we should be using winnow to skip over the chars
            // we know don't correspond to an actual Mul instruction.
            let mut suffix = &input[i..];

            if part == Part::P2 {
                // Enable or disable multiplications.
                if instr_do(&mut suffix).is_ok() {
                    enabled = true;
                }
                if instr_dont(&mut suffix).is_ok() {
                    enabled = false;
                }
            }
            // Do multiplications, if enabled and valid.
            if enabled {
                mul(&mut suffix).ok()
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parser() {
        let actual = parse("mul(31,4)", Part::P1);
        assert_eq!(actual, vec![Mul(31, 4)]);
    }

    #[test]
    fn test_q1() {
        let muls = parse(TEST_INPUT, Part::P1);
        let actual: u32 = muls.into_iter().map(|mul| mul.run()).sum();
        assert_eq!(actual, 161);
    }
}
