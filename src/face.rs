use super::img::Img;
use super::member::Member;
use super::point::Point;

use spade::delaunay::{FaceHandle, VertexHandle};
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Face {
    pub points: (
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
        Rc<RefCell<Member>>,
    ),
    pub color: image::Rgb<u8>,
    pub fitness: f32,
    index: usize,
    pub hash: String,
    pub triangle: Triangle,
}

impl Face {
    pub fn new(
        del_triangle: [VertexHandle<Point, ()>; 3],
        members: &Vec<Rc<RefCell<Member>>>,
        index: usize,
        img: &Img,
    ) -> Face {
        let triangle = Triangle::new(del_triangle);

        let mut v1 = triangle.0;
        let mut v2 = triangle.1;
        let mut v3 = triangle.2;

        // Sort so v1.y < v2.y < v3.y
        if v2.1 > v1.1 {
            let tmp = v1;
            v1 = v2;
            v2 = tmp;
        }
        if v3.1 > v2.1 {
            let tmp = v2;
            v2 = v3;
            v3 = tmp;

            if v2.1 > v1.1 {
                let tmp = v1;
                v1 = v2;
                v2 = tmp;
            }
        }

        // Associate with members
        let mut m1_opt = None;
        let mut m2_opt = None;
        let mut m3_opt = None;
        for m in members {
            if let Some(_) = m1_opt {
                if let Some(_) = m2_opt {
                    if let Some(_) = m3_opt {
                        break;
                    }
                }
            }
            let point = *m.borrow().point(index);
            if point.0 == v1.0 && point.1 == v1.1 {
                m1_opt = Some(Rc::clone(&m));
                continue;
            }
            if point.0 == v2.0 && point.1 == v2.1 {
                m2_opt = Some(Rc::clone(&m));
                continue;
            }
            if point.0 == v3.0 && point.1 == v3.1 {
                m3_opt = Some(Rc::clone(&m));
                continue;
            }
        }
        let m1 = m1_opt.unwrap();
        let m2 = m2_opt.unwrap();
        let m3 = m3_opt.unwrap();

        let p1 = *m1.borrow().point(index);
        let p2 = *m2.borrow().point(index);
        let p3 = *m3.borrow().point(index);

        let dim = img.dimensions();
        let width = dim.0 as f32;
        let height = dim.1 as f32;

        let sq = |px: image::Rgb<u8>| {
            (px.0[0] as i32).pow(2) + (px.0[1] as i32).pow(2) + (px.0[2] as i32).pow(2)
        };

        /*
        let mut n = 0;
        let mut difference = 0.0;
        let block_size: usize = 1;
        let mut sr0: i32 = 0;
        let mut sg0: i32 = 0;
        let mut sb0: i32 = 0;
        let mut ssq: i32 = 0;

        let line = move |x0: usize, x1: usize, y: usize| -> Option<Pixel> {
            let mut new_pixels: Vec<(u32, u32)> = vec![];
            if x0 >= 0 && x1 < width as usize {
                for x in x0..x1 {
                    new_pixels.push((x as u32, y as u32));
                }
            }
            Option::Some(Pixel::Line(new_pixels, x1 as i32 - x0 as i32))
        };
        let block = move |x: usize, y: usize| -> Option<Pixel> {
            if x < width as usize {
                Option::Some(Pixel::Block(x as u32, y as u32))
            } else {
                None
            }
        };

        let pixel_data = Face::rasterize(p1, p2, p3, block_size, Box::new(line), Box::new(block));
        for op in pixel_data {
            if let Some(p) = op {
                match p {
                    Pixel::Line(pixels, n_val) => {
                        for pair in pixels {
                            let pixel = img.get_pixel(pair.0, pair.1);
                            sr0 += pixel.0[0] as i32;
                            sg0 += pixel.0[1] as i32;
                            sb0 += pixel.0[2] as i32;
                            ssq += sq(pixel);
                        }
                        n += n_val;
                    }
                    Pixel::Block(x, y) => {
                        let pixel = img.get_pixel(x, y);
                        sr0 += pixel.0[0] as i32;
                        sg0 += pixel.0[1] as i32;
                        sb0 += pixel.0[2] as i32;
                        ssq += sq(pixel);
                        n += (block_size as i32).pow(2);
                    }
                }
            }
        }

        let mut diff = 0.0;
        if n != 0 {
            diff = (ssq as f32)
                - (((sr0 as f32 * sr0 as f32)
                    + (sg0 as f32 * sg0 as f32)
                    + (sb0 as f32 * sb0 as f32))
                    / (n as f32));
        }
        difference += diff;
        let fitness = diff;
        */

        /*
        * for all triangles
        // Lower the fitness based on how many blank pixels there are (the smaller the area)
        // (As the triangles should cover the entire image)
        blank := float64(w*h) - area

        difference += maxPixelDifference * blank

        return 1 - (difference / t.maxDifference)
        */

        // find color
        let top_left = Point::new(min(p1.0, p2.0, p3.0), min(p1.1, p2.1, p3.1));
        let bottom_right = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));

        let difference = 0.0;
        let mut color_sum: (u32, u32, u32) = (0, 0, 0);
        let mut mean_color: (u32, u32, u32) = (0, 0, 0);
        let mut additions = vec![];
        let mut total_area = 0;
        let mut total_sq = 0;
        for x in (top_left.0 as usize)..(bottom_right.0 as usize) {
            for y in (top_left.1 as usize)..(bottom_right.1 as usize) {
                if triangle.contains(Point::new(x as f32, y as f32)) {
                    total_area += 1;
                    let add = img.get_pixel(x as u32, y as u32);
                    color_sum.0 += add.0[0] as u32;
                    color_sum.1 += add.0[1] as u32;
                    color_sum.2 += add.0[2] as u32;
                    additions.push(add);
                }
            }
        }

        // set mean color
        if total_area != 0 {
            mean_color.0 = color_sum.0 / total_area;
            mean_color.1 = color_sum.1 / total_area;
            mean_color.2 = color_sum.2 / total_area;
        }

        // calculate covar
        let mut c = 0;
        for add in additions {
            let dr = add.0[0] as i32 - mean_color.0 as i32;
            let dg = add.0[1] as i32 - mean_color.1 as i32;
            let db = add.0[2] as i32 - mean_color.2 as i32;
            c += dr * dr + dg * dg + db * db;
        }
        let fitness = if c != 0 { 1.0 / c as f32 } else { 0.0 };

        let hash = Face::hash(p1, p2, p3);
        Face {
            points: (m1, m2, m3),
            color: image::Rgb([mean_color.0 as u8, mean_color.1 as u8, mean_color.2 as u8]),
            fitness,
            index,
            hash,
            triangle,
        }
    }
    pub fn hash(p1: Point, p2: Point, p3: Point) -> String {
        let mut s: Vec<Point> = vec![p1, p2, p3];
        for i in 0..3 {
            let mut small = i;
            for j in (i + 1)..3 {
                if s[j].0 < s[small].0 {
                    small = j;
                }
            }
            s.swap(small, i);
        }
        format!(
            "{}-{},{}-{},{}-{}",
            s[0].0, s[0].1, s[1].0, s[1].1, s[2].0, s[2].1
        )
    }
    /*
    pub fn add_fitness(&mut self, color: image::Rgb<u8>) -> () {
        let face_color = self.color.0;
        let dr = (face_color[0] as i32) - (color.0[0] as i32);
        let dg = (face_color[1] as i32) - (color.0[1] as i32);
        let db = (face_color[2] as i32) - (color.0[2] as i32);
        // diff = [0, 1, 4, 9, ...]
        let diff = (dr.pow(2) + dg.pow(2) + db.pow(2)) as f32;
        if diff == 0.0 {
            self.fitness += 100.0;
        } else {
            self.fitness += 1.0 / diff;
        }
    }
    */
    pub fn move_fitness(&mut self) -> () {
        self.points
            .0
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
        self.points
            .1
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
        self.points
            .2
            .borrow_mut()
            .add_fitness(self.index, self.fitness);
    }
    fn rasterize<'a>(
        p0: Point,
        p1: Point,
        p2: Point,
        block_size: usize,
        mut line: Box<dyn FnMut(usize, usize, usize) -> Option<Pixel>>,
        mut block: Box<dyn FnMut(usize, usize) -> Option<Pixel>>,
    ) -> Vec<Option<Pixel>> {
        // normalTriangleBlocks rasterizes the lines of a triangle, while trying to rasterize in blocks when possible.
        let mut pixels: Vec<Option<Pixel>> = vec![];

        // Calculate the slopes of the first two lines
        let mut m0 = p2.0 - p0.0 / p2.1 - p0.1;
        let mut m1 = p2.0 - p1.0 / p2.1 - p1.1;

        // Swap the slopes so m0 is the slope of the left line and m1 is the slope of the right line
        let swap = m0 > m1;
        if swap {
            let tmp = m0;
            m0 = m1;
            m1 = tmp;
        }

        // Start from the top vertex
        let mut lx0 = p2.0;
        let mut lx1 = p2.0;

        // Starting from the bottom y coordinate, iterate upwards through the pixels using incrementing by blockSize
        // let i := y1 - 1
        // for ; i > y2; i -= blockSize {
        let mut start = (p1.1 - 1.0) as usize;
        for i in (start..(p2.1 as usize)).step_by(block_size) {
            // ?
            let top = i - block_size + 1;

            let mut bottom_x = m0 * (i as f32 - p2.1) + lx0;
            let mut top_x = m0 * (top as f32 - p2.1) + lx0;
            let max_x = cmp::max(bottom_x as u32, top_x as u32) as f32;

            bottom_x = m1 * (i as f32 - p2.1) + lx1;
            top_x = m1 * (top as f32 - p2.1) + lx1;
            let min_x = cmp::min(bottom_x as u32, top_x as u32) as f32;

            // Leave the loop if the remaining triangle isn't wide enough to rasterize blocks
            if max_x as usize + block_size >= min_x as usize {
                break;
            }

            // Fill in the left section of the triangle where blocks can't be rasterized
            for y in 0..block_size {
                let iy = (i - y) as f32;
                let px0 = m0 * (iy - p2.1) + lx0;
                pixels.push(line(px0 as usize, max_x as usize, iy as usize));
            }

            // Rasterize the middle section of the triangle in blocks
            let mut x_out = max_x;
            for x in ((max_x as usize)..(min_x as usize)).step_by(block_size) {
                if (x + block_size) >= min_x as usize {
                    break;
                }
                pixels.push(block(x as usize, (i - block_size + 1) as usize));
                x_out = x as f32;
            }

            // Fill in the right section of the triangle where blocks can't be rasterized
            for y in 0..block_size {
                let iy = (i - y) as f32;
                let px1 = m1 * (iy - p2.1) + lx1;
                pixels.push(line(x_out as usize, px1 as usize, iy as usize));
            }
            start = i;
        }

        // Rasterize the remaining part of the top triangle with pixels
        for i in start..(p2.1 as usize) {
            // for ; i > y2; i-- {
            let px0 = m0 * (i as f32 - p2.1) + lx0;
            let px1 = m1 * (i as f32 - p2.1) + lx1;

            pixels.push(line(px0 as usize, px1 as usize, i));
        }

        // Calculate the new slope for the line that changed, and repeat the process above
        let mut d0 = 0.0;
        let mut d1 = 0.0;

        if swap {
            m0 = (p1.0 - p0.0) / (p1.1 - p0.1);
            lx0 = p1.0;
            d1 = p1.1 - p2.1;
        } else {
            m1 = (p1.0 - p0.0) / (p1.1 - p0.1);
            lx1 = p1.0;
            d0 = p1.1 - p2.1;
        }

        if p1.1 == p2.1 {
            lx0 = p2.0;
            lx1 = p1.0;

            if lx0 > lx1 {
                let tmp = lx0;
                lx0 = lx1;
                lx1 = tmp;
            }
            if m0 < m1 {
                let tmp = m0;
                m0 = m1;
                m1 = tmp;
            }
        }

        start = p1.1 as usize;

        // Starting from the top y coordinate, iterate downwards through the pixels using incrementing by blockSize
        for i in (start..(p0.1 as usize)).step_by(block_size) {
            if i + block_size >= p0.1 as usize {
                break;
            }
            let top = i + block_size - 1;

            let mut bottom_x = m0 * (i as f32 - p1.1 + d0) + lx0;
            let mut top_x = m0 * (top as f32 - p1.1 + d0) + lx0;
            let max_x = cmp::max(bottom_x as i32, top_x as i32) as f32;

            bottom_x = m1 * (i as f32 - p1.1 + d1) + lx1;
            top_x = m1 * (top as f32 - p1.1 + d1) + lx1;
            let min_x = cmp::min(bottom_x as i32, top_x as i32) as f32;

            // Leave the loop if the remaining triangle isn't wide enough to rasterize blocks
            if (max_x as usize + block_size) >= min_x as usize {
                break;
            }

            // Fill in the right section of the triangle where blocks can't be rasterized
            for y in 0..block_size {
                let iy = (i + y) as f32;
                let px0 = m0 * (iy - p1.1 + d0) + lx0;
                pixels.push(line(px0 as usize, max_x as usize, iy as usize));
            }

            // Rasterize the middle section of the triangle in blocks
            let mut x_out = max_x as usize;
            for x in (x_out..(min_x as usize)).step_by(block_size) {
                if x + block_size >= min_x as usize {
                    break;
                }
                pixels.push(block(x, i));
                x_out = x;
            }

            // Fill in the right section of the triangle where blocks can't be rasterized
            for y in 0..block_size {
                let iy = (i + y) as f32;
                let px1 = m1 * (iy - p1.1 + d1) + lx1;
                pixels.push(line(x_out, px1 as usize, iy as usize));
            }
            start = i;
        }

        // Rasterize the remaining part of the bottom triangle with pixels
        for i in start..p0.1 as usize {
            let px0 = m0 * (i as f32 - p1.1 + d0) + lx0;
            let px1 = m1 * (i as f32 - p1.1 + d1) + lx1;
            pixels.push(line(px0 as usize, px1 as usize, i));
        }

        pixels
    }
}

enum Pixel {
    Block(u32, u32),
    Line(Vec<(u32, u32)>, i32),
}

#[derive(Debug)]
pub struct FaceFinder<'a> {
    faces: &'a mut Vec<Face>,
    map: &'a HashMap<String, usize>,
    last_index: Option<i32>,
}

impl<'a> FaceFinder<'a> {
    pub fn new(faces: &'a mut Vec<Face>, map: &'a HashMap<String, usize>) -> FaceFinder<'a> {
        FaceFinder {
            faces,
            map,
            last_index: None,
        }
    }
    pub fn find(&mut self, x: f32, y: f32) -> Option<&mut Face> {
        let start = match self.last_index {
            Some(ind) => ind,
            None => 0,
        };
        let end = self.faces.len() as i32;
        let (mut i, mut j) = (start, start + 1);
        while i >= 0 || j < end {
            if i >= 0 {
                if self.faces[i as usize].triangle.contains(Point::new(x, y)) {
                    self.last_index = Some(i as i32);
                    return Option::Some(&mut self.faces[i as usize]);
                }
                i -= 1;
            }
            if j < end {
                if self.faces[j as usize].triangle.contains(Point::new(x, y)) {
                    self.last_index = Some(j as i32);
                    return Option::Some(&mut self.faces[j as usize]);
                }
                j += 1;
            }
        }
        // try to use a nearby pixel
        if x == 0.0 && y == 0.0 {
            return None;
        } else if x == 0.0 {
            return self.find(x, y - 1.0);
        } else if y == 0.0 {
            return self.find(x - 1.0, y);
        } else {
            return self.find(x - 1.0, y - 1.0);
        }
    }
}

#[derive(Debug)]
pub struct Triangle(Point, Point, Point, Point, (bool, bool));

impl Triangle {
    pub fn new(t: [VertexHandle<Point, ()>; 3]) -> Triangle {
        let p1 = *t[0];
        let p2 = *t[1];
        let p3 = *t[2];

        let max = Point::new(max(p1.0, p2.0, p3.0), max(p1.1, p2.1, p3.1));

        // does this triangle lie against the x=0 line?
        let vertical0 = (p1.0 == 0.0 && p2.0 == 0.0)
            || (p1.0 == 0.0 && p3.0 == 0.0)
            || (p2.0 == 0.0 && p3.0 == 0.0);

        // does this triangle lie against the y=0 line?
        let horizontal0 = (p1.1 == 0.0 && p2.1 == 0.0)
            || (p1.1 == 0.0 && p3.1 == 0.0)
            || (p2.1 == 0.0 && p3.1 == 0.0);

        Triangle(p1, p2, p3, max, (vertical0, horizontal0))
    }
    pub fn contains(&self, p: Point) -> bool {
        let x = p.0;
        let y = p.1;

        // if this is less than this triangle's top-left boundary box, skip
        if x > self.3 .0 && y > self.3 .1 {
            return false;
        }

        // x=0.0 line
        if self.4 .0 && x == 0.0 {
            if y >= self.0.1 && y <= self.1.1 || // 0 <= y <= 1
                y >= self.0.1 && y <= self.2.1 || // 0 <= y <= 2
                y >= self.1.1 && y <= self.0.1 || // 1 <= y <= 0
                y >= self.1.1 && y <= self.2.1 || // 1 <= y <= 2
                y >= self.2.1 && y <= self.0.1 || // 2 <= y <= 0
                y >= self.2.1 && y <= self.1.1
            // 2 <= y <= 1
            {
                return true;
            }
        }

        // x=0.0 line
        if self.4 .1 && y == 0.0 {
            if x >= self.0.0 && x <= self.1.0 || // 0 <= x <= 1
                x >= self.0.0 && x <= self.2.0 || // 0 <= x <= 2
                x >= self.1.0 && x <= self.0.0 || // 1 <= x <= 0
                x >= self.1.0 && x <= self.2.0 || // 1 <= x <= 2
                x >= self.2.0 && x <= self.0.0 || // 2 <= x <= 0
                x >= self.2.0 && x <= self.1.0
            // 2 <= x <= 1
            {
                return true;
            }
        }

        // exact vertex matches
        if x == self.0 .0 && y == self.0 .1 {
            return true;
        }
        if x == self.1 .0 && y == self.1 .1 {
            return true;
        }
        if x == self.2 .0 && y == self.2 .1 {
            return true;
        }

        /*
        let v0 = self.2 - self.0;
        let v1 = self.1 - self.0;
        let v2 = p - self.0;

        let d00 = dot(v0, v0);
        let d01 = dot(v0, v1);
        let d02 = dot(v0, v2);
        let d11 = dot(v1, v1);
        let d12 = dot(v1, v2);

        let inv_denom = 1.0 / det(Point::new(d00, d01), Point::new(d01, d11));
        let u = det(Point::new(d11, d01), Point::new(d12, d02)) * inv_denom;
        let v = det(Point::new(d00, d01), Point::new(d02, d12)) * inv_denom;
        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
        */

        // rewritten to be incomprehensible
        let v0x = self.2 .0 - self.0 .0;
        let v0y = self.2 .1 - self.0 .1;
        let v1x = self.1 .0 - self.0 .0;
        let v1y = self.1 .1 - self.0 .1;
        let v2x = x - self.0 .0;
        let v2y = y - self.0 .1;

        let d00 = (v0x * v0x) + (v0y * v0y);
        let d01 = (v0x * v1x) + (v0y * v1y);
        let d02 = (v0x * v2x) + (v0y * v2y);
        let d11 = (v1x * v1x) + (v1y * v1y);
        let d12 = (v1x * v2x) + (v1y * v2y);

        let inv_denom = 1.0 / ((d00 * d11) - (d01 * d01));
        let u = ((d11 * d02) - (d01 * d12)) * inv_denom;
        let v = ((d00 * d12) - (d01 * d02)) * inv_denom;
        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
}

/*
fn dot(a: Point, b: Point) -> f32 {
    (a.0 * b.0) + (a.1 * b.1)
}

fn det(a: Point, b: Point) -> f32 {
    (a.0 * b.1) - (a.1 * b.0)
}
*/

fn min(a: f32, b: f32, c: f32) -> f32 {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}

fn max(a: f32, b: f32, c: f32) -> f32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}
