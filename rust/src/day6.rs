use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;

type IsObstacle = bool;
type Point = (isize, isize);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    position: Point,
    direction: Dir,
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
    fn turn(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
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
        let space_in_front_of_guard = guard.direction.step_from(guard.position);
        let guard_facing_obstacle = grid.is_obstacle(space_in_front_of_guard);
        if guard_facing_obstacle {
            guard.direction = guard.direction.turn();
        }
        guard.position = guard.direction.step_from(guard.position);
        positions_visited.insert(guard.position);
    }
    positions_visited.len() - 1
}

#[aoc(day6, part2)]
fn q2((grid, guard): &(Grid, Guard)) -> usize {
    let mut choices_for_obstruction = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            // for x in 6..7 {
            //     for y in 7..8 {
            let p = (x as isize, y as isize);
            if guard.position == p || grid.is_obstacle(p) {
                continue;
            }
            let mut new_grid = grid.clone();
            new_grid.inner[y][x] = true;
            if loops(&new_grid, *guard) {
                eprintln!("Placing obstacle at ({},{}) causes a loop", x, y);
                choices_for_obstruction += 1;
            }
        }
    }
    choices_for_obstruction
}

fn loops(grid: &Grid, mut guard: Guard) -> bool {
    let mut positions_visited = HashSet::default();
    positions_visited.insert(guard);

    while grid.is_in_bounds(guard.position) {
        let space_in_front_of_guard = guard.direction.step_from(guard.position);
        let guard_facing_obstacle = grid.is_obstacle(space_in_front_of_guard);
        if guard_facing_obstacle {
            guard.direction = guard.direction.turn();
        }
        guard.position = guard.direction.step_from(guard.position);
        if !positions_visited.insert(guard) {
            eprintln!("Guard started a loop at {guard:?}");
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
