use crate::{dir::Dir, point::Point};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
}

type Grid = crate::grid::Grid<Cell>;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Cheat {
    start: Point,
    end: Point,
}

struct Input {
    path: Vec<Point>,
    grid: Grid,
    position_along_track: HashMap<Point, usize>,
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
    let position_along_track: HashMap<Point, usize> = path
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();

    // All done!
    Input {
        grid,
        path,
        position_along_track,
    }
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

/// Maps number of picoseconds saved to number of cheats which save that much time.
fn each_cheat_saves(input: &Input, max_cheat_secs: usize) -> HashMap<usize, usize> {
    // Find all possible cheat walls.
    let p = (0..input.grid.height)
        .cartesian_product(0..input.grid.width)
        .map(|(y, x)| Point::from((x, y)));
    let q = (0..input.grid.height)
        .cartesian_product(0..input.grid.width)
        .map(|(y, x)| Point::from((x, y)));
    let position_along_track = &input.position_along_track;
    let cheats = p.cartesian_product(q).filter_map(|(p, q)| {
        let dist = dist_btwn(p, q);
        if dist > max_cheat_secs || dist <= 1 {
            return None;
        }
        Some(
            if position_along_track.get(&p)? < position_along_track.get(&q)? {
                Cheat { start: p, end: q }
            } else {
                Cheat { start: q, end: p }
            },
        )
    });
    let mut saves = HashMap::default();
    let baseline = input.baseline_speed();
    let mut seen = HashSet::default();
    for cheat in cheats {
        if seen.insert(cheat) {
            continue;
        }
        let end = input.position_along_track.get(&cheat.end).unwrap();
        let start = input.position_along_track.get(&cheat.start).unwrap();
        let speed = start + (baseline - end);
        let delta = (baseline - speed) - dist_btwn(cheat.start, cheat.end);
        *saves.entry(delta).or_default() += 1;
    }
    saves
}

fn cheats_over(save_at_least: usize, input: &Input, max_cheat_secs: usize) -> usize {
    each_cheat_saves(input, max_cheat_secs)
        .iter()
        .filter_map(|(delta, freq)| {
            if delta >= &save_at_least {
                Some(freq)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day20, part1)]
fn q1(input: &Input) -> usize {
    cheats_over(100, input, 2)
}

#[aoc(day20, part2)]
fn q2(input: &Input) -> usize {
    let n = cheats_over(100, input, 20);
    assert!(n < 1028193);
    n
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

/*
There are 32 cheats that save 50 picoseconds.
There are 31 cheats that save 52 picoseconds.
There are 29 cheats that save 54 picoseconds.
There are 39 cheats that save 56 picoseconds.
There are 25 cheats that save 58 picoseconds.
There are 23 cheats that save 60 picoseconds.
There are 20 cheats that save 62 picoseconds.
There are 19 cheats that save 64 picoseconds.
There are 12 cheats that save 66 picoseconds.
There are 14 cheats that save 68 picoseconds.
There are 12 cheats that save 70 picoseconds.
There are 22 cheats that save 72 picoseconds.
There are 4 cheats that save 74 picoseconds.
There are 3 cheats that save 76 picoseconds.
*/
