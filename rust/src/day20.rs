use crate::{dir::Dir, point::Point};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
}

type Grid = crate::grid::Grid<Cell>;

#[derive(Debug, Eq, Clone, Copy)]
struct Cheat {
    start: Point,
    end: Point,
    start_ord: usize,
    end_ord: usize,
    dist: usize,
}

impl std::hash::Hash for Cheat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // These are the only fields that matter for uniquely identifying a hash.
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialEq for Cheat {
    fn eq(&self, other: &Self) -> bool {
        // These are the only fields that matter for uniquely identifying a hash.
        self.start == other.start && self.end == other.end
    }
}

struct Input {
    path: Vec<Point>,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    // First, parse 2D grid.
    let mut inner = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut start = Point::default();
    let mut end = Point::default();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let p = Point::from((x, y));
            match ch {
                '.' => {
                    inner.push(Cell::Empty);
                }
                '#' => inner.push(Cell::Wall),
                'S' => {
                    start = p;
                    inner.push(Cell::Empty);
                }
                'E' => {
                    end = p;
                    inner.push(Cell::Empty);
                }
                other => unreachable!("weird char {other}"),
            }
        }
    }
    let grid = Grid {
        width,
        height,
        inner,
    };

    // Find the singular path through the grid.
    let mut seen = HashSet::with_capacity_and_hasher(width * height, Default::default());
    let mut stack = vec![start];
    let mut path = Vec::with_capacity(width * height);
    while let Some(curr) = stack.pop() {
        path.push(curr);
        seen.insert(curr);
        for dir in Dir::all() {
            let next = curr.step_to(dir);
            if !seen.contains(&next) && grid.get_unchecked(next) == &Cell::Empty {
                stack.push(next);
            }
        }
    }
    assert_eq!(&start, path.first().unwrap());
    assert_eq!(&end, path.last().unwrap());

    Input { path }
}

impl Input {
    fn baseline_speed(&self) -> usize {
        self.path.len() - 1
    }
}

fn dist_btwn(p: Point, q: Point) -> usize {
    let d = (p.x - q.x).abs() + (p.y - q.y).abs();
    d as usize
}

fn cheats_over(save_at_least: usize, input: &Input, max_cheat_length: usize) -> usize {
    // Find all possible cheats. Store in a HashSet to deduplicate them.
    let cheats: HashSet<_> = (0..input.path.len())
        .cartesian_product(0..input.path.len())
        .filter_map(|(i, j)| {
            let p = input.path[i];
            let q = input.path[j];
            let dist = dist_btwn(p, q);
            if dist > max_cheat_length || dist <= 1 {
                return None;
            }
            Some(if i < j {
                Cheat {
                    start: p,
                    end: q,
                    start_ord: i,
                    end_ord: j,
                    dist,
                }
            } else {
                Cheat {
                    start: q,
                    end: p,
                    start_ord: j,
                    end_ord: i,
                    dist,
                }
            })
        })
        .collect();

    // Count all cheats that save you enough time.
    let baseline = input.baseline_speed();
    cheats
        .into_iter()
        .filter(|cheat| {
            let end = cheat.end_ord;
            let start = cheat.start_ord;
            let speed = start + (baseline - end);
            let delta = (baseline - speed) - cheat.dist;
            delta >= save_at_least
        })
        .count()
}

#[aoc(day20, part1)]
fn q1(input: &Input) -> usize {
    cheats_over(100, input, 2)
}

#[aoc(day20, part2)]
fn q2(input: &Input) -> usize {
    cheats_over(100, input, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        );
        assert_eq!(input.baseline_speed(), 84);
        assert_eq!(cheats_over(39, &input, 2), 2);
        assert_eq!(cheats_over(15, &input, 2), 5);
        assert_eq!(cheats_over(73, &input, 20), 7);
    }

    #[test]
    fn test_dist() {
        let p = Point { x: 1, y: 3 };
        let q = Point { x: 3, y: 7 };
        assert_eq!(dist_btwn(p, q), 6);
    }
}
