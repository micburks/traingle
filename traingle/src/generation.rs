use super::face::Face;
use super::img::Img;
use super::member::{Member, MemberType};
use super::point::Point;

use spade::delaunay::{
    DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation, PositionInTriangulation,
};
use spade::kernels::FloatKernel;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Generation<'a> {
    base: Vec<Rc<RefCell<Member>>>,
    img: &'a Img,
    mutations: usize,
    populations: Vec<Population>,
}

pub struct Population {
    faces: Vec<Face>,
    del: DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
    map: HashMap<String, usize>,
}

impl Population {
    pub fn new(
        faces: Vec<Face>,
        del: DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
        map: HashMap<String, usize>,
    ) -> Population {
        Population { faces, del, map }
    }
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
            img,
            mutations: 0,
            populations: vec![],
        }
    }
    pub fn mutate(&mut self, n: u32) -> () {
        for _i in 0..n {
            for point in &mut self.base {
                point.borrow_mut().mutate();
            }
            self.mutations += 1;
            let mut pop = self.triangulate(self.mutations);
            self.calculate_fitness(&mut pop);
            self.populations.push(pop);
        }
    }
    fn get_face_map(&self, faces: &Vec<Face>) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for i in 0..faces.len() {
            map.insert(faces[i].hash.clone(), i);
        }
        map
    }
    pub fn aggregate_beneficial_mutations(&mut self) -> () {
        for base_member in &self.base {
            base_member.borrow_mut().merge_mutations_into_base();
        }
        self.mutations += 1;
        let mut pop = self.triangulate(self.mutations);
        self.calculate_fitness(&mut pop);
        self.populations.push(pop);
    }
    pub fn triangulate(&self, index: usize) -> Population {
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

        let face_hash = self.get_face_map(&faces);

        Population::new(faces, delaunay, face_hash)
    }
    pub fn get_best_faces(&self) -> Population {
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

        let face_hash = self.get_face_map(&faces);

        Population::new(faces, delaunay, face_hash)
    }
    fn calculate_fitness(&self, pop: &mut Population) -> () {
        let (width, height) = self.img.dimensions();

        // Get fitness from pixels
        let num_pixels = (width * height) as u32;
        '_outer_fitness: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            let actual_color = self.img.get_pixel(x as u32, y as u32);
            let location = pop.del.locate(&Point::new(x, y));
            let face_handle = match location {
                PositionInTriangulation::InTriangle(f) => f,
                PositionInTriangulation::OnEdge(e) => e.face(),
                PositionInTriangulation::OnPoint(v) => {
                    for face in &mut pop.faces {
                        if face.has_vertex(v) {
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
                }
            };
            let [t1, t2, t3] = face_handle.as_triangle();
            let hash = Face::hash(
                Point::from((t1.0, t1.1)),
                Point::from((t2.0, t2.1)),
                Point::from((t3.0, t3.1)),
            );
            let face_index = pop.map.get(&hash).unwrap();
            pop.faces[*face_index].add_fitness(actual_color);
            continue '_outer_fitness;
            // should never be reached
        }

        // Move face fitness to points
        for f in &mut pop.faces {
            f.move_fitness();
        }
    }
    pub fn write(&self, filename: String) -> () {
        let pop = self.get_best_faces();
        self.write_faces(filename, pop);
    }
    pub fn write_faces(
        &self,
        filename: String,
        pop: Population,
    ) -> () {
        let (width, height) = self.img.dimensions();

        // Rasterize image
        let num_pixels = (width * height) as u32;
        let mut buf = vec![];
        '_outer_raster: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            let l = pop.del.locate(&Point::new(x, y));
            let face_handle = match l {
                PositionInTriangulation::InTriangle(f) => f,
                PositionInTriangulation::OnEdge(e) => e.face(),
                PositionInTriangulation::OnPoint(v) => {
                    for face in &pop.faces {
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
                }
            };
            let [t1, t2, t3] = face_handle.as_triangle();
            let hash = Face::hash(
                Point::from((t1.0, t1.1)),
                Point::from((t2.0, t2.1)),
                Point::from((t3.0, t3.1)),
            );
            let face_index = pop.map.get(&hash).unwrap();
            let [r, g, b] = pop.faces[*face_index].color.0;
            buf.push(r as u8);
            buf.push(g as u8);
            buf.push(b as u8);
            // should never be reached
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
                pop.faces.len(),
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
