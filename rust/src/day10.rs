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
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    /// Score a trailhead.
    fn score(&self, start: Point, grid: &Grid<Height>) -> usize {
        // Do a DFS and count every 9 we come across.
        let mut discovered = HashSet::default();
        let mut stack = vec![start];
        let mut nines_found = 0;
        while let Some(curr) = stack.pop() {
            let Some(curr_height) = grid.get(curr) else {
                continue;
            };
            if discovered.contains(&curr) {
                continue;
            }
            if *curr_height == 9 {
                nines_found += 1;
            }
            let neighbours = match self.edges.get(&curr) {
                Some(neighbours) => neighbours.as_slice(),
                None => &[],
            };
            stack.extend(neighbours);
            discovered.insert(curr);
        }
        nines_found
    }

    fn rating(&self, start: Point, grid: &Grid<Height>) -> usize {
        // Do a DFS and count every 9 we come across.
        let mut stack = vec![start];
        let mut nines_found = 0;
        while let Some(curr) = stack.pop() {
            let Some(curr_height) = grid.get(curr) else {
                continue;
            };
            if *curr_height == 9 {
                nines_found += 1;
            }
            let neighbours = match self.edges.get(&curr) {
                Some(neighbours) => neighbours.as_slice(),
                None => &[],
            };
            stack.extend(neighbours);
        }
        nines_found
    }
}

fn neighbours_of(Point { x, y }: Point) -> [Point; 4] {
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
    let mut graph = Graph::default();
    let mut trailheads = Vec::default();
    for y in 0..height {
        for x in 0..width {
            let (x, y) = (x as isize, y as isize);
            let point = Point { x, y };

            let this_height = *grid.get(point).unwrap();
            if this_height == 0 {
                trailheads.push(point);
            }
            for neighbour in neighbours_of(point) {
                if let Some(neighbour_height) = grid.get(neighbour) {
                    if *neighbour_height == this_height + 1 {
                        // eprintln!(
                        //     "{}@{} -> {}@{}",
                        //     point, this_height, neighbour, neighbour_height
                        // );
                        graph.insert_edge(point, neighbour);
                    }
                }
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
        .map(|start| input.graph.score(start, &input.grid))
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
