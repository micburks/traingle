mod img;
mod face;
mod cache;
mod point;
mod member;
mod triangle;
mod generation;
mod pixel_group;

use img::Img;
use generation::Generation;
use cache::Cache;

use std::time::Instant;
use image::io::Reader as ImageReader;

const SEGMENTS: u32 = 35;
const GENERATIONS: u32 = 20;
const MUTATIONS_PER_GENERATION: u32 = 20;

fn get_points((w, h): (f32, f32)) -> Vec<(f32, f32)> {
    // Create random points across image
    let mut points = vec![];
    for i in 0..SEGMENTS {
        for j in 0..SEGMENTS {
            points.push((
                i as f32 * (w / (SEGMENTS - 1) as f32),
                j as f32 * (h / (SEGMENTS - 1) as f32),
            ));
        }
    }
    points
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let filename = args.next().unwrap();
    let img = Img::new(
        ImageReader::open(filename)?.decode()?.to_rgb8(),
        SEGMENTS.pow(2) as f32,
    );
    println!("(w, h): {:?}", img.dimensions());

    let mut cache = Cache::new();

    let now = Instant::now();
    let mut previous;

    // Calculate fitness and create 0th generation
    let initial_points = get_points(img.dimensions());

    let mut gen = Generation::from(initial_points, &img, &mut cache);
    previous = gen.get_best_population();
    let time_to_generate = now.elapsed().as_secs();

    gen.write(String::from("output/output-0.jpg"), &mut previous);
    println!(
        "Generation 0, generated in {}s, written in {}s.",
        time_to_generate,
        now.elapsed().as_secs() - time_to_generate,
    );

    // Generation loop:
    for i in 0..GENERATIONS {
        let now = Instant::now();

        // - Create generation from previous generation (new base members)
        let mut gen = Generation::new(previous, &img, &mut cache);
        // - Mutate each base member equal number of times
        // - Calculate fitness of each new member
        // - If fitness is higher than base member, its marked as beneficial
        // - Base members are copied again, mutating them with all beneficial mutations
        // - Calculate fitness of new mutated base members
        gen.mutate(MUTATIONS_PER_GENERATION);

        // - Sort all members by fitness
        previous = gen.get_best_population();
        let time_to_generate = now.elapsed().as_secs();

        gen.write(format!("output/output-{}.jpg", i + 1), &mut previous);
        println!(
            "Generation {}, generated in {}s, written in {}s.",
            i + 1,
            time_to_generate,
            now.elapsed().as_secs() - time_to_generate,
        );
    }

    Ok(())
}
