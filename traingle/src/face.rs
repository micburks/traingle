use super::point::Point;

#[derive(Debug)]
struct Triangle(Point, Point, Point, Point);

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        let max = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));
        Triangle(p1, p2, p3, max)
    }
    pub fn contains(&self, p: Point) -> bool {
        // short circuit algorithm
        if p.0 > self.3 .0 && p.1 > self.3 .1 {
            return false;
        }
        // not actually necessary since we ascend through pixels
        // if p.0 < self.min.0 && p.1 < self.min.1 {
        //     return false;
        // }

        let v0 = self.2 - self.0;
        let v1 = self.1 - self.0;
        let v2 = p - self.0;

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
}

#[derive(Debug)]
pub struct Face {
    pub points: (Point, Point, Point),
    color: image::Rgb<u8>,
    // min: Point,
    max: Point,
    fitness: f32,
}

const SAMPLES: u32 = 5;
impl Face {
    pub fn new(
        p1: Point,
        p2: Point,
        p3: Point,
        img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> Face {
        let tri = Triangle::new(p1, p2, p3);
        let top_left = Point::new(min(p1.0, p2.0, p3.0), min(p1.1, p2.1, p3.1));
        let bottom_right = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));
        let x_inc = (bottom_right.0 - top_left.0) / SAMPLES as f32;
        let y_inc = (bottom_right.1 - top_left.1) / SAMPLES as f32;
        let mut color: (u32, u32, u32) = (0, 0, 0);
        let mut total_used = 0;
        for i in 0..SAMPLES {
            for j in 0..SAMPLES {
                let point = Point::new(
                    top_left.0 + (i as f32 * x_inc) + (x_inc / 2.0),
                    top_left.1 + (j as f32 * y_inc) + (y_inc / 2.0),
                );
                if tri.contains(point) {
                    total_used += 1;
                    let add = *img.get_pixel(point.0 as u32, point.1 as u32);
                    color.0 += add.0[0] as u32;
                    color.1 += add.0[1] as u32;
                    color.2 += add.0[2] as u32;
                }
            }
        }
        if total_used != 0 {
            color.0 /= total_used;
            color.1 /= total_used;
            color.2 /= total_used;
        }
        Face {
            points: (p1, p2, p3),
            color: image::Rgb([color.0 as u8, color.1 as u8, color.2 as u8]),
            // min: Point::new(min(p1.0, p2.0, p3.0), min(p1.1, p2.1, p3.1)),
            max: Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1)),
            fitness: 0.0,
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
    pub fn add_fitness(&mut self, color: image::Rgb<u8>) -> () {
        let face_color = self.color().0;
        let r = (face_color[0] as i32) - (color.0[0] as i32);
        let g = (face_color[1] as i32) - (color.0[1] as i32);
        let b = (face_color[2] as i32) - (color.0[2] as i32);
        let fitness = (r.pow(2) + g.pow(2) + b.pow(2)) as f32;
        if fitness != 0.0 {
            self.fitness = self.fitness + (1.0 / fitness);
        }
    }
    pub fn move_fitness(&mut self) -> () {
        self.points.0.add_fitness(self.fitness);
        self.points.1.add_fitness(self.fitness);
        self.points.2.add_fitness(self.fitness);
    }
    pub fn print_fitness(&self) -> () {
        println!(
            "{} {} {}",
            self.points.0 .2, self.points.1 .2, self.points.2 .2
        );
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
