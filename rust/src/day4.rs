use aoc_runner_derive::{aoc, aoc_generator};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_BACKWARDS: [char; 4] = ['S', 'A', 'M', 'X'];

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn at(&self, x: usize, y: usize) -> char {
        self.0[x][y]
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let vecs = input.lines().map(|line| line.chars().collect()).collect();
    Grid(vecs)
}

#[aoc(day4, part2)]
fn solve_q2(grid: &Grid) -> usize {
    let mut found = 0;
    for j in 1..(grid.0[0].len() - 1) {
        for i in 1..(grid.0.len() - 1) {
            // Check for an A
            if grid.at(i, j) != 'A' {
                continue;
            }
            // Check for the remaining MAS letters.
            let top_l = grid.at(i - 1, j - 1);
            let top_r = grid.at(i + 1, j - 1);
            let bot_l = grid.at(i - 1, j + 1);
            let bot_r = grid.at(i + 1, j + 1);
            let quadrants = [top_l, top_r, bot_l, bot_r];
            if [
                ['M', 'S', 'M', 'S'],
                ['S', 'M', 'S', 'M'],
                ['M', 'M', 'S', 'S'],
                ['S', 'S', 'M', 'M'],
            ]
            .contains(&quadrants)
            {
                found += 1;
            }
        }
    }
    found
}

#[aoc(day4, part1)]
fn solve_q1(grid: &Grid) -> usize {
    search_horizontal(grid, true)
        + search_horizontal(grid, false)
        + search_vertical(grid, true)
        + search_vertical(grid, false)
        + search_diag_top_left(grid, true)
        + search_diag_top_left(grid, false)
        + search_diag_top_right(grid, true)
        + search_diag_top_right(grid, false)
}

fn search_horizontal(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for row in &grid.0 {
        for i in 0..row.len() {
            if row[i..].starts_with(if reverse { &XMAS_BACKWARDS } else { &XMAS }) {
                found += 1;
            }
        }
    }
    found
}
fn search_vertical(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..grid.0[0].len() {
        for i in 0..(grid.0.len() - 3) {
            let x = grid.at(i, j);
            let m = grid.at(i + 1, j);
            let a = grid.at(i + 2, j);
            let s = grid.at(i + 3, j);
            if !reverse && [x, m, a, s] == XMAS {
                found += 1;
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1;
            }
        }
    }
    found
}
fn search_diag_top_right(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..(grid.0[0].len() - 3) {
        for i in 0..(grid.0.len() - 3) {
            let x = grid.at(i, j);
            let m = grid.at(i + 1, j + 1);
            let a = grid.at(i + 2, j + 2);
            let s = grid.at(i + 3, j + 3);
            if !reverse && [x, m, a, s] == XMAS {
                found += 1;
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1;
            }
        }
    }
    found
}

fn search_diag_top_left(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..(grid.0[0].len() - 3) {
        for i in (3..grid.0.len()).rev() {
            let x = grid.at(i, j);
            let m = grid.at(i - 1, j + 1);
            let a = grid.at(i - 2, j + 2);
            let s = grid.at(i - 3, j + 3);
            if !reverse && [x, m, a, s] == XMAS {
                found += 1;
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1;
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_q1() {
        let grid = parse(TEST_INPUT);
        assert_eq!(grid.0.len(), 10);
        assert_eq!(grid.0[0].len(), 10);
        let q1 = solve_q1(&grid);
        assert_eq!(q1, 18);
    }

    #[test]
    fn test_q1_vertical_forwards() {
        let grid = parse(
            "\
XQQQ
MQQQ
AQQQ
SQQQ
",
        );
        assert_eq!(grid.0.len(), 4);
        assert_eq!(grid.0[0].len(), 4);
        let q1 = solve_q1(&grid);
        assert_eq!(q1, 1);
    }
    #[test]
    fn test_q1_vertical_reverse() {
        let grid = parse(
            "\
SQQQ
AQQQ
MQQQ
XQQQ
",
        );
        assert_eq!(grid.0.len(), 4);
        assert_eq!(grid.0[0].len(), 4);
        let q1 = solve_q1(&grid);
        assert_eq!(q1, 1);
    }

    #[test]
    fn test_q2() {
        let grid = parse(TEST_INPUT);
        assert_eq!(grid.0.len(), 10);
        assert_eq!(grid.0[0].len(), 10);
        let q2 = solve_q2(&grid);
        assert_eq!(q2, 9);
    }
    #[test]
    fn test_q2_real() {
        let grid = parse(include_str!("../input/2024/day4.txt"));
        let q2 = solve_q2(&grid);
        assert_eq!(q2, 1905);
    }
    #[test]
    fn test_q1_real() {
        let grid = parse(include_str!("../input/2024/day4.txt"));
        let q1 = solve_q1(&grid);
        assert_eq!(q1, 2613);
    }
}
