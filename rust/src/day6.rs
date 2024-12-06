use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;

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

impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) @ {:?}",
            self.position.0, self.position.1, self.direction
        )
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    inner: Vec<Vec<IsObstacle>>,
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

impl Grid {
    fn is_in_bounds(&self, point: Point) -> bool {
        let out_of_bounds = point.0 < 0
            || point.1 < 0
            || point.0 >= self.width as isize
            || point.1 >= self.height as isize;
        !out_of_bounds
    }

    fn is_obstacle(&self, point: Point) -> bool {
        if !self.is_in_bounds(point) {
            return false;
        }
        let (x, y) = point;
        self.inner[y as usize][x as usize]
    }
}

/// Returns guard position too.
#[aoc_generator(day6)]
fn parse(input: &str) -> (Grid, Guard) {
    let mut guard = None;
    let inner: Vec<_> = input
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
    let height = inner.len();
    let width = inner[0].len();
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
    let mut positions_visited = HashSet::default();
    positions_visited.insert(guard.position);

    while grid.is_in_bounds(guard.position) {
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
    let mut new_grid = grid.clone();
    let mut choices_for_obstruction = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let p = (x as isize, y as isize);
            if guard.position == p || grid.is_obstacle(p) {
                continue;
            }
            let prev = new_grid.inner[y][x];
            new_grid.inner[y][x] = true;
            if loops(&new_grid, *guard) {
                choices_for_obstruction += 1;
            }
            new_grid.inner[y][x] = prev;
        }
    }
    choices_for_obstruction
}

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
