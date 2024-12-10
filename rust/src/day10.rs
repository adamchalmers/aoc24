use crate::grid::Grid;
use crate::point::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

#[derive(Debug, Default)]
struct Graph {
    edges: HashMap<Point, Vec<Point>>,
}

impl Graph {
    fn insert_edge(&mut self, from: Point, to: Point) {
        self.edges.entry(from).or_default().push(to);
    }

    /// Score a trailhead.
    fn score(&self, start: Point, grid: &Grid<Height>, discovered: &mut HashSet<Point>) -> usize {
        // Do a DFS and count every 9 we come across.
        let mut nines_found = 0;
        let Some(curr_height) = grid.get(start) else {
            return 0;
        };
        if discovered.contains(&start) {
            return 0;
        }
        if *curr_height == 9 {
            nines_found += 1;
        }
        let neighbours = match self.edges.get(&start) {
            Some(neighbours) => neighbours.as_slice(),
            None => &[],
        };
        discovered.insert(start);
        for neighbour in neighbours {
            nines_found += self.score(*neighbour, grid, discovered)
        }
        nines_found
    }

    fn rating(&self, start: Point, grid: &Grid<Height>) -> usize {
        // Do a DFS and count every 9 we come across.
        let mut nines_found = 0;
        let Some(start_height) = grid.get(start) else {
            return 0;
        };
        if *start_height == 9 {
            nines_found += 1;
        }
        let neighbours = match self.edges.get(&start) {
            Some(neighbours) => neighbours.as_slice(),
            None => &[],
        };
        for n in neighbours {
            nines_found += self.rating(*n, grid);
        }
        nines_found
    }
}

fn adjacent_to(Point { x, y }: Point) -> [Point; 4] {
    [
        Point { x, y: y - 1 },
        Point { x: x - 1, y },
        Point { x: x + 1, y },
        Point { x, y: y + 1 },
    ]
}

type Height = u8;
struct Input {
    grid: Grid<Height>,
    graph: Graph,
    trailheads: Vec<Point>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    // First, parse the input as a topographic map
    // (i.e. a 2D grid where each cell has an integer height)
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let grid: Vec<Height> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| {
                let Some(digit) = c.to_digit(10) else {
                    return 111;
                };
                digit.try_into().unwrap()
            })
        })
        .collect();
    let grid = Grid {
        inner: grid,
        width,
        height,
    };

    // Then convert that 2D grid into a directed acyclic graph,
    // where paths in the graph are good hiking trails
    // (i.e. edge from M to N means that N is 1 step higher).
    let mut graph = Graph::default();
    let mut trailheads = Vec::default();
    for y in 0..height {
        for x in 0..width {
            // Check for paths from here to any adjacent locations.
            let from = Point {
                x: x as isize,
                y: y as isize,
            };

            let this_height = *grid.get(from).unwrap();
            if this_height == 0 {
                trailheads.push(from);
            }
            for to in adjacent_to(from)
                .into_iter()
                .filter(|to| matches!(grid.get(*to), Some(to) if *to == this_height + 1))
            {
                graph.insert_edge(from, to);
            }
        }
    }

    Input {
        grid,
        graph,
        trailheads,
    }
}

#[aoc(day10, part1)]
fn q1(input: &Input) -> usize {
    // For each zero, do a DFS then find how many 9s it came across.
    input
        .trailheads
        .iter()
        .copied()
        .map(|start| {
            input
                .graph
                .score(start, &input.grid, &mut HashSet::default())
        })
        .sum()
}

#[aoc(day10, part2)]
fn q2(input: &Input) -> usize {
    // For each zero, do a DFS then find how many 9s it came across.
    input
        .trailheads
        .iter()
        .copied()
        .map(|start| input.graph.rating(start, &input.grid))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_big() {
        let input = parse(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(input.trailheads.len(), 9);
        let expected = 36;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_example_tiny0() {
        let input = parse(
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );
        assert_eq!(input.trailheads.len(), 1);
        let expected = 2;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_example_tiny1() {
        let input = parse(
            "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        );
        assert_eq!(input.trailheads.len(), 1);
        let expected = 4;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_example_tiny2() {
        let input = parse(
            "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );
        assert_eq!(input.trailheads.len(), 2);
        let expected = 3;
        assert_eq!(q1(&input), expected);
    }
}
