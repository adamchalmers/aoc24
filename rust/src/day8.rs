use crate::point::Point;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

struct Input {
    antennae_pairs: Vec<(Point, Point)>,
    width: isize,
    height: isize,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;
    let mut chars_to_point = HashMap::default();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == '.' {
                    None
                } else {
                    Some((
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        c,
                    ))
                }
            })
        })
        .flatten()
        .for_each(|(point, ch)| {
            chars_to_point.entry(ch).or_insert(Vec::new()).push(point);
        });

    let antennae_pairs: Vec<(Point, Point)> = chars_to_point
        .iter()
        .flat_map(|(_ch, points)| {
            // Build a list of all pairs of different antennae using the same frequency.
            let mut all_pairs = Vec::new();
            for (i, p0) in points.iter().enumerate() {
                for (j, p1) in points.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    all_pairs.push((*p0, *p1))
                }
            }
            all_pairs
        })
        .collect();

    Input {
        antennae_pairs,
        height,
        width,
    }
}

#[aoc(day8, part1)]
fn q1(input: &Input) -> usize {
    let places_with_antinodes: HashSet<Point> = input
        .antennae_pairs
        .iter()
        .copied()
        .flat_map(|(p0, p1)| {
            let d = p1 - p0;
            let anti1 = p1 + d;
            let anti0 = p0 - d;
            [anti0, anti1]
                .into_iter()
                .filter(|point| in_bounds(*point, input.width, input.height))
        })
        .collect();

    places_with_antinodes.len()
}

fn in_bounds(point: Point, width: isize, height: isize) -> bool {
    point.x < width && point.y < height && point.x >= 0 && point.y >= 0
}

#[aoc(day8, part2)]
fn q2(input: &Input) -> usize {
    let places_with_antinodes: HashSet<Point> = input
        .antennae_pairs
        .iter()
        .copied()
        .flat_map(|(p0, p1)| {
            let d = p1 - p0;
            let mut antis = vec![p0, p1];

            let mut dpos = d;
            while in_bounds(p1 + dpos, input.width, input.height) {
                antis.push(p1 + dpos);
                dpos += d;
            }

            let mut dmin = d;
            while in_bounds(p0 - dmin, input.width, input.height) {
                antis.push(p0 - dmin);
                dmin -= d;
            }
            antis
        })
        .collect();

    places_with_antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input = parse(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(q1(&input), 14);
    }

    #[test]
    fn test_q2() {
        let input = parse(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(q2(&input), 34);
    }
}
