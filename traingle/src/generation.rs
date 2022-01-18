use super::face::Face;
use super::img::Img;
use super::member::{Member, MemberType};
use super::point::Point;

use spade::delaunay::{
    DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation, PositionInTriangulation,
};
use spade::kernels::FloatKernel;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Generation<'a> {
    base: Vec<Rc<RefCell<Member>>>,
    faces: Vec<Vec<Face>>,
    dels: Vec<DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>>,
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
            dels: vec![],
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
            let (mut faces, d) = self.triangulate(self.mutations);
            self.calculate_fitness(&mut faces, &d);
            self.faces.push(faces);
            self.dels.push(d);
        }
    }
    pub fn aggregate_beneficial_mutations(&mut self) -> () {
        for base_member in &self.base {
            base_member.borrow_mut().merge_mutations_into_base();
        }
        self.mutations += 1;
        let (mut faces, d) = self.triangulate(self.mutations);
        self.calculate_fitness(&mut faces, &d);
        self.faces.push(faces);
        self.dels.push(d);
    }
    pub fn triangulate(
        &self,
        index: usize,
    ) -> (
        Vec<Face>,
        DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
    ) {
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

        (faces, delaunay)
    }
    pub fn get_best_faces(
        &self,
    ) -> (
        Vec<Face>,
        DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
    ) {
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut points: Vec<(f32, f32)> = vec![];
        for m in &self.base {
            let point = m.borrow_mut().get_best();
            delaunay.insert(Point::from(point));
            points.push(point);
        }

        let mut members: Vec<Rc<RefCell<Member>>> = vec![];
        for point in points {
            members.push(Rc::new(RefCell::new(Member::new(
                MemberType::Base,
                point,
                self.img.dimensions(),
            ))));
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(Box::new(triangle), &members, 0, &self.img));
        }

        (faces, delaunay)
    }
    pub fn calculate_fitness(&self, faces: &mut Vec<Face>, del: &DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>) -> () {
        let (width, height) = self.img.dimensions();

        // Get fitness from pixels
        let num_pixels = (width * height) as u32;
        '_outer_fitness: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            let location = del.locate(&Point::new(x, y));
            let face_handle = match location {
                PositionInTriangulation::InTriangle(f) => f,
                PositionInTriangulation::OnEdge(e) => e.face(),
                PositionInTriangulation::OnPoint(v) => {
                    for face in faces.into_iter() {
                        if face.has_vertex(v) {
                            let actual_color = self.img.get_pixel(x as u32, y as u32);
                            face.add_fitness(actual_color);
                            continue '_outer_fitness;
                        }
                    }
                    // should never be reached
                    continue '_outer_fitness;
                }
                _ => {
                    // should never be reached
                    continue '_outer_fitness;
                },
            };
            for face in faces.into_iter() {
                if face.is_same(face_handle) {
                    let actual_color = self.img.get_pixel(x as u32, y as u32);
                    face.add_fitness(actual_color);
                    continue '_outer_fitness;
                }
            }
            // should never be reached
        }

        // Move face fitness to points
        for f in faces.into_iter() {
            f.move_fitness();
        }
    }
    pub fn write(&self, filename: String) -> () {
        let (faces, d) = self.get_best_faces();
        self.write_faces(filename, faces, d)
    }
    pub fn write_faces(&self, filename: String, faces: Vec<Face>, d: DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>) -> () {
        let (width, height) = self.img.dimensions();

        // Rasterize image
        let num_pixels = (width * height) as u32;
        let mut buf = vec![];
        '_outer_raster: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            let l = d.locate(&Point::new(x, y));
            let face_handle = match l {
                PositionInTriangulation::InTriangle(f) => f,
                PositionInTriangulation::OnEdge(e) => e.face(),
                PositionInTriangulation::OnPoint(v) => {
                    for face in &faces {
                        if face.has_vertex(v) {
                            let [r, g, b] = face.color.0;
                            buf.push(r as u8);
                            buf.push(g as u8);
                            buf.push(b as u8);
                            continue '_outer_raster;
                        }
                    }
                    // should never be reached
                    continue '_outer_raster;
                }
                _ => {
                    // should never be reached
                    continue '_outer_raster;
                },
            };
            for face in &faces {
                if face.is_same(face_handle) {
                    let [r, g, b] = face.color.0;
                    buf.push(r as u8);
                    buf.push(g as u8);
                    buf.push(b as u8);
                    continue '_outer_raster;
                }
            }
            // should never be reached
            /*
            buf.push(255);
            buf.push(0);
            buf.push(255);
            */
        }

        println!(
            "faces {}, buf {}, should be {}",
            faces.len(),
            buf.len(),
            width * height * 3.0
        );
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
            ret.push(m.borrow().get_best());
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
