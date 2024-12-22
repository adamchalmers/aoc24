use crate::point::Point;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        };
        write!(f, "{}", d)
    }
}

impl Dir {
    pub fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    /// Take a step from `curr` in this direction.
    pub fn step_from(self, mut curr: Point) -> Point {
        match self {
            Dir::Up => curr.y -= 1,
            Dir::Down => curr.y += 1,
            Dir::Left => curr.x -= 1,
            Dir::Right => curr.x += 1,
        }
        curr
    }
}
