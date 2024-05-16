#[derive(Debug, Clone, Copy)]
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
}
