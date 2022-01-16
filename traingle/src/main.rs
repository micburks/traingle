use image::io::Reader as ImageReader;
use rand::prelude::*;
use rand_distr::StandardNormal;
use spade::delaunay::FloatDelaunayTriangulation;
use spade::{PointN, TwoDimensional};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    let img = ImageReader::open(filename)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();
    println!("w: {}, h: {}", width, height);

    // Convert image to set of pixels
    let pixels = img.pixels();

    // Create random points across image
    let mut points = vec![];
    let x_inc = 10;
    let y_inc = 10;
    for i in 0..x_inc + 1 {
        for j in 0..y_inc + 1 {
            points.push(Point(
                (i * (width / x_inc)) as f32,
                (j * (height / y_inc)) as f32,
            ));
        }
    }

    // Calculate delaunay triangles from points
    let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
    for p in points {
        delaunay.insert(p);
    }

    let mut tris: Vec<Face> = vec![];
    for face in delaunay.triangles() {
        let triangle = face.as_triangle();
        let centerpoint = (*triangle[0] + *triangle[1] + *triangle[2]) / 3.0;
        tris.push(Face::new(
            *triangle[0],
            *triangle[1],
            *triangle[2],
            *img.get_pixel(centerpoint.0 as u32, centerpoint.1 as u32),
        ));
    }

    // Calculate fitness and create 0th generation
    // Generation loop:
    // - Create generation from previous generation (new base members)
    // - Mutate each base member equal number of times
    // - Calculate fitness of each new member
    // - If fitness is higher than base member, its marked as beneficial
    // - Base members are copied again, mutating them with all beneficial mutations
    // - Calculate fitness of new mutated base members
    // - Sort all members by fitness

    // Rasterize image
    let num_pixels = width * height;
    let mut buf = vec![];
    '_outer: for i in 0..num_pixels {
        // find containing triangle
        let x = (i % width) as f32;
        let y = (i / width) as f32;
        for t in &tris {
            if t.contains(Point::new(x, y)) {
                let color = t.color();
                let [r, g, b] = color.0;
                buf.push(r as u8);
                buf.push(g as u8);
                buf.push(b as u8);
                continue '_outer;
            }
        }
    }

    // Write image
    println!("done here, pixels: {}/{}", buf.len(), width * height * 3);
    image::save_buffer(
        "output.jpg",
        &buf[..],
        width,
        height,
        image::ColorType::Rgb8,
    )?;
    Ok(())
}

#[derive(Debug)]
struct Point(f32, f32);

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point(x, y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1)
    }
}
impl std::ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.0 / rhs, self.1 / rhs)
    }
}
impl std::cmp::Eq for Point {}
impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self::new(self.0, self.1)
    }
}
impl PointN for Point {
    type Scalar = f32;
    fn dimensions() -> usize {
        2
    }
    fn from_value(v: f32) -> Self {
        Self::new(v, v)
    }
    fn nth(&self, s: usize) -> &f32 {
        if s == 0 {
            &self.0
        } else {
            &self.1
        }
    }
    fn nth_mut(&mut self, s: usize) -> &mut f32 {
        if s == 0 {
            &mut self.0
        } else {
            &mut self.1
        }
    }
}
impl TwoDimensional for Point {}

#[derive(Debug)]
struct Face(Point, Point, Point, image::Rgb<u8>);

impl Face {
    fn new(p1: Point, p2: Point, p3: Point, color: image::Rgb<u8>) -> Face {
        Face(p1, p2, p3, color)
    }
    fn contains(&self, p: Point) -> bool {
        let v0 = self.2 - self.0;
        let v1 = self.1 - self.0;
        let v2 = p - self.0;

        let d00 = dot(v0, v0);
        let d01 = dot(v0, v1);
        let d02 = dot(v0, v2);
        let d11 = dot(v1, v1);
        let d12 = dot(v1, v2);

        let inv_denom = 1.0 / det(Point(d00, d01), Point(d01, d11));
        let u = det(Point(d11, d01), Point(d12, d02)) * inv_denom;
        let v = det(Point(d00, d01), Point(d02, d12)) * inv_denom;
        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
    fn color(&self) -> image::Rgb<u8> {
        self.3
    }
}

fn dot(a: Point, b: Point) -> f32 {
    (a.0 * b.0) + (a.1 * b.1)
}

fn det(a: Point, b: Point) -> f32 {
    (a.0 * b.1) - (a.1 * b.0)
}

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    val
}

// will have to mess with normal distribution here
fn mutate(point: &Point) -> Point {
    Point(point.0 + (random() * 0.5), point.1 + (random() * 0.5))
}

// Fitness is the variance
fn fitness(face: &Face) -> f32 {
    // Calculate delaunay triangulation of a group of pixels
    // Iterate through pixels of each triangle and calculate variance to the target image
    1.3
}
