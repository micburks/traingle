use super::img::Img;
use super::member::Member;
use super::point::Point;

use spade::delaunay::{FaceHandle, VertexHandle};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Face {
    pub points: (
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
    ),
    pub color: image::Rgb<u8>,
    pub fitness: f32,
    index: usize,
    pub hash: String,
    pub triangle: Triangle,
}

impl Face {
    pub fn new(
        del_triangle: [VertexHandle<Point, ()>; 3],
        members: &Vec<Rc<RefCell<Member>>>,
        index: usize,
        img: &Img,
    ) -> Face {
        let triangle = Triangle::new(del_triangle);

        let v1 = triangle.0;
        let v2 = triangle.1;
        let v3 = triangle.2;

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
            let point = *m.borrow().point(index);
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

        let p1 = *m1.borrow().point(index);
        let p2 = *m2.borrow().point(index);
        let p3 = *m3.borrow().point(index);

        let dim = img.dimensions();
        let width = dim.0 as f32;
        let height = dim.1 as f32;
        let mut points: Vec<Point> = vec![p1, p2, p3];
        let p12_13 = (p1 - p2) * (1.0 / 3.0) + p1;
        points.push(p12_13);
        let p12_23 = (p1 - p2) * (2.0 / 3.0) + p1;
        points.push(p12_23);
        let p13_13 = (p1 - p3) * (1.0 / 3.0) + p1;
        points.push(p13_13);
        let p13_23 = (p1 - p3) * (2.0 / 3.0) + p1;
        points.push(p13_23);
        let p23_13 = (p2 - p3) * (1.0 / 3.0) + p2;
        points.push(p23_13);
        let p23_23 = (p2 - p3) * (2.0 / 3.0) + p2;
        points.push(p23_23);

        points.push((p12_13 - p23_13) * (1.0 / 2.0) + p12_13);
        points.push((p12_13 - p23_23) * (1.0 / 2.0) + p12_13);

        points.push((p12_23 - p13_13) * (1.0 / 2.0) + p12_23);
        points.push((p12_23 - p13_23) * (1.0 / 2.0) + p12_23);

        points.push((p23_13 - p13_13) * (1.0 / 2.0) + p23_13);
        points.push((p23_13 - p13_23) * (1.0 / 2.0) + p23_13);

        points.push((p23_23 - p12_23) * (1.0 / 2.0) + p23_23);

        points.push((p13_23 - p12_13) * (1.0 / 2.0) + p13_23);

        points.push((p13_13 - p23_23) * (1.0 / 2.0) + p13_13);

        let mut color: (u32, u32, u32) = (0, 0, 0);
        let mut total_used = 0;
        for point in points {
            if point.0 >= 0.0 && point.0 < width && point.1 >= 0.0 && point.1 < height {
                total_used += 1;
                let add = img.get_pixel(point.0 as u32, point.1 as u32);
                color.0 += add.0[0] as u32;
                color.1 += add.0[1] as u32;
                color.2 += add.0[2] as u32;
            }
        }
        if total_used != 0 {
            color.0 /= total_used;
            color.1 /= total_used;
            color.2 /= total_used;
        }

        let hash = Face::hash(p1, p2, p3);
        Face {
            points: (m1, m2, m3),
            color: image::Rgb([color.0 as u8, color.1 as u8, color.2 as u8]),
            fitness: 0.0,
            index,
            hash,
            triangle,
        }
    }
    pub fn hash(p1: Point, p2: Point, p3: Point) -> String {
        let mut s: Vec<Point> = vec![p1, p2, p3];
        for i in 0..3 {
            let mut small = i;
            for j in (i + 1)..3 {
                if s[j].0 < s[small].0 {
                    small = j;
                }
            }
            s.swap(small, i);
        }
        format!(
            "{}-{},{}-{},{}-{}",
            s[0].0, s[0].1, s[1].0, s[1].1, s[2].0, s[2].1
        )
    }
    pub fn add_fitness(&mut self, color: image::Rgb<u8>) -> () {
        let face_color = self.color.0;
        let dr = (face_color[0] as i32) - (color.0[0] as i32);
        let dg = (face_color[1] as i32) - (color.0[1] as i32);
        let db = (face_color[2] as i32) - (color.0[2] as i32);
        // diff = [0, 1, 4, 9, ...]
        let diff = (dr.pow(2) + dg.pow(2) + db.pow(2)) as f32;
        if diff == 0.0 {
            self.fitness += 10.0;
        } else {
            self.fitness += 1.0 / diff;
        }
    }
    pub fn move_fitness(&mut self) -> () {
        self.points
            .0
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
        self.points
            .1
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
        self.points
            .2
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
    }
    pub fn is_same(&self, face_handle: FaceHandle<Point, ()>) -> bool {
        let p0 = self.points.0.borrow().values(0);
        let p1 = self.points.1.borrow().values(0);
        let p2 = self.points.2.borrow().values(0);
        for tri in face_handle.as_triangle() {
            if p0.0 == tri.0 && p0.1 == tri.1 {
                continue;
            }
            if p1.0 == tri.0 && p1.1 == tri.1 {
                continue;
            }
            if p2.0 == tri.0 && p2.1 == tri.1 {
                continue;
            }
            return false;
        }
        true
    }
    pub fn has_vertex(&self, vertex: VertexHandle<Point, ()>) -> bool {
        let p0 = self.points.0.borrow().values(0);
        if p0.0 == vertex.0 && p0.1 == vertex.1 {
            return true;
        }
        let p1 = self.points.1.borrow().values(0);
        if p1.0 == vertex.0 && p1.1 == vertex.1 {
            return true;
        }
        let p2 = self.points.2.borrow().values(0);
        if p2.0 == vertex.0 && p2.1 == vertex.1 {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct FaceFinder<'a> {
    faces: &'a mut Vec<Face>,
    map: &'a HashMap<String, usize>,
    last_index: Option<i32>,
}

impl<'a> FaceFinder<'a> {
    pub fn new(faces: &'a mut Vec<Face>, map: &'a HashMap<String, usize>) -> FaceFinder<'a> {
        FaceFinder {
            faces,
            map,
            last_index: None,
        }
    }
    pub fn find(&mut self, x: f32, y: f32) -> Option<&mut Face> {
        let start = match self.last_index {
            Some(ind) => ind,
            None => 0,
        };
        let end = self.faces.len() as i32;
        let (mut i, mut j) = (start, start + 1);
        while i >= 0 || j < end {
            if i >= 0 {
                if self.faces[i as usize].triangle.contains(Point::new(x, y)) {
                    self.last_index = Some(i);
                    return Option::Some(&mut self.faces[i as usize]);
                }
                i -= 1;
            }
            if j < end {
                if self.faces[j as usize].triangle.contains(Point::new(x, y)) {
                    self.last_index = Some(j);
                    return Option::Some(&mut self.faces[j as usize]);
                }
                j += 1;
            }
        }
        // println!("missed face detection {},{} - start{}/end{}", x, y, start, end);
        None
    }
}

#[derive(Debug)]
pub struct Triangle(Point, Point, Point, Point, bool);

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

        Triangle(p1, p2, p3, max, vertical0)
    }
    pub fn contains(&self, p: Point) -> bool {
        let x = p.0;
        let y = p.1;

        // if this is less than this triangle's top-left boundary box, skip
        if x > self.3.0 && y > self.3.1 {
            return false;
        }

        // x=0.0 line
        if self.4 && x == 0.0 {
            if y >= self.0.1 && y <= self.1.1 || // 0 <= y <= 1
                y >= self.0.1 && y <= self.2.1 || // 0 <= y <= 2
                y >= self.1.1 && y <= self.0.1 || // 1 <= y <= 0
                y >= self.1.1 && y <= self.2.1 || // 1 <= y <= 2
                y >= self.2.1 && y <= self.0.1 || // 2 <= y <= 0
                y >= self.2.1 && y <= self.1.1 // 2 <= y <= 1
            {
                return true;
            }
        }

        // exact vertex matches
        if x == self.0 .0 && y == self.0 .1 {
            return true;
        }
        if x == self.1 .0 && y == self.1 .1 {
            return true;
        }
        if x == self.2 .0 && y == self.2 .1 {
            return true;
        }

        /*
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
        */

        // rewritten to be incomprehensible
        let v0x = self.2.0 - self.0.0;
        let v0y = self.2.1 - self.0.1;
        let v1x = self.1.0 - self.0.0;
        let v1y = self.1.1 - self.0.1;
        let v2x = x - self.0.0;
        let v2y = y - self.0.1;

        let d00 = (v0x * v0x) + (v0y * v0y);
        let d01 = (v0x * v1x) + (v0y * v1y);
        let d02 = (v0x * v2x) + (v0y * v2y);
        let d11 = (v1x * v1x) + (v1y * v1y);
        let d12 = (v1x * v2x) + (v1y * v2y);

        let inv_denom = 1.0 / ((d00 * d11) - (d01 * d01));
        let u = ((d11 * d02) - (d01 * d12)) * inv_denom;
        let v = ((d00 * d12) - (d01 * d02)) * inv_denom;
        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
}

/*
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
*/

fn max(a: f32, b: f32, c: f32) -> f32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}
