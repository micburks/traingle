use super::generation::Generation;
use super::member::Member;
use super::point::Point;

use spade::delaunay::VertexHandle;
use std::cell::RefCell;
use std::rc::Rc;

const SIZE_THRESHOLD: f32 = 0.001;
const BENEFICIAL_DISTANCE: f32 = 100.0;

#[derive(Debug)]
pub struct Face {
    pub points: (
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
    ),
    pub color: image::Rgb<u8>,
    pub fitness: f32,
    pub triangle: Triangle,
}

impl Face {
    pub fn new(
        del_triangle: [VertexHandle<Point, ()>; 3],
        members: &Vec<Rc<RefCell<Member>>>,
        gen: &mut Generation,
    ) -> Face {
        let triangle = Triangle::new(del_triangle);

        let mut v1 = triangle.0;
        let mut v2 = triangle.1;
        let mut v3 = triangle.2;

        // Sort so v1.y < v2.y < v3.y
        if v2.1 > v1.1 {
            let tmp = v1;
            v1 = v2;
            v2 = tmp;
        }
        if v3.1 > v2.1 {
            let tmp = v2;
            v2 = v3;
            v3 = tmp;

            if v2.1 > v1.1 {
                let tmp = v1;
                v1 = v2;
                v2 = tmp;
            }
        }

        // Associate with members
        let mut m1_opt = None;
        let mut m2_opt = None;
        let mut m3_opt = None;
        for m in members {
            if let Some(_) = m1_opt {
                if let Some(_) = m2_opt {
                    if let Some(_) = m3_opt {
                        break;
                    }
                }
            }
            let point = *m.borrow().point;
            if point.0 == v1.0 && point.1 == v1.1 {
                m1_opt = Some(Rc::clone(&m));
                continue;
            }
            if point.0 == v2.0 && point.1 == v2.1 {
                m2_opt = Some(Rc::clone(&m));
                continue;
            }
            if point.0 == v3.0 && point.1 == v3.1 {
                m3_opt = Some(Rc::clone(&m));
                continue;
            }
        }
        let m1 = m1_opt.unwrap();
        let m2 = m2_opt.unwrap();
        let m3 = m3_opt.unwrap();

        let p1 = *m1.borrow().point;
        let p2 = *m2.borrow().point;
        let p3 = *m3.borrow().point;

        let img = gen.img;

        let calc = |p1: Point, p2: Point, p3: Point| -> (f32, image::Rgb<u8>) {
            // find color
            let top_left = Point::new(min(p1.0, p2.0, p3.0), min(p1.1, p2.1, p3.1));
            let bottom_right = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));

            let mut count = 0.0;
            let mut mean = (0.0, 0.0, 0.0);
            let mut m_2 = (0.0, 0.0, 0.0);
            let mut pixels = vec![];
            for x in (top_left.0 as usize)..(bottom_right.0 as usize) {
                for y in (top_left.1 as usize)..(bottom_right.1 as usize) {
                    if triangle.contains(Point::new(x as f32, y as f32)) {
                        let p = img.get_pixel(x as u32, y as u32);
                        let pixel = (p.0[0] as f32, p.0[1] as f32, p.0[2] as f32);
                        pixels.push(pixel);

                        count += 1.0;
                        let delta = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                        mean = (
                            mean.0 + (delta.0 / count),
                            mean.1 + (delta.1 / count),
                            mean.2 + (delta.2 / count),
                        );
                        let delta2 = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                        m_2 = (
                            m_2.0 + (delta.0 * delta2.0),
                            m_2.1 + (delta.1 * delta2.1),
                            m_2.2 + (delta.2 * delta2.2),
                        );
                    }
                }
            }

            let mut pixels_with_distance = vec![];
            let mut beneficial_distances = 0.0;
            for pixel in pixels {
                let distance = (mean.0 - pixel.0).powf(2.0)
                    + (mean.1 - pixel.1).powf(2.0)
                    + (mean.2 - pixel.2).powf(2.0);
                pixels_with_distance.push((pixel, distance));
                if distance < BENEFICIAL_DISTANCE {
                    beneficial_distances += 1.0;
                }
            }

            // sort by distance ascending
            pixels_with_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let mut count = 0.0;
            let mut mean = (0.0, 0.0, 0.0);
            let mut m_2 = (0.0, 0.0, 0.0);
            let end = (pixels_with_distance.len() as f32 * 0.95) as usize;
            for i in 0..end {
                let pixel = pixels_with_distance[i].0;
                count += 1.0;
                let delta = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                mean = (
                    mean.0 + (delta.0 / count),
                    mean.1 + (delta.1 / count),
                    mean.2 + (delta.2 / count),
                );
                let delta2 = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                m_2 = (
                    m_2.0 + (delta.0 * delta2.0),
                    m_2.1 + (delta.1 * delta2.1),
                    m_2.2 + (delta.2 * delta2.2),
                );
            }

            let fitness = if count == 0.0 {
                0.0
            } else {
                ((m_2.0 + m_2.1 + m_2.2) / count)
                    * (beneficial_distances / pixels_with_distance.len() as f32)
                    * (pixels_with_distance.len() as f32)
            };

            let color = image::Rgb([mean.0 as u8, mean.1 as u8, mean.2 as u8]);

            (fitness, color)
        };

        let (fitness, color) = gen.cache.insert(p1, p2, p3, calc);

        m1.borrow_mut().add_fitness(fitness);
        m2.borrow_mut().add_fitness(fitness);
        m3.borrow_mut().add_fitness(fitness);

        Face {
            points: (m1, m2, m3),
            color,
            fitness,
            triangle,
        }
    }
}

#[derive(Debug)]
pub struct FaceFinder<'a> {
    faces: &'a mut Vec<Face>,
    last_index: i32,
}

impl<'a> FaceFinder<'a> {
    pub fn new(faces: &'a mut Vec<Face>) -> FaceFinder<'a> {
        FaceFinder {
            faces,
            last_index: 0,
        }
    }
    pub fn find(&mut self, x: f32, y: f32) -> Option<&mut Face> {
        let start = self.last_index;
        let end = self.faces.len() as i32;
        let point = Point::new(x, y);
        let (mut i, mut j) = (start, start + 1);
        while i >= 0 || j < end {
            if i >= 0 {
                if self.faces[i as usize].triangle.contains(point) {
                    self.last_index = i as i32;
                    let i = i as usize;
                    // if self.faces[i].triangle.area() < SIZE_THRESHOLD {
                    return Option::Some(&mut self.faces[i]);
                    /*
                    } else {
                        let i = if i > 0 { i - 1 } else { i + 1 };
                        return Option::Some(&mut self.faces[i]);
                    }
                    */
                }
                i -= 1;
            }
            if j < end {
                if self.faces[j as usize].triangle.contains(point) {
                    self.last_index = j as i32;
                    let j = j as usize;
                    // if self.faces[j].triangle.area() < SIZE_THRESHOLD {
                    return Option::Some(&mut self.faces[j]);
                    /*
                    } else {
                        let j = if j > 0 { j - 1 } else { j + 1 };
                        return Option::Some(&mut self.faces[j]);
                    }
                    */
                }
                j += 1;
            }
        }

        return None;
    }
}

#[derive(Debug)]
pub struct Triangle(Point, Point, Point, Point, (bool, bool));

impl Triangle {
    pub fn new(t: [VertexHandle<Point, ()>; 3]) -> Triangle {
        let p1 = *t[0];
        let p2 = *t[1];
        let p3 = *t[2];

        let max = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));

        // does this triangle lie against the x=0 line?
        let vertical0 = (p1.0 == 0.0 && p2.0 == 0.0)
            || (p1.0 == 0.0 && p3.0 == 0.0)
            || (p2.0 == 0.0 && p3.0 == 0.0);

        // does this triangle lie against the y=0 line?
        let horizontal0 = (p1.1 == 0.0 && p2.1 == 0.0)
            || (p1.1 == 0.0 && p3.1 == 0.0)
            || (p2.1 == 0.0 && p3.1 == 0.0);

        Triangle(p1, p2, p3, max, (vertical0, horizontal0))
    }
    pub fn contains(&self, p: Point) -> bool {
        let x = p.0;
        let y = p.1;

        // if this is less than this triangle's top-left boundary box, skip
        if x > self.3.0 && y > self.3.1 {
            return false;
        }

        // x=0.0 line
        if self.4.0 && x == 0.0 {
            if (y >= self.0.1 && y <= self.1.1) || // 0 <= y <= 1
                (y >= self.0.1 && y <= self.2.1) || // 0 <= y <= 2
                (y >= self.1.1 && y <= self.0.1) || // 1 <= y <= 0
                (y >= self.1.1 && y <= self.2.1) || // 1 <= y <= 2
                (y >= self.2.1 && y <= self.0.1) || // 2 <= y <= 0
                (y >= self.2.1 && y <= self.1.1)
            // 2 <= y <= 1
            {
                return true;
            }
        }

        // x=0.0 line
        if self.4.1 && y == 0.0 {
            if (x >= self.0.0 && x <= self.1.0) || // 0 <= x <= 1
                (x >= self.0.0 && x <= self.2.0) || // 0 <= x <= 2
                (x >= self.1.0 && x <= self.0.0) || // 1 <= x <= 0
                (x >= self.1.0 && x <= self.2.0) || // 1 <= x <= 2
                (x >= self.2.0 && x <= self.0.0) || // 2 <= x <= 0
                (x >= self.2.0 && x <= self.1.0)
            // 2 <= x <= 1
            {
                return true;
            }
        }

        // exact vertex matches
        if x == self.0.0 && y == self.0.1 {
            return true;
        }
        if x == self.1.0 && y == self.1.1 {
            return true;
        }
        if x == self.2.0 && y == self.2.1 {
            return true;
        }

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
    pub fn area(&self) -> f32 {
        let (x1, y1) = self.0.values();
        let (x2, y2) = self.1.values();
        let (x3, y3) = self.2.values();

        ((x1 * y2) + (x2 * y3) + (x3 * y1) - (y1 * x2) - (y2 * x3) - (y3 * x1)) / 2.0
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
