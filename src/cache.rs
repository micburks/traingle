use super::geom::Point;
use super::pixel_group::Group;

use std::collections::HashMap;

pub struct Cache {
    store: HashMap<String, Group>,
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
    ) -> Group
    where
        F: FnMut() -> Group,
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
