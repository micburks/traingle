use super::point::Point;

use spade::delaunay::VertexHandle;
use std::iter::FusedIterator;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: (Point, Point, Point),
    max: Point,
    is_vertical: bool, 
    is_horizontal: bool,
}

impl Triangle {
    pub fn new(t: [VertexHandle<Point, ()>; 3]) -> Triangle {
        let mut p1 = *t[0];
        let mut p2 = *t[1];
        let mut p3 = *t[2];

        // Sort so p1.y < p2.y < p3.y
        if p2.1 > p1.1 {
            let tmp = p1;
            p1 = p2;
            p2 = tmp;
        }
        if p3.1 > p2.1 {
            let tmp = p2;
            p2 = p3;
            p3 = tmp;

            if p2.1 > p1.1 {
                let tmp = p1;
                p1 = p2;
                p2 = tmp;
            }
        }

        let max = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));

        // does this triangle lie against the x=0 line?
        let vertical0 = (p1.0 == 0.0 && p2.0 == 0.0)
            || (p1.0 == 0.0 && p3.0 == 0.0)
            || (p2.0 == 0.0 && p3.0 == 0.0);

        // does this triangle lie against the y=0 line?
        let horizontal0 = (p1.1 == 0.0 && p2.1 == 0.0)
            || (p1.1 == 0.0 && p3.1 == 0.0)
            || (p2.1 == 0.0 && p3.1 == 0.0);

        Triangle {
            vertices: (p1, p2, p3),
            max,
            is_vertical: vertical0,
            is_horizontal: horizontal0,
        }
    }
    pub fn contains(&self, p: Point) -> bool {
        let x = p.0;
        let y = p.1;

        let (v0, v1, v2) = self.vertices;

        // if this is less than this triangle's top-left boundary box, skip
        if x > self.max.0 && y > self.max.1 {
            return false;
        }

        // x=0.0 line
        if self.is_vertical && x == 0.0 {
            if (y >= v0.1 && y <= v1.1)  || // 0 <= y <= 1
                (y >= v0.1 && y <= v2.1) || // 0 <= y <= 2
                (y >= v1.1 && y <= v0.1) || // 1 <= y <= 0
                (y >= v1.1 && y <= v2.1) || // 1 <= y <= 2
                (y >= v2.1 && y <= v0.1) || // 2 <= y <= 0
                (y >= v2.1 && y <= v1.1)
            // 2 <= y <= 1
            {
                return true;
            }
        }

        // y=0.0 line
        if self.is_horizontal && y == 0.0 {
            if (x >= v0.0 && x <= v1.0)  || // 0 <= x <= 1
                (x >= v0.0 && x <= v2.0) || // 0 <= x <= 2
                (x >= v1.0 && x <= v0.0) || // 1 <= x <= 0
                (x >= v1.0 && x <= v2.0) || // 1 <= x <= 2
                (x >= v2.0 && x <= v0.0) || // 2 <= x <= 0
                (x >= v2.0 && x <= v1.0)
            // 2 <= x <= 1
            {
                return true;
            }
        }

        // exact vertex matches
        if x == v0.0 && y == v0.1 {
            return true;
        }
        if x == v1.0 && y == v1.1 {
            return true;
        }
        if x == v2.0 && y == v2.1 {
            return true;
        }

        // Sort so p0.x < p1.x < p2.x
        let (mut p0, mut p1, mut p2) = self.vertices;
        if p1.0 > p0.0 {
            let tmp = p0;
            p0 = p1;
            p1 = tmp;
        }
        if p2.0 > p1.0 {
            let tmp = p1;
            p1 = p2;
            p2 = tmp;

            if p1.0 > p0.0 {
                let tmp = p0;
                p0 = p1;
                p1 = tmp;
            }
        }
        let m10 = (p1.1 - p0.1) / (p1.0 - p0.0);
        let y10 = (m10 * p.0) + (p0.1 - (m10 * p0.0));
        if (y10 - p.1).abs() < 0.0001 {
            return true;
        }
        let m20 = (p2.1 - p0.1) / (p2.0 - p0.0);
        let y20 = (m20 * p.0) + (p0.1 - (m20 * p0.0));
        if (y20 - p.1).abs() < 0.0001 {
            return true;
        }
        let m21 = (p2.1 - p1.1) / (p2.0 - p1.0);
        let y21 = (m21 * p.0) + (p1.1 - (m21 * p1.0));
        if (y21 - p.1).abs() < 0.0001 {
            return true;
        }

        let v0 = self.vertices.2 - self.vertices.0;
        let v1 = self.vertices.1 - self.vertices.0;
        let v2 = p - self.vertices.0;

        let d00 = dot(v0, v0);
        let d01 = dot(v0, v1);
        let d02 = dot(v0, v2);
        let d11 = dot(v1, v1);
        let d12 = dot(v1, v2);

        let inv_denom = 1.0 / det(Point::new(d00, d01), Point::new(d01, d11));
        let u = det(Point::new(d11, d01), Point::new(d12, d02)) * inv_denom;
        let v = det(Point::new(d00, d01), Point::new(d02, d12)) * inv_denom;
        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
    pub fn area(t: [VertexHandle<Point, ()>; 3]) -> f32 {
        let p1 = *t[0];
        let p2 = *t[1];
        let p3 = *t[2];
        let (x1, y1) = p1.values();
        let (x2, y2) = p2.values();
        let (x3, y3) = p3.values();
        ((x1 * y2) + (x2 * y3) + (x3 * y1) - (y1 * x2) - (y2 * x3) - (y3 * x1)) / 2.0
    }
    pub fn iter(&self) -> PointIterator {
        PointIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a Triangle {
    type Item = Point;
    type IntoIter = PointIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct PointIterator<'a> {
    triangle: &'a Triangle,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    x: f32,
    y: f32,
}

impl<'a> PointIterator<'a> {
    fn new(t: &'a Triangle) -> PointIterator {
        let p1 = t.vertices.0;
        let p2 = t.vertices.1;
        let p3 = t.vertices.2;
        let left = min(p1.0, p2.0, p3.0);
        let right = max(p1.0, p2.0, p3.0);
        let top = min(p1.1, p2.1, p3.1);
        let bottom = max(p1.1, p2.1, p3.1);
        PointIterator {
            triangle: t,
            left,
            right,
            top,
            bottom,
            x: left,
            y: top,
        }
    }
}

impl<'a> FusedIterator for PointIterator<'a> {}
impl<'a> Iterator for PointIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // iterate over bounding square, only using points in this triangle
        while self.x < self.right && self.y < self.bottom {
            self.x += 1.0;
            if self.x >= self.right {
                self.x = self.left;
                self.y += 1.0;
                if self.y >= self.bottom {
                    return Option::None;
                }
            }
            let point = Point::new(self.x as f32, self.y as f32);
            if self.triangle.contains(point) {
                return Option::Some(point);
            }
        }
        Option::None
    }
}

fn min(a: f32, b: f32, c: f32) -> f32 {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}

fn max(a: f32, b: f32, c: f32) -> f32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

fn dot(a: Point, b: Point) -> f32 {
    (a.0 * b.0) + (a.1 * b.1)
}

fn det(a: Point, b: Point) -> f32 {
    (a.0 * b.1) - (a.1 * b.0)
}
