use super::cache::Cache;
use super::face::{Face, FaceFinder};
use super::img::Img;
use super::member::{Member, MemberType};
use super::point::Point;

use spade::delaunay::FloatDelaunayTriangulation;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
//use spade::delaunay::{DelaunayTriangulation, DelaunayWalkLocate, FloatDelaunayTriangulation};

pub struct Generation<'a> {
    base: Vec<Rc<RefCell<Member>>>,
    pub img: &'a Img,
    mutations: usize,
    populations: Vec<Population>,
    pub cache: &'a mut Cache,
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
    pub fn new(previous: Population, img: &'a Img, cache: &'a mut Cache) -> Generation<'a> {
        let base: Vec<Rc<RefCell<Member>>> = previous.points
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
            .collect();

        let mut gen = Generation {
            base: vec![],
            img,
            mutations: 0,
            populations: vec![],
            cache,
        };

        let pop = Generation::triangulate(&mut gen, &base);
        gen.populations.push(pop);
        gen.base = base;
        gen
    }
    pub fn from(points: Vec<(f32, f32)>, img: &'a Img, cache: &'a mut Cache) -> Generation<'a> {
        let base: Vec<Rc<RefCell<Member>>> = points
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
            .collect();

        let mut gen = Generation {
            base: vec![],
            img,
            mutations: 0,
            populations: vec![],
            cache,
        };

        let pop = Generation::triangulate(&mut gen, &base);
        gen.populations.push(pop);
        gen.base = base;
        gen
    }
    pub fn mutate(&mut self, n: u32) -> () {
        for _i in 0..n {
            let mut members = vec![];
            for point in &mut self.base {
                members.push(point.borrow_mut().mutate());
            }
            let pop = Generation::triangulate(self, &members);
            self.populations.push(pop);
        }

        // aggregate best mutations
        let mut members = vec![];
        for base_member in &self.base {
            members.push(base_member.borrow_mut().merge_mutations_into_base());
        }
        let pop = Generation::triangulate(self, &members);
        self.populations.push(pop);
    }
    // Create triangles from a set of points
    // Calculate fitness of each triangle and aggregate in each member
    fn triangulate(generation: &mut Generation, members: &Vec<Rc<RefCell<Member>>>) -> Population {
        // Calculate delaunay triangles from points
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut points = vec![];
        for m in members {
            let point = &m.borrow().point;
            delaunay.insert(**point);
            points.push(point.values());
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(
                triangle,
                members,
                generation,
            ));
        }

        Population::new(faces, points)
    }
    pub fn get_best_population(&mut self) -> Population {
        let points = self.get_best_points();

        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        let mut members = vec![];
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
            faces.push(Face::new(triangle, &members, self));
        }

        Population::new(faces, points)
    }
    fn get_best_points(&self) -> Vec<(f32, f32)> {
        let mut sorted_faces = vec![];
        for pop in &self.populations {
            for face in &pop.faces {
                sorted_faces.push(face);
            }
        }
        sorted_faces.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let mut points: Vec<(f32, f32)> = vec![];
        let mut seen: HashSet<usize> = HashSet::new();
        let mut sum = 0.0;
        for i in 0..sorted_faces.len() {
            if points.len() >= self.img.points() as usize {
                break;
            }
            let face = sorted_faces[i];
            let m1 = face.points.0.borrow();
            if !seen.contains(&m1.id) {
                points.push(m1.point.values());
                seen.insert(m1.id);
                sum += m1.fitness;
            }
            let m2 = face.points.1.borrow();
            if !seen.contains(&m2.id) {
                points.push(m2.point.values());
                seen.insert(m2.id);
                sum += m2.fitness;
            }
            let m3 = face.points.2.borrow();
            if !seen.contains(&m3.id) {
                points.push(m3.point.values());
                seen.insert(m3.id);
                sum += m3.fitness
            }
        }
        println!("average fitness {}", sum / points.len() as f32);
        points
    }
    pub fn write(&self, filename: String, population: &mut Population) -> () {
        let (width, height) = self.img.dimensions();

        // Rasterize image
        let num_pixels = (width * height) as u32;
        let mut face_finder = FaceFinder::new(&mut population.faces);
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
                        None => [0, 255, 255],
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
                population.faces.len(),
                buf.len(),
                width * height * 3.0
            ),
            Err(e) => println!("error {}", e),
        }
    }
}
