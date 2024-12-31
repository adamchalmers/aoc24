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

#[derive(Debug)]
struct Cheat {
    start: Point,
    end: Point,
}

struct Input {
    path: Vec<Point>,
    grid: Grid,
    cheats: Vec<Cheat>,
    path_to_order: HashMap<Point, usize>,
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
    let path_to_order: HashMap<Point, usize> = path
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();

    // Find all possible cheat walls.
    let cheats: Vec<_> = (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| Point::from((x, y)))
        .filter_map(|p| {
            if grid.get(p) != Some(&Cell::Wall) {
                return None;
            }
            let l = p.step_to(Dir::Left);
            let r = p.step_to(Dir::Right);
            let u = p.step_to(Dir::Up);
            let d = p.step_to(Dir::Down);
            if grid.get(l) == Some(&Cell::Empty) && grid.get(r) == Some(&Cell::Empty) {
                return Some(
                    if path_to_order.get(&l).unwrap() < path_to_order.get(&r).unwrap() {
                        Cheat { start: l, end: r }
                    } else {
                        Cheat { start: r, end: l }
                    },
                );
            }
            if grid.get(u) == Some(&Cell::Empty) && grid.get(d) == Some(&Cell::Empty) {
                return Some(
                    if path_to_order.get(&u).unwrap() < path_to_order.get(&d).unwrap() {
                        Cheat { start: u, end: d }
                    } else {
                        Cheat { start: d, end: u }
                    },
                );
            }
            None
        })
        .collect();

    // All done!
    Input {
        grid,
        path,
        cheats,
        path_to_order,
    }
}

impl Input {
    fn baseline_speed(&self) -> usize {
        self.path.len() - 1
    }
}

/// Maps number of picoseconds saved to number of cheats which save that much time.
fn each_cheat_saves(input: &Input) -> HashMap<usize, usize> {
    let mut saves = HashMap::with_capacity_and_hasher(input.cheats.len(), Default::default());
    let baseline = dbg!(input.baseline_speed());
    for cheat in &input.cheats {
        let end = input.path_to_order.get(&cheat.end).unwrap();
        let start = input.path_to_order.get(&cheat.start).unwrap();
        let speed = start + (baseline - end);
        let delta = (baseline - speed) - 2;
        // println!("{cheat:?} => {delta}");
        *saves.entry(delta).or_default() += 1;
    }
    // let mut v: Vec<_> = saves.into_iter().collect();
    // v.sort();
    // v.reverse();
    // v
    saves
}

fn cheats_over(n: usize, input: &Input) -> usize {
    each_cheat_saves(input)
        .iter()
        .filter_map(|(delta, freq)| if delta >= &n { Some(freq) } else { None })
        .sum()
}

#[aoc(day20, part1)]
fn q1(input: &Input) -> usize {
    cheats_over(100, input)
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
        assert_eq!(cheats_over(39, &input), 2);
        assert_eq!(cheats_over(15, &input), 5);
    }
}
