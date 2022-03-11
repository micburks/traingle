use super::member::Member;
use super::pixel_group::Group;
use super::generation::Generation;
use super::geom::{Point, Triangle};

use spade::delaunay::VertexHandle;
use std::cell::RefCell;
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
    pub triangle: Triangle,
}

impl Face {
    pub fn new(
        del_triangle: [VertexHandle<Point, ()>; 3],
        members: &Vec<Rc<RefCell<Member>>>,
        gen: &mut Generation,
    ) -> Face {
        let triangle = Triangle::new(del_triangle);

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
            let vertex = triangle.vertices.0;
            if point.0 == vertex.0 && point.1 == vertex.1 {
                m1_opt = Some(Rc::clone(&m));
                continue;
            }
            let vertex = triangle.vertices.1;
            if point.0 == vertex.0 && point.1 == vertex.1 {
                m2_opt = Some(Rc::clone(&m));
                continue;
            }
            let vertex = triangle.vertices.2;
            if point.0 == vertex.0 && point.1 == vertex.1 {
                m3_opt = Some(Rc::clone(&m));
                continue;
            }
        }
        let m1 = m1_opt.unwrap();
        let m2 = m2_opt.unwrap();
        let m3 = m3_opt.unwrap();

        let img = gen.img;

        let calc = || -> Group {
            let mut pixels = triangle.iter().map(|point| {
                let p = img.get_pixel(point.0 as u32, point.1 as u32);
                (p.0[0] as f32, p.0[1] as f32, p.0[2] as f32)
            });
            Group::new(&mut pixels)
        };

        let group = gen.cache.insert(triangle.vertices.0, triangle.vertices.1, triangle.vertices.2, calc);

        m1.borrow_mut().add_fitness(group.fitness);
        m2.borrow_mut().add_fitness(group.fitness);
        m3.borrow_mut().add_fitness(group.fitness);

        Face {
            points: (m1, m2, m3),
            color: group.color,
            fitness: group.fitness,
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
                    return Option::Some(&mut self.faces[i]);
                }
                i -= 1;
            }
            if j < end {
                if self.faces[j as usize].triangle.contains(point) {
                    self.last_index = j as i32;
                    let j = j as usize;
                    return Option::Some(&mut self.faces[j]);
                }
                j += 1;
            }
        }

        return None;
    }
}
