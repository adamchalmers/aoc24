use crate::dir::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
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
            if player == Point::from((x, y)) {
                print!("{}@{}", color::Fg(color::Red), color::Fg(color::Reset));
                continue;
            }
            let ch = grid.get_unchecked(Point::from((x, y)));
            let sigil_color: Box<dyn color::Color> = match ch {
                Cell::Wall => Box::new(color::Blue),
                Cell::Block => Box::new(color::Magenta),
                Cell::Empty => Box::new(color::White),
            };
            let sc: &(dyn color::Color) = sigil_color.as_ref();
            print!("{}{ch}{}", color::Fg(sc), color::Fg(color::Reset));
        }
        println!();
    }
    println!();
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
    // eprintln!("Moved");
    let start = dir.step_from(player);
    if grid.get(start) != Some(&Cell::Block) {
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
            Some(Cell::Block) => {
                maybe_end = maybe_end.step_to(dir);
            }
            None => return,
        }
    };
    // `end` is now the first empty space after this stack of blocks.
    // eprintln!("Shifting {start}, {end}");
    grid.set(end, Cell::Block);
    grid.set(start, Cell::Empty);
}

fn has_free_space_to(dir: Dir, player: Point, grid: &Grid) -> bool {
    let mut curr = player.step_to(dir);
    while let Some(curr_cell) = grid.get(curr) {
        match curr_cell {
            Cell::Empty => return true,
            Cell::Wall => return false,
            Cell::Block => {}
        }
        curr = curr.step_to(dir);
    }
    false
}

fn try_move(dir: Dir, player: &mut Point, grid: &mut Grid) {
    if has_free_space_to(dir, *player, grid) {
        shift_blocks(dir, *player, grid);
        *player = player.step_to(dir);
    }
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
    let mut total = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let p = Point::from((x, y));
            if *grid.get_unchecked(p) == Cell::Block {
                total += gps(p);
            }
        }
    }
    total
}

fn gps(p: Point) -> usize {
    (100 * p.y + p.x).try_into().unwrap()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Cell {
    Wall,
    Block,
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Cell::Wall => '#',
            Cell::Block => 'O',
            Cell::Empty => '.',
        };
        write!(f, "{ch}",)
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
    fn test_example_small() {
        let input = parse(TEST_INPUT_SMALL);
        print(&input.grid, input.player);
        let expected = 2028;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_example_medium() {
        let input = parse(TEST_INPUT_MEDIUM);
        print(&input.grid, input.player);
        let expected = 10092;
        assert_eq!(q1(&input), expected);
    }

    #[test]
    fn test_real() {
        let input = parse(&std::fs::read_to_string("input/2024/day15.txt").unwrap());
        assert_eq!(q1(&input), 1516281);
    }
}
