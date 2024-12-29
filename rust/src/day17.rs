use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    registers: Registers,
    program: Vec<u64>,
}

fn f(s: Option<&str>, pref: &'static str) -> u64 {
    s.unwrap()
        .strip_prefix(pref)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let a = f(lines.next(), "Register A: ");
    let b = f(lines.next(), "Register B: ");
    let c = f(lines.next(), "Register C: ");
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    Input {
        registers: Registers {
            a,
            b,
            c,
            output: Vec::new(),
        },
        program,
    }
}

#[derive(Clone, Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
    output: Vec<u64>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Operand(u64);

impl Operand {
    fn literal(self) -> u64 {
        self.0
    }
    fn combo(self, registers: &Registers) -> u64 {
        match self.0 {
            // Combo operands 0 through 3 represent literal values 0 through 3.
            0..=3 => self.0,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => unreachable!("Combo operand 7 is reserved and will not appear in valid programs."),
        }
    }
}

enum Effect {
    SetIp(usize),
}

/*
Register A: 60589763
Register B: 0
Register C: 0

Program:
2,4  B = A%8
1,5  B ^= 101
7,5  C = A / (2**B)
1,6  B ^= 110
4,1  B ^= C
5,5  Out B % 8
0,3  A /= 2**C
3,0  Loop back to start
*/

impl Opcode {
    fn run(&self, registers: &mut Registers, operand: Operand) -> Option<Effect> {
        match self {
            Opcode::Adv => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                let numerator = registers.a;
                let denominator = pow2(operand, registers);
                registers.a = numerator / denominator;
                // println!("A = A / (2 ** combo({}))", operand.0);
            }
            Opcode::Bxl => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                registers.b ^= operand.literal();
                // println!("B ^= {:b}", operand.literal());
            }
            Opcode::Bst => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                registers.b = operand.combo(registers) % 8;
                // println!("B = combo({}) % 8", operand.0);
            }
            Opcode::Jnz => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if registers.a != 0 {
                    let new_ip = operand.literal().try_into().unwrap();
                    //     println!("IP = {:b}", operand.literal());
                    return Some(Effect::SetIp(new_ip));
                }
                // println!("JNZ ignored");
            }
            Opcode::Bxc => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                registers.b ^= registers.c;
                // println!("B ^= C");
            }
            Opcode::Out => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                registers.output.push(operand.combo(registers) % 8);
                // println!("Out combo({}) % 8", operand.0)
            }
            Opcode::Bdv => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let numerator = registers.a;
                let denominator = pow2(operand, registers);
                registers.b = numerator / denominator;
                // println!("B = A / (2 ** combo({}))", operand.0);
            }
            Opcode::Cdv => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let numerator = registers.a;
                let denominator = pow2(operand, registers);
                registers.c = numerator / denominator;
                // println!("C = A / (2 ** combo({}))", operand.0);
            }
        }
        None
    }
}

fn pow2(operand: Operand, registers: &Registers) -> u64 {
    2u64.pow(operand.combo(registers).try_into().unwrap())
}

fn run(mut registers: Registers, program: &[u64], stop_early: bool) -> String {
    // Run the program.
    let mut ip = 0;
    while ip < program.len() {
        let opcode = Opcode::from(program[ip]);
        let operand = Operand::from(program[ip + 1]);
        match opcode.run(&mut registers, operand) {
            Some(Effect::SetIp(new_ip)) => ip = new_ip,
            None => ip += 2,
        }
        if stop_early && !program.starts_with(&registers.output) {
            return String::new();
        }
    }

    // Join the output.
    registers
        .output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part1)]
fn q1(input: &Input) -> String {
    run(input.registers.clone(), &input.program, false)
}

#[aoc(day17, part2)]
fn q2(input: &Input) -> u64 {
    use rayon::prelude::*;
    let goal = input
        .program
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    (0..u64::MAX)
        .into_par_iter()
        .find_first(|i| {
            let reg = Registers {
                a: *i,
                ..input.registers.clone()
            };
            run(reg, &input.program, true) == goal
        })
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Opcode {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_example() {
        let input = parse(EXAMPLE);
        let expected = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_example_q2() {
        let input = parse(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        );
        let expected = 117440;
        assert_eq!(q2(&input), expected);
    }
}
