mod face;
mod generation;
mod img;
mod member;
mod point;

use generation::Generation;
use img::Img;

use image::io::Reader as ImageReader;
use std::time::Instant;

const X_SEGMENTS: u32 = 10;
const Y_SEGMENTS: u32 = 10;
const GENERATIONS: u32 = 5;
const MUTATIONS_PER_GENERATION: u32 = 2;

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

    let now = Instant::now();

    // Calculate fitness and create 0th generation
    let initial_points = get_points(img.dimensions());
    let gen = Generation::new(initial_points, &img);
    let (faces, del) = gen.triangulate(0);
    gen.write_faces(String::from("output-0.jpg"), faces, del);
    let mut previous = gen.base();

    let elapsed_time = now.elapsed();
    println!("Generation 0 took {} seconds.", elapsed_time.as_secs());

    // Generation loop:
    for i in 0..GENERATIONS {
        let now = Instant::now();

        // - Create generation from previous generation (new base members)
        let mut gen = Generation::new(previous, &img);
        // - Mutate each base member equal number of times
        // - Calculate fitness of each new member
        // - If fitness is higher than base member, its marked as beneficial
        gen.mutate(MUTATIONS_PER_GENERATION);
        // - Base members are copied again, mutating them with all beneficial mutations
        // - Calculate fitness of new mutated base members
        gen.aggregate_beneficial_mutations();
        // - Sort all members by fitness
        previous = gen.get_best_points();
        gen.write(format!("output-{}.jpg", i + 1));

        let elapsed_time = now.elapsed();
        println!("Generation {} took {} seconds.", i + 1, elapsed_time.as_secs());
    }

    Ok(())
}
