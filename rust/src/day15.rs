use crate::dir::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use termion::color;

use crate::point::Point;

#[derive(Clone)]
struct Input {
    grid: Grid,
    player: Point,
    instructions: Vec<Dir>,
}

type Grid = crate::grid::Grid<Cell>;

#[allow(dead_code)]
fn print(grid: &Grid, player: Point) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::from((x, y));
            if player == p {
                print!("{}@{}", color::Fg(color::Red), color::Fg(color::Reset));
                continue;
            }
            let ch = grid.get_unchecked(p);
            let sigil_fg: Box<dyn color::Color> = match ch {
                Cell::Wall => Box::new(color::Blue),
                Cell::Block => Box::new(color::White),
                Cell::BlockLeft | Cell::BlockRight => Box::new(color::Cyan),
                Cell::Empty => Box::new(color::White),
            };
            print!(
                "{}{ch}{}",
                color::Fg(sigil_fg.as_ref()),
                color::Fg(color::Reset)
            );
        }
        println!();
    }
    println!();
}

fn expand(grid: Grid) -> Grid {
    let width = grid.width * 2;
    let height = grid.height;
    assert_eq!(grid.width * grid.height, grid.inner.len());

    let inner: Vec<_> = grid
        .inner
        .into_iter()
        .flat_map(|cell| {
            match cell {
                /*
                If the tile is #, the new map contains ## instead.
                If the tile is O, the new map contains [] instead.
                If the tile is ., the new map contains .. instead.
                If the tile is @, the new map contains @. instead.
                */
                Cell::Wall => [Cell::Wall, Cell::Wall],
                Cell::Block => [Cell::BlockLeft, Cell::BlockRight],
                Cell::Empty => [Cell::Empty, Cell::Empty],
                Cell::BlockLeft | Cell::BlockRight => unreachable!(),
            }
        })
        .collect();
    assert_eq!(width * height, inner.len());
    Grid {
        width,
        height,
        inner,
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Input {
    let (input, instructions) = input.split_once("\n\n").unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut inner = Vec::with_capacity(width * height);
    let mut player = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            let cell = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::Block,
                '@' => {
                    // foo
                    player = Some(Point::from((x, y)));
                    Cell::Empty
                }
                _ => unreachable!(),
            };
            inner.push(cell);
        }
    }
    let instructions = instructions
        .lines()
        .flat_map(|line| {
            line.trim().chars().map(|ch| match ch {
                'v' => Dir::Down,
                '^' => Dir::Up,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!(),
            })
        })
        .collect();
    Input {
        grid: Grid {
            width,
            height,
            inner,
        },
        player: player.unwrap(),
        instructions,
    }
}

fn shift_blocks(dir: Dir, player: Point, grid: &mut Grid) {
    let start = dir.step_from(player);
    if !matches!(
        grid.get(start),
        Some(Cell::Block | Cell::BlockLeft | Cell::BlockRight)
    ) {
        return;
    }
    // At this point we know `start` is a block.
    // Try to find the `end` block.
    let mut maybe_end = start.step_to(dir);
    let end = loop {
        match grid.get(maybe_end) {
            Some(Cell::Empty) => {
                break maybe_end;
            }
            Some(Cell::Wall) => return,
            Some(Cell::Block | Cell::BlockLeft | Cell::BlockRight) => {
                maybe_end = maybe_end.step_to(dir);
            }
            None => return,
        }
    };
    // `end` is now the first empty space after this stack of blocks.
    // println!("Shifting {start}, {end}");
    grid.set(end, Cell::Block);
    grid.set(start, Cell::Empty);
}

fn has_free_space_to(dir: Dir, player: Point, grid: &Grid) -> bool {
    let mut curr = player.step_to(dir);
    while let Some(curr_cell) = grid.get(curr) {
        match curr_cell {
            Cell::Empty => return true,
            Cell::Wall => return false,
            Cell::Block | Cell::BlockLeft | Cell::BlockRight => {}
        }
        curr = curr.step_to(dir);
    }
    false
}

fn has_free_space_to2(dir: Dir, player: Point, grid: &Grid) -> Option<Vec<Cell>> {
    let mut curr = player.step_to(dir);
    let mut boxes = Vec::new();
    while let Some(curr_cell) = grid.get(curr) {
        match curr_cell {
            Cell::Empty => return Some(boxes),
            Cell::Wall => return None,
            Cell::Block | Cell::BlockLeft | Cell::BlockRight => boxes.push(*curr_cell),
        }
        curr = curr.step_to(dir);
    }
    None
}

fn try_move(dir: Dir, player: &mut Point, grid: &mut Grid) {
    if has_free_space_to(dir, *player, grid) {
        shift_blocks(dir, *player, grid);
        *player = player.step_to(dir);
    }
}

#[aoc(day15, part2)]
fn q2(input: &Input) -> usize {
    // println!("Input grid:");
    // print(&input.grid, input.player);
    let mut grid = expand(input.grid.clone());
    let mut player = Point {
        x: input.player.x * 2,
        y: input.player.y,
    };
    let instructions = input.instructions.iter().copied();
    for (_i, dir) in instructions.into_iter().enumerate() {
        // print(&grid, player);
        // println!("{i}: Move {dir:?}");
        match dir {
            Dir::Left | Dir::Right => {
                if let Some(boxes) = has_free_space_to2(dir, player, &grid) {
                    if boxes.is_empty() {
                        // println!("Moving");
                    } else {
                        // println!("Moving, pushing {} boxes", boxes.len());
                    }
                    let mut curr = player.step_to(dir).step_to(dir);
                    // println!("{curr}");
                    for cell in boxes.iter() {
                        grid.set(curr, *cell);
                        curr = curr.step_to(dir);
                    }
                    grid.set(player.step_to(dir), Cell::Empty);
                    player = player.step_to(dir);
                } else {
                    // println!("Can't move");
                }
            }
            Dir::Down | Dir::Up => match grid.get(player.step_to(dir)) {
                // Try to move the player up.
                Some(Cell::Empty) => {
                    player = player.step_to(dir);
                    // println!("Moved");
                }
                Some(Cell::Wall) | None => {
                    // println!("Blocked by wall");
                }
                Some(Cell::Block) => unreachable!("These don't exist in Q2"),
                Some(b @ Cell::BlockLeft | b @ Cell::BlockRight) => {
                    // Find all blocks above. Track their left side cell.
                    let mut boxes_found: Vec<Point> = Vec::new();
                    let mut fringe = if matches!(b, Cell::BlockLeft) {
                        vec![player.step_to(dir)]
                    } else {
                        vec![player.step_to(dir).step_to(Dir::Left)]
                    };
                    let mut had_free_space = true;
                    while let Some(curr) = fringe.pop() {
                        boxes_found.push(curr);
                        // Are there boxes above this one?
                        // let next = curr.step_to(dir);
                        let curr_cell = grid.get(curr);
                        let nexts = if curr_cell == Some(&Cell::BlockLeft) {
                            [curr.step_to(dir), curr.step_to(dir).step_to(Dir::Right)]
                        } else if curr_cell == Some(&Cell::BlockRight) {
                            [curr.step_to(dir).step_to(Dir::Left), curr.step_to(dir)]
                        } else {
                            panic!("Idk why {:?} is in the grid", curr_cell);
                            // continue;
                        };
                        for next in nexts {
                            match grid
                                .get(next)
                                .expect("should be a wall surrounding everything")
                            {
                                Cell::Wall => {
                                    had_free_space = false;
                                    break;
                                }
                                Cell::Block => unreachable!("None of these in Q2"),
                                Cell::Empty => {}
                                Cell::BlockLeft => fringe.push(next),
                                Cell::BlockRight => fringe.push(next.step_to(Dir::Left)),
                            }
                        }
                    }
                    if had_free_space {
                        // Move all boxes 1 space up, and player too.
                        boxes_found.sort_unstable_by_key(|p| p.y);
                        if dir == Dir::Down {
                            boxes_found.reverse();
                        }
                        // println!("Shifted {} blocks", boxes_found.len());
                        for p in boxes_found {
                            grid.set(p.step_to(dir), Cell::BlockLeft);
                            grid.set(p.step_to(dir).step_to(Dir::Right), Cell::BlockRight);
                            grid.set(p, Cell::Empty);
                            grid.set(p.step_to(Dir::Right), Cell::Empty);
                        }
                        player = player.step_to(dir);
                    } else {
                        // println!("Blocked by box");
                    }
                }
            },
        }
    }
    // print(&grid, player);
    score(&grid, Cell::BlockLeft)
}

#[aoc(day15, part1)]
fn q1(input: &Input) -> usize {
    let Input {
        instructions,
        mut player,
        mut grid,
    } = input.to_owned();
    for dir in instructions {
        // println!("Move #{i} {:?}", dir);
        try_move(dir, &mut player, &mut grid);
        // print(&grid, player);
    }
    score(&grid, Cell::Block)
}

fn score(grid: &Grid, target: Cell) -> usize {
    (0..grid.width)
        .cartesian_product(0..grid.height)
        .filter(|(x, y)| *grid.get_unchecked(Point::from((*x, *y))) == target)
        .map(|(x, y)| gps(Point::from((x, y))))
        .sum()
}

fn gps(p: Point) -> usize {
    (100 * p.y + p.x).try_into().unwrap()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Cell {
    Wall,
    Block,
    Empty,
    BlockLeft,
    BlockRight,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Cell::Wall => '#',
            Cell::Block => 'O',
            Cell::Empty => '.',
            Cell::BlockLeft => '[',
            Cell::BlockRight => ']',
        };
        write!(f, "{ch}",)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_SMALL: &str = "\
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    <^^>>>vv<v>>v<<";

    const TEST_INPUT_MEDIUM: &str = "\
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn example_small() {
        let input = parse(TEST_INPUT_SMALL);
        print(&input.grid, input.player);
        let expected = 2028;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn example_medium() {
        let input = parse(TEST_INPUT_MEDIUM);
        print(&input.grid, input.player);
        // assert_eq!(q1(&input), 10092);
        assert_eq!(q2(&input), 9021)
    }

    #[test]
    fn example_q2() {
        let input = parse(
            "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        );
        q2(&input);
    }

    #[test]
    fn test_real() {
        let input = parse(&std::fs::read_to_string("input/2024/day15.txt").unwrap());
        assert_eq!(q1(&input), 1516281);
    }
}
