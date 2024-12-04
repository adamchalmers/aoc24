const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_BACKWARDS: [char; 4] = ['S', 'A', 'M', 'X'];

fn main() {
    let input = include_str!("../input");
    let grid = parse(input);
    let q1 = solve_q1(&grid);
    println!("Q1: {q1}");
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn width(&self) -> usize {
        self.0.len()
    }

    fn height(&self) -> usize {
        self.0[0].len()
    }

    fn at(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width() || y >= self.height() {
            return None;
        }
        Some(self.0[x][y])
    }
}

fn parse(input: &str) -> Grid {
    let vecs = input.lines().map(|line| line.chars().collect()).collect();
    Grid(vecs)
}

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
    println!("Horizontal ({reverse}) -> {found}");
    found
}
fn search_vertical(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..grid.0[0].len() {
        for i in 0..grid.0.len() {
            let Some(x) = grid.at(i, j) else {
                continue;
            };
            let Some(m) = grid.at(i + 1, j) else {
                continue;
            };
            let Some(a) = grid.at(i + 2, j) else {
                continue;
            };
            let Some(s) = grid.at(i + 3, j) else {
                continue;
            };
            if !reverse && [x, m, a, s] == XMAS {
                found += 1
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1
            }
        }
    }
    println!("Vertical ({reverse}) -> {found}");
    found
}
fn search_diag_top_right(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..grid.0[0].len() {
        for i in 0..grid.0.len() {
            let Some(x) = grid.at(i, j) else {
                continue;
            };
            let Some(m) = grid.at(i + 1, j + 1) else {
                continue;
            };
            let Some(a) = grid.at(i + 2, j + 2) else {
                continue;
            };
            let Some(s) = grid.at(i + 3, j + 3) else {
                continue;
            };
            if !reverse && [x, m, a, s] == XMAS {
                found += 1
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1
            }
        }
    }
    println!("DiagTopRight ({reverse}) -> {found}");
    found
}

fn search_diag_top_left(grid: &Grid, reverse: bool) -> usize {
    let mut found = 0;
    for j in 0..grid.0[0].len() {
        for i in (3..grid.0.len()).rev() {
            let Some(x) = grid.at(i, j) else {
                continue;
            };
            let Some(m) = grid.at(i - 1, j + 1) else {
                continue;
            };
            let Some(a) = grid.at(i - 2, j + 2) else {
                continue;
            };
            let Some(s) = grid.at(i - 3, j + 3) else {
                continue;
            };
            if !reverse && [x, m, a, s] == XMAS {
                found += 1
            }
            if reverse && [x, m, a, s] == XMAS_BACKWARDS {
                found += 1
            }
        }
    }
    println!("DiagTopLeft ({reverse}) -> {found}");
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
}
