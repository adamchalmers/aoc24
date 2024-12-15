use aoc_runner_derive::{aoc, aoc_generator};
use termion::color;

use crate::point::Point;

type Input = Vec<Robot>;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn parse(s: &str) -> Self {
        let (p, v) = s.split_once(' ').unwrap();
        let (px, py) = p.split_once('=').unwrap().1.split_once(',').unwrap();
        let (vx, vy) = v.split_once('=').unwrap().1.split_once(',').unwrap();
        let [px, py, vx, vy] = [px, py, vx, vy].map(|s| s.parse().unwrap());
        Self {
            position: Point { x: px, y: py },
            velocity: Point { x: vx, y: vy },
        }
    }

    fn step(&mut self, width: usize, height: usize) {
        self.position += self.velocity;
        self.position %= Point {
            x: width as isize,
            y: height as isize,
        };
        if self.position.x < 0 {
            self.position.x += width as isize;
        }
        if self.position.y < 0 {
            self.position.y += height as isize;
        }
    }

    fn quadrant(self, width: usize, height: usize) -> Option<Quadrant> {
        let mw = (width as isize) / 2;
        let mh = (height as isize) / 2;
        let Point { x, y } = self.position;
        if x > mw && y < mh {
            Some(Quadrant::Tr)
        } else if x > mw && y > mh {
            Some(Quadrant::Br)
        } else if x < mw && y < mh {
            Some(Quadrant::Tl)
        } else if x < mw && y > mh {
            Some(Quadrant::Bl)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn debug(robots: &[Robot], width: usize, height: usize) {
    use fxhash::FxHashMap as HashMap;
    let mut positions: HashMap<Point, usize> = HashMap::default();
    for r in robots {
        *positions.entry(r.position).or_default() += 1;
    }
    for y in 0..height {
        for x in 0..width {
            if let Some(count) = positions.get(&Point {
                x: x as isize,
                y: y as isize,
            }) {
                print!(
                    "{}{count}{}",
                    color::Fg(color::Green),
                    color::Fg(color::Reset)
                );
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

enum Quadrant {
    Tl,
    Tr,
    Bl,
    Br,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Input {
    input.lines().map(Robot::parse).collect()
}

#[aoc(day14, part1)]
fn q1(robots: &Input) -> usize {
    let seconds = 100;
    let width = 101;
    let height = 103;
    solve_q1(robots.to_owned(), width, height, seconds)
}

fn solve_q1(robots: Input, width: usize, height: usize, seconds: usize) -> usize {
    let mut robots = robots.to_owned();
    for _ in 0..seconds {
        robots
            .iter_mut()
            .for_each(|r| Robot::step(r, width, height));
    }
    robots
        .into_iter()
        .filter_map(|r| Robot::quadrant(r, width, height))
        .fold(
            [0, 0, 0, 0],
            |[mut tl, mut tr, mut bl, mut br], quadrant| {
                match quadrant {
                    Quadrant::Tl => tl += 1,
                    Quadrant::Tr => tr += 1,
                    Quadrant::Bl => bl += 1,
                    Quadrant::Br => br += 1,
                }

                [tl, tr, bl, br]
            },
        )
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut robot = parse("p=2,4 v=2,-3").pop().unwrap();
        let width = 11;
        let height = 7;
        for _ in 0..6 {
            println!("{}", robot.position);
            debug(&[robot], width, height);
            println!();
            robot.step(width, height);
        }
    }

    #[test]
    fn test_example() {
        let input = parse(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        );
        let expected = 12;
        let width = 11;
        let height = 7;
        debug(&input, width, height);
        assert_eq!(solve_q1(input, width, height, 100), expected);
    }
}
