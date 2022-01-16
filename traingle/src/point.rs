use spade::{PointN, TwoDimensional};

#[derive(Debug)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point(x, y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1)
    }
}
impl std::ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.0 / rhs, self.1 / rhs)
    }
}
impl std::cmp::Eq for Point {}
impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self::new(self.0, self.1)
    }
}
impl PointN for Point {
    type Scalar = f32;
    fn dimensions() -> usize {
        2
    }
    fn from_value(v: f32) -> Self {
        Self::new(v, v)
    }
    fn nth(&self, s: usize) -> &f32 {
        if s == 0 {
            &self.0
        } else {
            &self.1
        }
    }
    fn nth_mut(&mut self, s: usize) -> &mut f32 {
        if s == 0 {
            &mut self.0
        } else {
            &mut self.1
        }
    }
}
impl TwoDimensional for Point {}
