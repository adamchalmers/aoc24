use crate::{grid::Grid, point::Point};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;

type Input = Grid<char>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let inner = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    Grid {
        width,
        height,
        inner,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[aoc(day12, part2)]
fn q2(garden_map: &Input) -> usize {
    let Regions {
        cell_to_region,
        regions,
    } = find_regions(garden_map);
    drop(cell_to_region);
    regions
        .par_iter()
        .map(|(_plant, region)| {
            let area = region.len();
            let edges: HashSet<(Point, Dir)> = region
                .iter()
                .flat_map(|cell| {
                    let mut edges = vec![];
                    {
                        let up = cell.up();
                        if !region.contains(&up) {
                            edges.push((*cell, Dir::Up));
                        }
                    }
                    {
                        let down = cell.down();
                        if !region.contains(&down) {
                            edges.push((*cell, Dir::Down));
                        }
                    }
                    {
                        let left = cell.left();
                        if !region.contains(&left) {
                            edges.push((*cell, Dir::Left));
                        }
                    }
                    {
                        let right = cell.right();
                        if !region.contains(&right) {
                            edges.push((*cell, Dir::Right));
                        }
                    }
                    edges
                })
                .collect();
            let sides = sides_from(edges);
            area * sides
        })
        .sum()
}

fn sides_from(edges: HashSet<(Point, Dir)>) -> usize {
    edges
        .iter()
        .filter(|(point, dir)| match dir {
            Dir::Up if !edges.contains(&(point.right(), Dir::Up)) => true,
            Dir::Down if !edges.contains(&(point.right(), Dir::Down)) => true,
            Dir::Left if !edges.contains(&(point.down(), Dir::Left)) => true,
            Dir::Right if !edges.contains(&(point.down(), Dir::Right)) => true,
            _ => false,
        })
        .count()
}

#[aoc(day12, part1)]
fn q1(garden_map: &Input) -> usize {
    let Regions {
        cell_to_region: _,
        regions,
    } = find_regions(garden_map);
    regions
        .par_iter()
        .map(|(_plant, region)| {
            let area = region.len();
            let perimeter: usize = region
                .iter()
                .map(|p| {
                    4 - p
                        .cardinal()
                        .iter()
                        .filter(|dir| region.contains(dir))
                        .count()
                })
                .sum();
            area * perimeter
        })
        .sum()
}

struct Regions {
    cell_to_region: Grid<usize>,
    regions: Vec<(char, Vec<Point>)>,
}

fn find_regions(garden_map: &Input) -> Regions {
    let mut regions = Vec::new();
    let mut cell_to_region: Grid<Option<usize>> =
        Grid::new(garden_map.width, garden_map.height, None);
    for y in 0..garden_map.width {
        for x in 0..garden_map.width {
            let curr = Point::from((x, y));
            let curr_plant = *garden_map.get_unchecked(curr);
            // Check we haven't already labeled this one
            if let Some(_region) = cell_to_region.get_unchecked(curr) {
                continue;
            }
            // Cells discovered to be in this region.
            let mut discovered = HashSet::default();
            // Cells that might be in this region.
            let mut to_explore = Vec::new();
            to_explore.push(curr);
            to_explore.extend(curr.cardinal());
            let mut joined_other_region = false;

            // Find all cells connected to curr.
            while let Some(neighbour) = to_explore.pop() {
                if discovered.contains(&neighbour) {
                    continue;
                }
                let Some(neighbour_plant) = garden_map.get(neighbour) else {
                    continue;
                };
                if *neighbour_plant != curr_plant {
                    continue;
                }
                to_explore.extend(neighbour.cardinal());
                discovered.insert(neighbour);
                // So, we have a neighbouring cell with the same type of plant.
                // If it's already got a region, then our current region joins it.
                let maybe_region_id = *cell_to_region.get_unchecked(neighbour);
                if let Some(region_id) = maybe_region_id {
                    // Set everything we've discovered to the region.
                    for p in &discovered {
                        cell_to_region.set(*p, Some(region_id));
                    }
                    joined_other_region = true;
                    break;
                }
                // Otherwise, this step on the fringe doesn't have a region yet.
                // Add it to ours and move on.
                discovered.insert(neighbour);
            } // Found all cells connected to curr.

            if !joined_other_region {
                for p in &discovered {
                    cell_to_region.set(*p, Some(regions.len()));
                }
                regions.push((curr_plant, discovered.into_iter().collect()))
            }
        }
    }
    Regions {
        cell_to_region: cell_to_region.map(|id| id.unwrap()),
        regions,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_small() {
        let input = parse(
            "AAAA
BBCD
BBCC
EEEC",
        );
        let regions = find_regions(&input);
        debug_regions(&regions.cell_to_region, &input);
        assert_eq!(q1(&input), 140);
        assert_eq!(q2(&input), 80);
    }

    #[test]
    fn test_example_2() {
        let input = parse(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        let regions = find_regions(&input);
        debug_regions(&regions.cell_to_region, &input);
        assert_eq!(q2(&input), 236);
    }

    #[test]
    fn test_example_3() {
        let input = parse(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        let regions = find_regions(&input);
        debug_regions(&regions.cell_to_region, &input);
        assert_eq!(q2(&input), 368, "wrong answer for Q2");
    }

    #[test]
    fn test_example_bigger() {
        let input = parse(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );
        let regions = find_regions(&input);
        debug_regions(&regions.cell_to_region, &input);
        assert_eq!(q1(&input), 1930);
        assert_eq!(q2(&input), 1206);
    }
    fn debug_regions(r: &Grid<usize>, garden_map: &Grid<char>) {
        use termion::color;
        for y in 0..r.height {
            for x in 0..r.width {
                let p = Point::from((x, y));
                let region_id = r.get_unchecked(p);
                let plant = garden_map.get_unchecked(p);

                match region_id {
                    0 => {
                        print!("{}{plant:2}", color::Fg(color::Red))
                    }
                    1 => {
                        print!("{}{plant:2}", color::Fg(color::Green))
                    }
                    2 => {
                        print!("{}{plant:2}", color::Fg(color::Blue))
                    }
                    3 => {
                        print!("{}{plant:2}", color::Fg(color::Magenta))
                    }
                    4 => {
                        print!("{}{plant:2}", color::Fg(color::LightYellow))
                    }
                    5 => {
                        print!("{}{plant:2}", color::Fg(color::Cyan))
                    }
                    6 => {
                        print!("{}{plant:2}", color::Fg(color::White))
                    }
                    7 => {
                        print!("{}{plant:2}", color::Fg(color::Yellow))
                    }
                    8 => {
                        print!("{}{plant:2}", color::Fg(color::LightRed))
                    }
                    9 => {
                        print!("{}{plant:2}", color::Fg(color::LightGreen))
                    }
                    10 => {
                        print!("{}{plant:2}", color::Fg(color::LightBlue))
                    }
                    11 => {
                        print!("{}{plant:2}", color::Fg(color::LightMagenta))
                    }
                    12 => {
                        print!("{}{plant:2}", color::Fg(color::LightBlack))
                    }
                    13 => {
                        print!("{}{plant:2}", color::Fg(color::LightCyan))
                    }
                    14 => {
                        print!("{}{plant:2}", color::Fg(color::LightWhite))
                    }
                    15 => {
                        print!(
                            "{}{}{plant:2}",
                            color::Bg(color::LightBlue),
                            color::Fg(color::Black)
                        )
                    }
                    _ => {
                        panic!("Too many plants");
                    }
                };
                print!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
            }
            println!();
        }
    }
}
