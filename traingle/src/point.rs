use spade::{PointN, TwoDimensional};

#[derive(Debug)]
pub struct Point(pub f32, pub f32, pub f32, pub bool);

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point(x, y, 0.0, false)
    }
    pub fn clone(x: f32, y: f32, fit: f32, ben: bool) -> Point {
        Point(x, y, fit, ben)
    }
    pub fn from((x, y): (f32, f32)) -> Point {
        Point(x, y, 0.0, false)
    }
    pub fn add_fitness(&mut self, fitness: f32) -> () {
        self.2 += fitness;
    }
    pub fn values(&self) -> (f32, f32) {
        (self.0, self.1)
    }
    pub fn fitness(&self) -> f32 {
        self.2
    }
    pub fn is_beneficial(&self) -> bool {
        self.3
    }
    pub fn set_to_beneficial(&mut self) -> () {
        self.3 = true;
    }
    pub fn mutate(&mut self, delta: Point, (width, height): (f32, f32)) -> Point {
        let mut x = self.0;
        let mut y = self.1;
        if x != 0.0 && x != width as f32 {
            x += delta.0;
            if x > width as f32 {
                x = width as f32;
            } else if x < 0.0 {
                x = 0.0
            }
        }
        if y != 0.0 && y != height as f32 {
            y += delta.1;
            if y > height as f32 {
                y = height as f32;
            } else if self.1 < 0.0 {
                y = 0.0;
            }
        }
        self.0 = x;
        self.1 = y;
        *self
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
        Self::clone(self.0, self.1, self.2, self.3)
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
