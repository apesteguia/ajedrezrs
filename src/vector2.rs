#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vect2<T>
where
    T: Eq + PartialEq + std::fmt::Display + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vect2<T>
where
    T: Eq + PartialEq + std::fmt::Display + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn igual(&self, v: Vect2<T>) -> bool {
        self.x == v.x && self.y == v.y
    }
}
