use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

use crate::dir::Dir;
use crate::point::Point;

type Input = Vec<Point>;

#[aoc_generator(day18)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(',').unwrap();
            Point {
                x: l.parse().unwrap(),
                y: r.parse().unwrap(),
            }
        })
        .collect()
}

type Graph = HashMap<Point, HashSet<Point>>;

fn make_graph(width: usize, num_corrupted: usize, input: &Input) -> Graph {
    // Build the graph
    let mut graph: Graph = HashMap::with_capacity_and_hasher(width * width, Default::default());
    let corrupted: HashSet<_> = input.iter().take(num_corrupted).collect();
    for x in 0..width {
        for y in 0..width {
            let p = Point::from((x, y));
            if corrupted.contains(&p) {
                continue;
            }
            for dir in Dir::all() {
                let next = p.step_to(dir);
                if corrupted.contains(&next)
                    || next.x < 0
                    || next.y < 0
                    || next.x >= width as isize
                    || next.y >= width as isize
                {
                    continue;
                }
                graph.entry(p).or_default().insert(next);
            }
        }
    }
    graph
}

#[aoc(day18, part1)]
fn q1(input: &Input) -> usize {
    let width = 71;
    let num_corrupted = 1024;
    let graph = make_graph(width, num_corrupted, input);
    bfs(&graph, width, Point::default()).unwrap()
}

#[aoc(day18, part2)]
fn q2(input: &Input) -> String {
    let width = 71;
    let num_corrupted = 1024;
    let graph = make_graph(width, num_corrupted, input);
    find_last_block(width, num_corrupted, graph, input)
}

fn find_last_block(width: usize, num_corrupted: usize, mut graph: Graph, input: &Input) -> String {
    for i in 1.. {
        let curr = input[num_corrupted + i];
        for dir in Dir::all() {
            let prev = curr.step_to(dir);
            if let Some(prev_edges) = graph.get_mut(&prev) {
                prev_edges.remove(&curr);
            }
        }
        if bfs(&graph, width, Point::default()).is_none() {
            return curr.to_string();
        }
    }
    panic!("Never found a block which cut off escape");
}

fn bfs(graph: &Graph, width: usize, start: Point) -> Option<usize> {
    // The node we're trying to find a path to.
    let end = Point {
        x: (width - 1) as isize,
        y: (width - 1) as isize,
    };

    // Queue of nodes to explore next.
    let mut queue = VecDeque::with_capacity(graph.len());
    queue.push_back(start);

    // Cost to get to each node -- in this graph, it's the length of the shortest path to this node.
    let mut cost = HashMap::with_capacity_and_hasher(graph.len(), Default::default());
    cost.insert(start, 0);

    // Main loop
    while let Some(curr) = queue.pop_front() {
        let curr_cost = *cost.get(&curr).unwrap();
        if curr == end {
            return Some(curr_cost);
        }
        for neighbour in graph.get(&curr).unwrap() {
            if !cost.contains_key(neighbour) {
                cost.insert(*neighbour, curr_cost + 1);
                queue.push_back(*neighbour);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse(
            "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        );
        let width = 7;
        let num_corrupted = 12;
        let graph = make_graph(width, num_corrupted, &input);
        assert_eq!(bfs(&graph, width, Point::default()), Some(22));
        assert_eq!(
            find_last_block(width, num_corrupted, graph, &input),
            "(6,1)"
        );
    }
}
