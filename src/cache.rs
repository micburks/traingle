use super::point::Point;

use std::collections::HashMap;

pub struct Cache {
    store: HashMap<String, (f32, image::Rgb<u8>)>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            store: HashMap::new(),
        }
    }
    pub fn insert<F>(
        &mut self,
        p1: Point,
        p2: Point,
        p3: Point,
        mut f: F,
    ) -> (f32, image::Rgb<u8>)
    where
        F: FnMut() -> (f32, image::Rgb<u8>),
    {
        *self
            .store
            .entry(Self::hash(p1, p2, p3))
            .or_insert(f())
    }
    fn hash(p1: Point, p2: Point, p3: Point) -> String {
        format!("{},{}-{},{}-{},{}", p1.0, p1.1, p2.0, p2.1, p3.0, p3.1)
    }
}
