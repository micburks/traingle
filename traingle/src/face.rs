use super::point::Point;

#[derive(Debug)]
pub struct Face {
    pub points: (Point, Point, Point),
    color: image::Rgb<u8>,
    // min: Point,
    max: Point,
}

impl Face {
    pub fn new(p1: Point, p2: Point, p3: Point, color: image::Rgb<u8>) -> Face {
        Face {
            points: (p1, p2, p3),
            color,
            // min: Point::new(min(p1.0, p2.0, p3.0), min(p1.1, p2.1, p3.1)),
            max: Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1)),
        }
    }
    pub fn contains(&self, p: Point) -> bool {
        // short circuit algorithm
        if p.0 > self.max.0 && p.1 > self.max.1 {
            return false;
        }
        // not actually necessary since we ascend through pixels
        // if p.0 < self.min.0 && p.1 < self.min.1 {
        //     return false;
        // }

        let v0 = self.points.2 - self.points.0;
        let v1 = self.points.1 - self.points.0;
        let v2 = p - self.points.0;

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
    pub fn color(&self) -> image::Rgb<u8> {
        self.color
    }
}

fn dot(a: Point, b: Point) -> f32 {
    (a.0 * b.0) + (a.1 * b.1)
}

fn det(a: Point, b: Point) -> f32 {
    (a.0 * b.1) - (a.1 * b.0)
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
