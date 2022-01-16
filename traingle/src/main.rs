mod point;
mod face;

use point::Point;
use face::Face;

use image::io::Reader as ImageReader;
use rand::prelude::*;
use rand_distr::StandardNormal;
use spade::delaunay::FloatDelaunayTriangulation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    let img = ImageReader::open(filename)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();
    println!("w: {}, h: {}", width, height);

    // Convert image to set of pixels
    // let pixels = img.pixels();

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
