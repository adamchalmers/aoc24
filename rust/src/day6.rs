use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;

type IsObstacle = bool;
type Point = (isize, isize);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    position: Point,
    direction: Dir,
}

impl Guard {
    fn is_facing_obstacle(&self, grid: &Grid) -> bool {
        let in_front = self.direction.step_from(self.position);
        grid.is_obstacle(in_front)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    /// Take a step from `curr` in this direction.
    fn step_from(&self, mut curr: Point) -> Point {
        match self {
            Dir::Up => curr.1 -= 1,
            Dir::Down => curr.1 += 1,
            Dir::Left => curr.0 -= 1,
            Dir::Right => curr.0 += 1,
        }
        curr
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    inner: Vec<IsObstacle>,
}

impl Grid {
    fn is_in_bounds(&self, point: Point) -> bool {
        let out_of_bounds = point.0 < 0
            || point.1 < 0
            || point.0 >= self.width as isize
            || point.1 >= self.height as isize;
        !out_of_bounds
    }

    fn set_obstacle(&mut self, point: Point, obstacle: IsObstacle) {
        let (x, y) = point;
        self.inner[y as usize * self.height + x as usize] = obstacle;
    }

    fn is_obstacle(&self, point: Point) -> bool {
        if !self.is_in_bounds(point) {
            return false;
        }
        let (x, y) = point;
        self.inner[y as usize * self.height + x as usize]
    }
}

/// Parse the puzzle input.
#[aoc_generator(day6)]
fn parse(input: &str) -> (Grid, Guard) {
    let mut guard = None;
    let two_d: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '^' {
                        guard = Some((x, y));
                        false
                    } else {
                        ch == '#'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let height = two_d.len();
    let width = two_d[0].len();
    let inner = two_d.into_iter().flatten().collect();
    let guard = guard.unwrap();

    (
        Grid {
            inner,
            height,
            width,
        },
        Guard {
            position: (guard.0 as isize, guard.1 as isize),
            direction: Dir::Up,
        },
    )
}

#[aoc(day6, part1)]
fn q1((grid, mut guard): &(Grid, Guard)) -> usize {
    // Track every position the guard has visited.
    let mut positions_visited = HashSet::default();
    positions_visited.insert(guard.position);

    while grid.is_in_bounds(guard.position) {
        // Advance the guard, turning her if necessary.
        if guard.is_facing_obstacle(grid) {
            guard.direction.turn();
        }
        guard.position = guard.direction.step_from(guard.position);
        positions_visited.insert(guard.position);
    }
    positions_visited.len() - 1
}

#[aoc(day6, part2)]
fn q2((grid, guard): &(Grid, Guard)) -> usize {
    // Iterate over every column and row in the grid,
    // but iterate over rows *in parallel*.
    // This dramatically improves performance on multi-core machines.
    (0..grid.width)
        .into_par_iter()
        .map(|x| {
            let mut new_grid = grid.clone();
            (0..grid.height)
                .filter(|y| guard_loops_at(x, *y, &mut new_grid, guard))
                .count()
        })
        .sum()
}

/// Checks if placing an obstacle at the given (x,y) point will
/// cause the given guard to get stuck in a loop on the given grid.
fn guard_loops_at(x: usize, y: usize, grid: &mut Grid, guard: &Guard) -> bool {
    let p = (x as isize, y as isize);
    // Early termination checks
    if guard.position == p || grid.is_obstacle(p) {
        return false;
    }

    // Place the obstacle in the grid.
    let prev = grid.is_obstacle(p);
    grid.set_obstacle(p, true);
    // Check if guard loops.
    let is_loop = loops(grid, *guard);
    // Reset the grid.
    grid.set_obstacle(p, prev);

    is_loop
}

/// Given this grid, does the guard get stuck in a loop?
fn loops(grid: &Grid, mut guard: Guard) -> bool {
    let mut positions_visited = HashSet::default();
    positions_visited.insert(guard);

    while grid.is_in_bounds(guard.position) {
        while guard.is_facing_obstacle(grid) {
            guard.direction.turn();
        }
        guard.position = guard.direction.step_from(guard.position);
        if !positions_visited.insert(guard) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_q1() {
        let (grid, guard) = parse(TEST_INPUT);
        assert_eq!(q1(&(grid, guard)), 41);
    }

    #[test]
    fn test_q2() {
        let (grid, guard) = parse(TEST_INPUT);
        assert_eq!(q2(&(grid, guard)), 6);
    }
}
