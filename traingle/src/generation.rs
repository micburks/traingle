use super::face::Face;
use super::img::Img;
use super::member::{Member, MemberType};
use super::point::Point;

use spade::delaunay::FloatDelaunayTriangulation;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Generation<'a> {
    base: Vec<Rc<RefCell<Member>>>,
    faces: Vec<Vec<Face>>,
    img: &'a Img,
    mutations: usize,
}

impl<'a> Generation<'a> {
    pub fn new(points: Vec<(f32, f32)>, img: &'a Img) -> Generation<'a> {
        Generation {
            base: points
                .into_iter()
                .map(|p| {
                    Rc::new(RefCell::new(Member::new(
                        MemberType::Base,
                        p,
                        img.dimensions(),
                    )))
                })
                .collect(),
            faces: vec![],
            img,
            mutations: 0,
        }
    }
    pub fn mutate(&mut self, n: u32) -> () {
        for _i in 0..n {
            for point in &mut self.base {
                point.borrow_mut().mutate();
            }
            self.mutations += 1;
            self.faces
                .push(self.triangulate_and_calculate_fitness(self.mutations));
        }
    }
    pub fn aggregate_beneficial_mutations(&mut self) -> () {
        for base_member in &self.base {
            base_member.borrow_mut().merge_mutations_into_base();
        }
        self.mutations += 1;
        self.faces
            .push(self.triangulate_and_calculate_fitness(self.mutations));
    }
    pub fn triangulate_and_calculate_fitness(&self, index: usize) -> Vec<Face> {
        // ) -> Vec<(Box<[VertexHandle<'static, Point, ()>; 3]>, image::Rgb<u8>)> {
        let (width, height) = self.img.dimensions();

        // Calculate delaunay triangles from points
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut members: Vec<Rc<RefCell<Member>>> = vec![];
        for m in &self.base {
            let point = *m.borrow().point(index);
            delaunay.insert(point);
            members.push(Rc::clone(m));
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(Box::new(triangle), &members, index, &self.img));
        }

        // Get fitness from pixels
        let num_pixels = (width * height) as u32;
        '_outer_fitness: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            for f in &mut faces {
                if f.contains(Point::new(x, y)) {
                    let actual_color = self.img.get_pixel(x as u32, y as u32);
                    f.add_fitness(actual_color);
                    continue '_outer_fitness;
                }
            }
        }

        // Move face fitness to points
        for f in &mut faces {
            f.move_fitness();
            f.print_fitness();
        }

        // check whether mutations were beneficial
        for m in &self.base {
            if index != 0 {
                m.borrow_mut().mark_beneficial_mutations(index);
            }
        }

        faces
    }
    pub fn get_best_faces(&self) -> Vec<Face> {
        let (width, height) = self.img.dimensions();

        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut points: Vec<(f32, f32)> = vec![];
        for m in &self.base {
            let point = m.borrow_mut().get_best();
            delaunay.insert(Point::new(point.0, point.1));
            points.push(point);
        }

        let mut members: Vec<Rc<RefCell<Member>>> = vec![];
        for point in points {
            members.push(Rc::new(RefCell::new(Member::new(
                MemberType::Base,
                point,
                (width, height),
            ))));
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new_without_index(Box::new(triangle), &members, &self.img));
            // faces.push(Box::new(triangle), Face::average_color(triangle, &self.img));
        }

        faces
    }
    pub fn write(&self, filename: String) -> () {
        let faces = self.get_best_faces();
        let (width, height) = self.img.dimensions();

        // Rasterize image
        let num_pixels = (width * height) as u32;
        let mut buf = vec![];
        '_outer_raster: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            for face in &faces {
                if face.contains(Point::new(x, y)) {
                    let [r, g, b] = face.color.0;
                    buf.push(r as u8);
                    buf.push(g as u8);
                    buf.push(b as u8);
                    continue '_outer_raster;
                }
            }
            // some edges are still not hitting
            buf.push(255);
            buf.push(0);
            buf.push(255);
        }

        match image::save_buffer(
            filename,
            &buf[..],
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        ) {
            Ok(_) => println!(
                "done, face: {}, pixels: {}/{}",
                faces.len(),
                buf.len(),
                width * height * 3.0
            ),
            Err(e) => println!("error {}", e),
        }
    }
    pub fn get_best_points(&self) -> Vec<(f32, f32)> {
        let mut ret = vec![];
        for m in &self.base {
            ret.push(m.borrow_mut().get_best());
        }
        ret
    }
    pub fn base(&self) -> Vec<(f32, f32)> {
        let mut ret = vec![];
        for m in &self.base {
            ret.push(m.borrow().values(0));
        }
        ret
    }
}
