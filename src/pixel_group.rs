const BENEFICIAL_DISTANCE: f32 = 100.0;
const MIN_PIXEL_PERCENT: f32 = 0.75;
const BIN_PERCENT_THRESHOLD: f32 = 0.05;

pub struct Group {
    pub color: image::Rgb<u8>,
    pub fitness: f32,
}

impl Group {
    pub fn new<I>(pixels: &mut I) -> Group
    where
        I: Iterator<Item = (f32, f32, f32)>,
    {
        let first_pixel = match pixels.next() {
            Some(p) => p,
            None => {
                // println!("pixels empty?");
                return Group {
                    fitness: 0.0,
                    color: image::Rgb([0, 0, 0]),
                };
            }
        };

        // first pixel creates first bin
        let mut total = 1;
        let mut bins = vec![GroupBin::new(first_pixel)];
        // loop pixels - pixel in pixels
        '_pixels: for pixel in pixels {
            total += 1;
            // loop bins - bin in bins
            for bin in &mut bins {
                // if current pixel is close to this bin, push pixel into bin
                let dist = distance(bin.mean, pixel);
                // - if distance(pixel, bin.mean) < threshold { bin.values.add(pixel) }
                if dist < BENEFICIAL_DISTANCE {
                    // - bin.values.add - calculates moving mean, size
                    bin.add(pixel);
                    continue '_pixels;
                }
            }
            // if no bins match, create new one
            bins.push(GroupBin::new(pixel));
        }

        // sort bins by size descending
        bins.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap());

        let min_pixel_count = (total as f32) * MIN_PIXEL_PERCENT;
        let color;
        let fitness;
        if (bins[0].count as f32) < min_pixel_count {
            let mut mean = (0.0, 0.0, 0.0);
            // let mut m_2 = (0.0, 0.0, 0.0);
            let mut count = 0.0;
            let mut index = 0;
            let mut bin_index = 0;
            for _i in 0..(min_pixel_count as usize) {
                if index >= bins[bin_index].values.len() {
                    index = 0;
                    bin_index += 1;
                }
                let pixel = bins[bin_index].values[index];
                count += 1.0;
                let delta = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                mean = (
                    mean.0 + (delta.0 / count),
                    mean.1 + (delta.1 / count),
                    mean.2 + (delta.2 / count),
                );
                /*
                let delta2 = (pixel.0 - mean.0, pixel.1 - mean.1, pixel.2 - mean.2);
                m_2 = (
                    m_2.0 + (delta.0 * delta2.0),
                    m_2.1 + (delta.1 * delta2.1),
                    m_2.2 + (delta.2 * delta2.2),
                );
                */
                index += 1;
            }
            let mut cumulative_distance_from_mean = 0.0;
            let mut index = 0;
            let mut bin_index = 0;
            for _i in 0..(min_pixel_count as usize) {
                if index >= bins[bin_index].values.len() {
                    index = 0;
                    bin_index += 1;
                }
                let pixel = bins[bin_index].values[index];
                cumulative_distance_from_mean += distance(mean, pixel);
                index += 1;
            }
            fitness = Group::fitness(&bins, total, cumulative_distance_from_mean);
            color = image::Rgb([mean.0 as u8, mean.1 as u8, mean.2 as u8]);
        } else {
            let mean = bins[0].mean;
            let mut cumulative_distance_from_mean = 0.0;
            for pixel in &bins[0].values {
                cumulative_distance_from_mean += distance(mean, *pixel);
            }
            fitness = Group::fitness(&bins, total, cumulative_distance_from_mean);
            color = image::Rgb([mean.0 as u8, mean.1 as u8, mean.2 as u8]);
        }
        Group {
            color,
            fitness,
        }
    }
    fn fitness(bins: &Vec<GroupBin>, total: i32, cumulative_distance_from_mean: f32) -> f32 {
        let mut substantial_bins: i32 = 0;
        let bin_threshold = ((total as f32) * BIN_PERCENT_THRESHOLD) as i32;
        for bin in bins {
            if bin.count > bin_threshold {
                substantial_bins += 1;
            }
        }
        let bin_size_multiplier = if substantial_bins == 0 {
            0.0
        } else if substantial_bins == 1 {
            1.0
        } else {
            bins[0].count as f32 / total as f32
        };

        /*
        let n_bins_sq: i32 = substantial_bins.pow(2);
        let largest_bin_coverage = bins[0].count / total;
        // (1 / n_bins**2) * (bin_area / total_area)
        (1.0 / n_bins_sq as f32) * largest_bin_coverage as f32
        */

        // only 1 bin
        // - reward size
        // multiple bins
        // - reward percent size of largest bin
        // - punish bins over 2
        // - punish distance from mean

        // reward bin size
        bin_size_multiplier
            // punish multiple substantial bins
            * (1.0 / substantial_bins as f32)
            // punish distance from mean
            * (1000000.0 / cumulative_distance_from_mean as f32)
    }
}

fn distance(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    let distance = (a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0) + (a.2 - b.2).powf(2.0);
    distance
}

struct GroupBin {
    count: i32,
    mean: (f32, f32, f32),
    values: Vec<(f32, f32, f32)>,
}

impl GroupBin {
    fn new(pixel: (f32, f32, f32)) -> GroupBin {
        GroupBin {
            count: 1,
            mean: (pixel.0, pixel.1, pixel.2),
            values: vec![pixel],
        }
    }
    fn add(&mut self, pixel: (f32, f32, f32)) -> () {
        self.values.push(pixel);
        self.count += 1;
        let delta = (
            pixel.0 - self.mean.0,
            pixel.1 - self.mean.1,
            pixel.2 - self.mean.2,
        );
        self.mean = (
            self.mean.0 + (delta.0 / (self.count as f32)),
            self.mean.1 + (delta.1 / (self.count as f32)),
            self.mean.2 + (delta.2 / (self.count as f32)),
        );
    }
}
