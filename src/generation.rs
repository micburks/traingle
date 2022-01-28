use super::face::{Face, FaceFinder};
use super::img::Img;
use super::member::{Member, MemberType};
use super::point::Point;

use spade::delaunay::{
    DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation
};
// use spade::kernels::FloatKernel;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Generation<'a> {
    base: Vec<Rc<RefCell<Member>>>,
    img: &'a Img,
    mutations: usize,
    populations: Vec<Population>,
}

pub struct Population {
    pub faces: Vec<Face>,
    // del: DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
    pub points: Vec<(f32, f32)>,
}

impl Population {
    pub fn new(
        faces: Vec<Face>,
        //del: DelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
        points: Vec<(f32, f32)>,
    ) -> Population {
        Population { faces, points }
    }
}

impl<'a> Generation<'a> {
    pub fn new(points: Vec<(f32, f32)>, img: &'a Img) -> Generation<'a> {
        Generation {
            base: points
                .into_iter()
                .enumerate()
                .map(|(id, p)| {
                    Rc::new(RefCell::new(Member::new(
                        id,
                        MemberType::Base,
                        p,
                        img.dimensions(),
                        img.points(),
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
            let pop = self.triangulate(self.mutations);
            self.populations.push(pop);
        }

        // aggregate best mutations
        for base_member in &self.base {
            base_member.borrow_mut().merge_mutations_into_base();
        }
        self.mutations += 1;
        let pop = self.triangulate(self.mutations);
        self.populations.push(pop);
    }
    // Create triangles from a set of points
    // Calculate fitness of each triangle and aggregate in each member
    pub fn triangulate(&self, index: usize) -> Population {
        // Calculate delaunay triangles from points
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut members: Vec<Rc<RefCell<Member>>> = vec![];
        let mut points = vec![];
        for m in &self.base {
            let point = *m.borrow().point(index);
            delaunay.insert(point);
            members.push(Rc::clone(m));
            points.push(point.values());
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(triangle, &members, index, &self.img));
        }

        Population::new(faces, points)
    }
    pub fn get_best_faces(&self) -> Population {
        let points = self.get_best_points();

        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut members: Vec<Rc<RefCell<Member>>> = vec![];
        for (id, point) in points.clone().into_iter().enumerate() {
            delaunay.insert(Point::from(point));
            members.push(Rc::new(RefCell::new(Member::new(
                id,
                MemberType::Base,
                point,
                self.img.dimensions(),
                self.img.points(),
            ))));
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(triangle, &members, 0, &self.img));
        }

        Population::new(faces, points)
    }
    pub fn write_faces(&self, filename: String, faces: &mut Vec<Face>) -> () {
        let (width, height) = self.img.dimensions();

        // Rasterize image
        let num_pixels = (width * height) as u32;
        let mut face_finder = FaceFinder::new(faces);
        let mut buf = vec![];
        '_outer_raster: for i in 0..num_pixels {
            // find containing triangle
            let x = i as f32 % width;
            let y = i as f32 / width;
            let face = face_finder.find(x, y);
            let [r, g, b] = match face {
                Some(f) => f.color.0,
                None => {
                    // try to use a nearby pixel
                    let found = if x == 0.0 && y == 0.0 {
                        None
                    } else if x == 0.0 {
                        face_finder.find(x, y - 1.0)
                    } else if y == 0.0 {
                        face_finder.find(x - 1.0, y)
                    } else {
                        face_finder.find(x - 1.0, y - 1.0)
                    };
                    match found {
                        Some(f) => f.color.0,
                        None => [255, 0, 255],
                    }
                }
            };
            buf.push(r as u8);
            buf.push(g as u8);
            buf.push(b as u8);
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
        let mut v = vec![];
        for pop in &self.populations {
            for face in &pop.faces {
                v.push(face);
            }
        }
        v.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let mut points: Vec<(f32, f32)> = vec![];
        let mut i = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut sum = 0.0;
        while i < v.len() && points.len() < self.img.points() as usize {
            let face = v[i];
            let m1 = face.points.0.borrow();
            if !seen.contains(&m1.id) {
                points.push(m1.values(face.index));
                seen.insert(m1.id);
                sum += m1.fitness(face.index);
            }
            let m2 = face.points.1.borrow();
            if !seen.contains(&m2.id) {
                points.push(m2.values(face.index));
                seen.insert(m2.id);
                sum += m2.fitness(face.index);
            }
            let m3 = face.points.2.borrow();
            if !seen.contains(&m3.id) {
                points.push(m3.values(face.index));
                seen.insert(m3.id);
                sum += m3.fitness(face.index);
            }
            i += 1;
        }

        println!("average fitness {}", sum / points.len() as f32);
        points
    }
    pub fn base(&self) -> Vec<(f32, f32)> {
        let mut ret = vec![];
        for m in &self.base {
            ret.push(m.borrow().values(0));
        }
        ret
    }
}
