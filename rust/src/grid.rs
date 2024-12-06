type Point = (isize, isize);

#[derive(Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<T>,
}

impl<T> Grid<T> {
    pub fn is_in_bounds(&self, point: Point) -> bool {
        let out_of_bounds = point.0 < 0
            || point.1 < 0
            || point.0 >= self.width as isize
            || point.1 >= self.height as isize;
        !out_of_bounds
    }

    pub fn set(&mut self, point: Point, val: T) {
        let (x, y) = point;
        self.inner[y as usize * self.height + x as usize] = val;
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if !self.is_in_bounds(point) {
            return None;
        }
        Some(self.get_unchecked(point))
    }

    pub fn get_unchecked(&self, point: Point) -> &T {
        let (x, y) = point;
        &self.inner[y as usize * self.height + x as usize]
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn get_copied(&self, point: Point) -> Option<T> {
        self.get(point).copied()
    }
}
