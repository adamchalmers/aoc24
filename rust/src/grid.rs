use crate::point::Point;

#[derive(Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, initial_value: T) -> Self {
        let inner = vec![initial_value; width * height];
        Self {
            width,
            height,
            inner,
        }
    }
}

impl<T> Grid<T> {
    #[must_use]
    pub fn is_in_bounds(&self, point: Point) -> bool {
        let out_of_bounds = point.x < 0
            || point.y < 0
            || point.x >= self.width as isize
            || point.y >= self.height as isize;
        !out_of_bounds
    }

    pub fn set(&mut self, point: Point, val: T) {
        let Point { x, y } = point;
        self.inner[y as usize * self.height + x as usize] = val;
    }

    #[must_use]
    pub fn get(&self, point: Point) -> Option<&T> {
        if !self.is_in_bounds(point) {
            return None;
        }
        Some(self.get_unchecked(point))
    }

    #[must_use]
    pub fn get_unchecked(&self, point: Point) -> &T {
        let Point { x, y } = point;
        &self.inner[y as usize * self.height + x as usize]
    }

    #[must_use]
    pub fn map<U, F>(self, f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        Grid {
            width: self.width,
            height: self.height,
            inner: self.inner.into_iter().map(f).collect(),
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    #[must_use]
    pub fn get_copied(&self, point: Point) -> Option<T> {
        self.get(point).copied()
    }
}
