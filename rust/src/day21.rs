use fxhash::FxHashMap as HashMap;
use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<char>>;

/// What are all 3 robots pointing at?
/// 11 x 5 x 5 = 275 possible values.
#[derive(Default, Hash, Eq, PartialEq, Clone)]
struct State {
    keys_pressed: Vec<char>,
    dpad_outer: Dpad,
    dpad_inner: Dpad,
    keypad: Keypad,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} [{}]",
            self.dpad_outer,
            self.dpad_inner,
            self.keypad,
            self.keys_pressed.iter().copied().collect::<String>()
        )
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Keypad {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    #[default]
    A,
}

macro_rules! b {
    ($c:expr) => {
        Some(Output {
            new_position: $c,
            key_pressed: None,
        })
    };
}

struct Output<S, T> {
    new_position: S,
    key_pressed: Option<T>,
}

impl<S, T> Output<S, T> {
    fn pressed(new_position: S, pressed: T) -> Self {
        Self {
            new_position,
            key_pressed: Some(pressed),
        }
    }
}

impl Dpad {
    fn all_states() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::A].into_iter()
    }

    /*
    The D-pad controlling the robot on this D-pad was pressed.
    What is the new position of the robot's finger?
    And did the robot press any key? If so, which one?

    Layout:
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */
    #[must_use]
    fn update(self, button: Dpad) -> Option<Output<Self, Self>> {
        match (self, button) {
            (Dpad::Up, Dpad::Up) => None,
            (Dpad::Up, Dpad::Down) => b!(Self::Down),
            (Dpad::Up, Dpad::Left) => None,
            (Dpad::Up, Dpad::Right) => b!(Self::A),
            (Dpad::Down, Dpad::Up) => b!(Self::Up),
            (Dpad::Down, Dpad::Down) => None,
            (Dpad::Down, Dpad::Left) => b!(Self::Left),
            (Dpad::Down, Dpad::Right) => b!(Self::Right),
            (Dpad::Left, Dpad::Up) => None,
            (Dpad::Left, Dpad::Down) => None,
            (Dpad::Left, Dpad::Left) => None,
            (Dpad::Left, Dpad::Right) => b!(Self::Down),
            (Dpad::Right, Dpad::Up) => b!(Self::A),
            (Dpad::Right, Dpad::Down) => None,
            (Dpad::Right, Dpad::Left) => b!(Self::Down),
            (Dpad::Right, Dpad::Right) => None,
            (Dpad::A, Dpad::Up) => None,
            (Dpad::A, Dpad::Down) => b!(Self::Right),
            (Dpad::A, Dpad::Left) => b!(Self::Up),
            (Dpad::A, Dpad::Right) => None,
            (_, Dpad::A) => Some(Output::pressed(self, button)),
        }
    }
}

impl Keypad {
    fn to_char(self) -> char {
        match self {
            Keypad::K0 => '0',
            Keypad::K1 => '1',
            Keypad::K2 => '2',
            Keypad::K3 => '3',
            Keypad::K4 => '4',
            Keypad::K5 => '5',
            Keypad::K6 => '6',
            Keypad::K7 => '7',
            Keypad::K8 => '8',
            Keypad::K9 => '9',
            Keypad::A => 'A',
        }
    }

    /// The D-pad controlling the robot on this keypad was pressed.
    /// What is the new position of the robot's finger?
    /// And did the robot press any key? If so, which one?
    ///
    /// Layout:
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+

    #[must_use]
    fn update(self, button: Dpad) -> Option<Output<Self, char>> {
        match (self, button) {
            (Keypad::K0, Dpad::Up) => b!(Self::K2),
            (Keypad::K0, Dpad::Down) => None,
            (Keypad::K0, Dpad::Left) => None,
            (Keypad::K0, Dpad::Right) => b!(Self::A),
            (Keypad::K1, Dpad::Up) => b!(Self::K4),
            (Keypad::K1, Dpad::Down) => None,
            (Keypad::K1, Dpad::Left) => None,
            (Keypad::K1, Dpad::Right) => b!(Self::K2),
            (Keypad::K2, Dpad::Up) => b!(Self::K5),
            (Keypad::K2, Dpad::Down) => b!(Self::K0),
            (Keypad::K2, Dpad::Left) => b!(Self::K1),
            (Keypad::K2, Dpad::Right) => b!(Self::K3),
            (Keypad::K3, Dpad::Up) => b!(Self::K6),
            (Keypad::K3, Dpad::Down) => b!(Self::A),
            (Keypad::K3, Dpad::Left) => b!(Self::K2),
            (Keypad::K3, Dpad::Right) => None,
            (Keypad::K4, Dpad::Up) => b!(Self::K7),
            (Keypad::K4, Dpad::Down) => b!(Self::K1),
            (Keypad::K4, Dpad::Left) => None,
            (Keypad::K4, Dpad::Right) => b!(Self::K5),
            (Keypad::K5, Dpad::Up) => b!(Self::K8),
            (Keypad::K5, Dpad::Down) => b!(Self::K2),
            (Keypad::K5, Dpad::Left) => b!(Self::K4),
            (Keypad::K5, Dpad::Right) => b!(Self::K6),
            (Keypad::K6, Dpad::Up) => b!(Self::K9),
            (Keypad::K6, Dpad::Down) => b!(Self::K3),
            (Keypad::K6, Dpad::Left) => b!(Self::K5),
            (Keypad::K6, Dpad::Right) => None,
            (Keypad::K7, Dpad::Up) => None,
            (Keypad::K7, Dpad::Down) => b!(Self::K4),
            (Keypad::K7, Dpad::Left) => None,
            (Keypad::K7, Dpad::Right) => b!(Self::K8),
            (Keypad::K8, Dpad::Up) => None,
            (Keypad::K8, Dpad::Down) => b!(Self::K5),
            (Keypad::K8, Dpad::Left) => b!(Self::K7),
            (Keypad::K8, Dpad::Right) => b!(Self::K9),
            (Keypad::K9, Dpad::Up) => None,
            (Keypad::K9, Dpad::Down) => b!(Self::K6),
            (Keypad::K9, Dpad::Left) => b!(Self::K8),
            (Keypad::K9, Dpad::Right) => None,
            (Keypad::A, Dpad::Up) => b!(Self::K3),
            (Keypad::A, Dpad::Down) => None,
            (Keypad::A, Dpad::Left) => b!(Self::K0),
            (Keypad::A, Dpad::Right) => None,
            (k, Dpad::A) => Some(Output::pressed(self, k.to_char())),
        }
    }
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Dpad {
    Up,
    Down,
    Left,
    Right,
    #[default]
    A,
}

impl std::fmt::Debug for Dpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Dpad::Up => "U",
            Dpad::Down => "D",
            Dpad::Left => "L",
            Dpad::Right => "R",
            Dpad::A => "A",
        };
        write!(f, "{s}")
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn numeric_part(code: &[char]) -> usize {
    code.iter()
        .rev()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .fold(0, |acc, (i, d)| acc + d * 10_u32.pow(i as u32)) as usize
}

fn bfs(start: State, code: &[char]) -> (usize, State) {
    let mut queue = VecDeque::default();
    queue.push_back((start.clone(), 0));
    let mut explored = HashMap::default();
    explored.insert(start.clone(), ());
    while let Some((state, n)) = queue.pop_front() {
        // Check if we've solved it.
        if state.keys_pressed == code {
            return (n, state);
        }

        // Otherwise check all possible edges from here.
        println!("{state:?}");
        for (choice, new_state) in state.clone().successors(code) {
            // Already explored?
            if explored.contains_key(&new_state) {
                continue;
            }
            // If not, then let's explore it.
            println!("\t-> push {choice:?} -> {new_state:?}");
            queue.push_back((new_state.clone(), n + 1));
            explored.insert(new_state.clone(), ());
        }
    }
    panic!("Never found a path");
}

impl State {
    fn successors(self, code: &[char]) -> Vec<(Dpad, State)> {
        Dpad::all_states()
            .filter_map(|human_choice| {
                let Some(new_state) = clone().advance(human_choice) else {
                    return None;
                };
                if !code.starts_with(new_state.keys()) {
                    return None;
                }
                Some((human_choice, new_state))
            })
            .collect()
    }

    fn keys(&self) -> &[char] {
        &self.keys_pressed
    }

    // Something's wrong on 3, the inner dpad state should have changed.
    fn advance(self, human_choice: Dpad) -> Option<Self> {
        let mut new_state = self.clone();

        // Human pressed a key which controls the outer dpad robot.
        let Output {
            new_position: new_dpad_outer_pos,
            key_pressed: dpad_outer_key_pressed,
        } = self.dpad_outer.update(human_choice)?;
        println!("Human pressed {human_choice:?}, now DpadOuter is at {new_dpad_outer_pos:?}");
        new_state.dpad_outer = new_dpad_outer_pos;

        // Did the outer dpad robot change the inner dpad robot?
        // If not, we can return now.
        let Some(outer_key_pressed) = dpad_outer_key_pressed else {
            return Some(new_state);
        };

        // If so, change the inner dpad robot.
        let Output {
            new_position: new_dpad_inner_pos,
            key_pressed: dpad_inner_key_pressed,
        } = self.dpad_inner.update(outer_key_pressed)?;
        println!(
            "DpadOuter pressed {outer_key_pressed:?}, now DpadInner is at {new_dpad_inner_pos:?}"
        );
        new_state.dpad_inner = new_dpad_inner_pos;

        // Did the inner dpad robot change the keypad robot?
        // If not, we can return now.
        let Some(inner_key_pressed) = dpad_inner_key_pressed else {
            return Some(new_state);
        };

        // If so, change the keyboard robot.
        let Output {
            new_position: new_keypad_pos,
            key_pressed: keypad_key_pressed,
        } = self.keypad.update(inner_key_pressed)?;
        new_state.keypad = new_keypad_pos;

        // Did the keypad robot press anything?
        // If not, we can return now.
        if let Some(key) = keypad_key_pressed {
            new_state.keys_pressed.push(key);
        }

        Some(new_state)
    }
}

#[aoc(day21, part1)]
fn q1(input: &Input) -> usize {
    let mut state = State::default();
    let mut total_complexity = 0;
    for code in input {
        let (path_len, new_state) = bfs(state, code);
        state = new_state;
        state.keys_pressed = Default::default();
        let complexity = numeric_part(code) * path_len;
        total_complexity += complexity;
    }
    total_complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    //     fn test_example() {
    //         let input = parse(
    //             "029A
    // 980A
    // 179A
    // 456A
    // 379A",
    //         );
    //         let expected = 126384;
    //         assert_eq!(q1(&input), expected);
    //     }

    // #[test]
    // fn test_enumerate_states() {
    //     assert_eq!(State::all_states().count(), 275);
    // }

    #[test]
    fn test_numeric_part() {
        assert_eq!(numeric_part(&['0', '2', '9', 'A']), 29);
    }

    #[test]
    fn test_worked_example() {
        let code = "029A";
        let mut state = State::default();
        let human_keys = vec![
            // <vA<AA>>^
            Dpad::Left,
            Dpad::Down,
            Dpad::A,
            // Dpad::Left,
            // Dpad::A,
            // Dpad::A,
            // Dpad::Right,
            // Dpad::Right,
            // Dpad::Up,
        ];
        // let human_keys = vec![
        //     Dpad::Left,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Down,
        //     Dpad::Left,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Down,
        //     Dpad::Left,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::A,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Down,
        //     Dpad::Left,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Down,
        //     Dpad::A,
        //     Dpad::Left,
        //     Dpad::Up,
        //     Dpad::A,
        //     Dpad::Right,
        //     Dpad::A,
        // ];
        println!(" 0: {state:?}");
        for (i, key) in human_keys.into_iter().enumerate() {
            state = state.advance(key).unwrap();
            println!("{:2}: {key:?} -> {state:?}", i + 1);
        }
        assert_eq!(state.keys_pressed, ['0', '2', '9', 'A']);
    }

    #[test]
    fn test_adv() {
        for (state, expected) in [
            (
                State {
                    keys_pressed: Default::default(),
                    dpad_outer: Dpad::A,
                    dpad_inner: Dpad::A,
                    keypad: Keypad::K3,
                },
                Some(State {
                    keys_pressed: vec!['3'],
                    dpad_outer: Dpad::A,
                    dpad_inner: Dpad::A,
                    keypad: Keypad::K3,
                }),
            ),
            (
                Default::default(),
                Some(State {
                    keys_pressed: vec!['A'],
                    ..Default::default()
                }),
            ),
        ] {
            let actual = state.clone().advance(Dpad::A);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_edges() {
        assert_eq!(
            State::default().successors(&['1', '2', '3', '4']),
            vec![
                (
                    Dpad::Down,
                    State {
                        dpad_outer: Dpad::Right,
                        ..Default::default()
                    }
                ),
                (
                    Dpad::Left,
                    State {
                        dpad_outer: Dpad::Up,
                        ..Default::default()
                    }
                )
            ]
        );
    }
}
