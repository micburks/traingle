use rand::prelude::*;
use rand_distr::StandardNormal;
use image::io::Reader as ImageReader;
use spade::{PointN, TwoDimensional};
use spade::delaunay::FloatDelaunayTriangulation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read in image
    let img = ImageReader::open("test_image.jpg")?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();
    println!("w: {}, h: {}", width, height);

    // Convert image to set of pixels
    let pixels = img.pixels();

    // Create random points across image
    let mut points = vec![];
    let x_inc = 10;
    let y_inc = 10;
    for i in 0..x_inc {
        for j in 0..y_inc {
            points.push(
                Point(
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
        tris.push(
            Face::new(
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
            if t.contains(x, y) {
                let color = t.color();
                let [r, g, b] = color.0;
                buf.push(r as u8);
                buf.push(g as u8);
                buf.push(b as u8);
                continue '_outer;
            }
        }
        // make purple to debug why we get here
        buf.push(200);
        buf.push(0);
        buf.push(200);
        /*
        let x: f32 = (i % width) as f32;
        let y: f32 = (i / width) as f32;
        let r = x / width as f32;
        let g = y / height as f32;
        let b = 0.25;
        buf.push((r * 256.0) as u8);
        buf.push((g * 256.0) as u8);
        buf.push((b * 256.0) as u8);
        */
    }

    // Write image
    println!("done here, {} but should be {}", buf.len(), width * height * 3);
    image::save_buffer("output.jpg", &buf[..], width, height, image::ColorType::Rgb8)?;
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
    fn clone(&self) -> Self{
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
struct Face (
    Point,
    Point,
    Point,
    image::Rgb<u8>,
);

impl Face {
    fn new(p1: Point, p2: Point, p3: Point, color: image::Rgb<u8>) -> Face {
        Face(p1, p2, p3, color)
    }
    fn contains(&self, x: f32, y: f32) -> bool {
        let v = Point::new(x, y);
        let a = (det(v, self.2) - det(self.0, self.2)) / det(self.1, self.2);
        let b = -((det(v, self.1) - det(self.0, self.1)) / det(self.1, self.2));
        a > 0.0 && b > 0.0 && (a + b) < 1.0
    }
    fn color(&self) -> image::Rgb<u8> {
        self.3
    }
}

// determinant
fn det(a: Point, b: Point) -> f32 {
   (a.0 * b.1) - (a.1 * b.0)
}

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    val
}

// will have to mess with normal distribution here
fn mutate(point: &Point) -> Point {
    Point (
        point.0 + (random() * 0.5),
        point.1 + (random() * 0.5),
    )
}

// Fitness is the variance
fn fitness(face: &Face) -> f32 {
    // Calculate delaunay triangulation of a group of pixels
    // Iterate through pixels of each triangle and calculate variance to the target image
    1.3
}
