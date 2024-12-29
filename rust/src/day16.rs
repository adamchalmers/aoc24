use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;
use priority_queue::PriorityQueue;

use crate::{dir::Dir, grid::Grid, point::Point};
const TURN: usize = 1000;
const FWD: usize = 1;

struct Input {
    grid: Grid<Cell>,
    start: Point,
    end: Point,
    graph: Graph,
}

impl Input {
    fn start(&self) -> State {
        State {
            position: self.start,
            facing: Dir::Right,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
}

/// Priority is higher the lower the cost is.
#[derive(PartialEq, Eq, Copy, Clone)]
struct Priority {
    cost: usize,
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut inner = Vec::with_capacity(width * height);
    let mut start = Point::default();
    let mut end = Point::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            let p = Point::from((x, y));
            let cell = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'S' => {
                    start = p;
                    Cell::Empty
                }
                'E' => {
                    end = p;
                    Cell::Empty
                }
                _ => unreachable!(),
            };
            inner.push(cell);
        }
    }
    let grid = Grid {
        width,
        height,
        inner,
    };
    let graph = make_graph(&grid);
    Input {
        grid,
        start,
        end,
        graph,
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct State {
    position: Point,
    facing: Dir,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    next: State,
    cost: usize,
}

type Graph = HashMap<State, Vec<Edge>>;

fn make_graph(grid: &Grid<Cell>) -> Graph {
    let n = grid.width * grid.height * 4;
    let mut graph: Graph = HashMap::with_capacity_and_hasher(n, Default::default());
    // List all nodes, i.e. cartesian product of
    // - every empty cell
    // - every orientation
    for x in 0..grid.width {
        for y in 0..grid.height {
            let position = Point::from((x, y));
            if grid.get(position) != Some(&Cell::Empty) {
                continue;
            }
            for dir in Dir::all() {
                let curr = State {
                    position,
                    facing: dir,
                };
                // Horse has either 2 or 3 options.
                // It can always turn left, it can always turn right.
                let mut edges = vec![
                    Edge {
                        next: State {
                            facing: curr.facing.to_left(),
                            ..curr
                        },
                        cost: TURN,
                    },
                    Edge {
                        next: State {
                            facing: curr.facing.to_right(),
                            ..curr
                        },
                        cost: TURN,
                    },
                ];
                // The horse can sometimes move forward, if it's facing an empty space.
                let next = curr.position.step_to(dir);
                if grid.get(next) == Some(&Cell::Empty) {
                    edges.push(Edge {
                        next: State {
                            position: next,
                            facing: dir,
                        },
                        cost: FWD,
                    })
                }
                graph.insert(curr, edges);
            }
        }
    }
    graph
}

fn dijkstra(input: &Input, start: State, end: Point) -> (State, usize) {
    // Initialize the data structures.
    let mut visited: HashMap<State, usize> = HashMap::default();
    let mut tentative = PriorityQueue::new();
    let mut solutions = HashMap::default();
    tentative.push(start, Priority { cost: 0 });

    // Start the main loop.
    // Each iteration, get highest-priority item.
    while let Some((curr, priority)) = tentative.pop() {
        let cost = priority.cost;

        // Are we at the final node?
        if curr.position == end {
            solutions.insert(curr, cost);
        }

        if solutions.len() == 4 {
            return solutions
                .into_iter()
                .min_by_key(|(_state, cost)| *cost)
                .unwrap();
        }

        // Check each neighbour of the current node.
        for neighbour in input.graph.get(&curr).unwrap() {
            // Don't visit the same node twice.
            if visited.contains_key(&neighbour.next) {
                continue;
            }

            // Update this neighbour's tentative cost, as the minimum cost to reach it.
            let cost_through_here = cost + neighbour.cost;
            let min_cost = if let Some(previous_cost) =
                tentative.get_priority(&neighbour.next).map(|p| p.cost)
            {
                cost_through_here.min(previous_cost)
            } else {
                cost_through_here
            };
            tentative.push(neighbour.next, Priority { cost: min_cost });
        }

        // Finished with this node.
        visited.insert(curr, cost);
    }
    panic!("Finished all tentative nodes but never found a terminal node")
}

#[aoc(day16, part2)]
fn q2(input: &Input) -> usize {
    let empties: Vec<_> = input
        .grid
        .inner
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| {
            if matches!(cell, Cell::Empty) {
                Some(Point {
                    x: (i % input.grid.width) as isize,
                    y: (i / input.grid.width) as isize,
                })
            } else {
                None
            }
        })
        .collect();
    println!("Found {} empty spaces", empties.len());
    let best_path_cost = dijkstra(input, input.start(), input.end).1;
    empties
        .into_iter()
        .filter(|p| {
            // If this point is part of a best path?
            let p = *p;
            let (p_state, cost_to) = dijkstra(input, input.start(), p);
            if cost_to > best_path_cost {
                return false;
            }
            let cost_from = dijkstra(input, p_state, input.end).1;
            cost_to + cost_from == best_path_cost
        })
        .count()
}

#[aoc(day16, part1)]
fn q1(input: &Input) -> usize {
    dijkstra(input, input.start(), input.end).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_2: &str = "\
    #################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_example() {
        let input = parse(EXAMPLE);
        assert_eq!(q1(&input), 7036);
        assert_eq!(q2(&input), 45);
    }

    #[test]
    fn test_example2() {
        let input = parse(EXAMPLE_2);
        assert_eq!(q1(&input), 11048);
        assert_eq!(q2(&input), 64);
    }
}
