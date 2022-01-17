mod face;
mod point;

use face::Face;
use point::Point;

use image::io::Reader as ImageReader;
use spade::delaunay::FloatDelaunayTriangulation;

const X_SEGMENTS: u32 = 10;
const Y_SEGMENTS: u32 = 10;

fn get_points(w: u32, h: u32) -> Vec<Point> {
    // Create random points across image
    let mut points = vec![];
    for i in 0..X_SEGMENTS + 1 {
        for j in 0..Y_SEGMENTS + 1 {
            points.push(Point::new(
                i as f32 * (w as f32 / X_SEGMENTS as f32),
                j as f32 * (h as f32 / Y_SEGMENTS as f32),
            ));
        }
    }
    points
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    let img = ImageReader::open(filename)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();
    println!("w: {}, h: {}", width, height);

    let mut previous = get_points(width, height);
    let mut final_faces: Vec<Face> = vec![];

    for i in 0..1 {
        let points = if i == 0 {
            previous
        } else {
            previous
                .into_iter()
                .map(|p| p.clone().mutate(width, height))
                .collect()
        };

        // Calculate delaunay triangles from points
        let mut delaunay = FloatDelaunayTriangulation::with_walk_locate();
        for p in &points {
            delaunay.insert(*p);
        }

        let mut faces: Vec<Face> = vec![];
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            faces.push(Face::new(*triangle[0], *triangle[1], *triangle[2], &img));
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

        // go through pixels and add fitness calc to each member
        // after seeing all pixels of each member, go through members and move their fitness to each point (where fitness actually matters
        // make a copy of all

        // Get fitness from pixels
        let num_pixels = width * height;
        '_outer_fitness: for i in 0..num_pixels {
            // find containing triangle
            let x = (i % width) as f32;
            let y = (i / width) as f32;
            for f in &mut faces {
                if f.contains(Point::new(x, y)) {
                    let actual_color = *img.get_pixel(x as u32, y as u32);
                    f.add_fitness(actual_color);
                    continue '_outer_fitness;
                }
            }
        }

        // move face fitness to points
        for f in &mut faces {
            f.move_fitness();
        }

        previous = points;
        final_faces = faces;
        println!("{}", final_faces.len());
    }

    // Rasterize image
    let num_pixels = width * height;
    let mut buf = vec![];
    '_outer_raster: for i in 0..num_pixels {
        // find containing triangle
        let x = (i % width) as f32;
        let y = (i / width) as f32;
        for f in &mut final_faces {
            if f.contains(Point::new(x, y)) {
                let color = f.color();
                let [r, g, b] = color.0;
                buf.push(r as u8);
                buf.push(g as u8);
                buf.push(b as u8);
                continue '_outer_raster;
            }
        }
        // some edges are still not hitting
        buf.push(0);
        buf.push(0);
        buf.push(0);
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
