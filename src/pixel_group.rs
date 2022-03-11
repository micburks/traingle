// max distance used to group pixels into bins
const BENEFICIAL_DISTANCE: f32 = 150.0;

// if main bin contains less than this percent of total pixels,
//  pixels are used from multiple bins to calculate color/fitness
const MIN_PIXEL_PERCENT: f32 = 0.95;

// bin must contain this percent of total pixels to be considered substantial
const SUBSTANTIAL_BIN_AREA_PERCENT_THRESHOLD: f32 = 0.01;

// entire group must contain this many pixels to have fitness
const TOTAL_GROUP_SIZE_THRESHOLD: i32 = 10;

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
                    color: image::Rgb([255, 0, 255]),
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
                let dist = distance(bin.mean, pixel);
                // if current pixel is close to this bin, push pixel into bin
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
        Group { color, fitness }
    }
    fn fitness(bins: &Vec<GroupBin>, total: i32, cumulative_distance_from_mean: f32) -> f32 {
        if total < TOTAL_GROUP_SIZE_THRESHOLD {
            return 0.0;
        }
        let mut substantial_bins: i32 = 0;
        let bin_threshold = ((total as f32) * SUBSTANTIAL_BIN_AREA_PERCENT_THRESHOLD) as i32;
        for bin in bins {
            if bin.count > bin_threshold {
                substantial_bins += 1;
            }
        }

        let group_size_multiplier = 1.0; // (total as f32) / 10.0;
        let main_bin_percent_size = bins[0].count as f32 / total as f32;
        let bin_size_multiplier = if substantial_bins == 0 {
            0.0
        } else if substantial_bins == 1 {
            total as f32
            // 1000.0
        } else if main_bin_percent_size > 0.9 {
            100.0
        } else if main_bin_percent_size > 0.75 {
            10.0
        } else {
            1.0
        };
        let bin_count_factor = 1.0 / (substantial_bins as f32).powf(2.0);
        let distance_factor = if cumulative_distance_from_mean < 10.0 {
            1.0
        } else if cumulative_distance_from_mean > 1000.0 {
            // 1 x 10^-6
            0.000_001
        } else {
            1.0 / cumulative_distance_from_mean.powf(3.0)
        };

        // only 1 bin
        // - reward GROUP size
        // multiple bins
        // - reward percent size of largest bin
        // - punish bins over 2
        // - punish distance from mean

        // reward total group size - ???
        group_size_multiplier
            // reward bin size
            * bin_size_multiplier
            // punish multiple substantial bins
            * bin_count_factor
            // punish distance from mean
            * distance_factor
    }
}

impl Copy for Group {}
impl Clone for Group {
    fn clone(&self) -> Group {
        Group {
            fitness: self.fitness,
            color: self.color,
        }
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
