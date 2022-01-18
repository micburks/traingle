mod face;
mod generation;
mod img;
mod member;
mod point;

use generation::Generation;
use img::Img;

use image::io::Reader as ImageReader;

const X_SEGMENTS: u32 = 20;
const Y_SEGMENTS: u32 = 20;
const GENERATIONS: u32 = 5;
const MUTATIONS_PER_GENERATION: u32 = 5;

fn get_points((w, h): (f32, f32)) -> Vec<(f32, f32)> {
    // Create random points across image
    let mut points = vec![];
    for i in 0..X_SEGMENTS + 1 {
        for j in 0..Y_SEGMENTS + 1 {
            points.push((
                i as f32 * (w / X_SEGMENTS as f32),
                j as f32 * (h / Y_SEGMENTS as f32),
            ));
        }
    }
    points
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    let img = Img::new(ImageReader::open(filename)?.decode()?.to_rgb8());
    println!("(w, h): {:?}", img.dimensions());

    let initial_points = get_points(img.dimensions());
    // let mut final_faces: Vec<Face> = vec![];

    // Calculate fitness and create 0th generation
    let gen = Generation::new(initial_points, &img);
    gen.triangulate_and_calculate_fitness(0);
    gen.write(String::from("output-0.jpg"));
    let mut previous = gen.base();

    // Generation loop:
    for i in 0..GENERATIONS {
        // - Create generation from previous generation (new base members)
        let mut gen = Generation::new(previous, &img);
        // - Mutate each base member equal number of times
        // - Calculate fitness of each new member
        // - If fitness is higher than base member, its marked as beneficial
        gen.mutate(MUTATIONS_PER_GENERATION);
        // - Base members are copied again, mutating them with all beneficial mutations
        // - Calculate fitness of new mutated base members
        gen.copy_base_members_with_beneficial_mutations();
        // - Sort all members by fitness
        previous = gen.get_best_points();
        gen.write(format!("output-{}.jpg", i + 1));
    }

    /*
    for i in 0..5 {
        // generation created and fitness calculated by end of loop
        let points = if i == 0 {
            previous
        } else {
            // Mutate previous generation to
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
        println!("done, face: {}, pixels: {}/{}", final_faces.len(), buf.len(), width * height * 3);
        image::save_buffer(
            format!("output-{}.jpg", i),
            &buf[..],
            width,
            height,
            image::ColorType::Rgb8,
        )?;
    }
    */

    Ok(())
}
