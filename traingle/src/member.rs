use super::point::Point;

use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug)]
pub struct Member {
    source: MemberType,
    point: Box<Point>,
    mutations: Vec<Member>,
    size: usize,
    dimensions: (f32, f32),
}

impl Member {
    pub fn new(source: MemberType, point: (f32, f32), dimensions: (f32, f32)) -> Member {
        Member {
            source: source.clone(),
            point: match source {
                MemberType::Base => Box::new(Point::new(point.0, point.1)),
                MemberType::Mutation(delta) => {
                    Box::new(Point::from(point).mutate(delta, dimensions))
                }
            },
            mutations: vec![],
            size: 0,
            dimensions,
        }
    }
    pub fn mutate(&mut self) -> () {
        // will have to mess with normal distribution here
        let random_point = Point::new(random(), random());
        self.mutations.push(Member::new(
            MemberType::Mutation(random_point),
            self.point.values(),
            self.dimensions,
        ));
        self.size += 1;
    }
    pub fn point(&self, index: usize) -> &Point {
        if index == 0 {
            &self.point
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            &self.mutations[index - 1].point(0)
        }
    }
    pub fn merge_mutations_into_base(&mut self) -> () {
        let mut aggregate = (*self.point).clone();
        for mutation in &self.mutations {
            match mutation.source {
                MemberType::Mutation(delta) => {
                    if aggregate.is_beneficial() {
                        aggregate.mutate(delta, self.dimensions);
                    }
                }
                _ => (),
            }
        }
        self.mutations.push(Member::new(
            MemberType::Base,
            aggregate.values(),
            self.dimensions,
        ));
        self.size += 1;
    }
    pub fn set_to_beneficial(&mut self) -> () {
        self.point.set_to_beneficial();
    }
    pub fn check_if_beneficial(&mut self, index: usize) -> () {
        // println!("{} {}", self.point(0).fitness(), self.point(index).fitness());
        if index == 0 {
            return;
        }
        if self.point(0).fitness() < self.point(index).fitness() {
            if index > self.size {
                panic!("size out of bounds");
            }
            self.mutations[index - 1].set_to_beneficial();
        }
    }
    pub fn add_fitness(&mut self, index: usize, fitness: f32) -> () {
        if index == 0 {
            self._add_fitness(fitness);
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            self.mutations[index - 1]._add_fitness(fitness);
        }
    }
    pub fn _add_fitness(&mut self, fitness: f32) -> () {
        self.point.add_fitness(fitness);
    }
    pub fn get_best(&mut self) -> (f32, f32) {
        let mut index = 0;
        let mut highest = self.point(0).fitness();
        for (i, mutation) in (&self.mutations).into_iter().enumerate() {
            if mutation.point(0).fitness() > highest {
                index = i;
                highest = mutation.point(0).fitness();
            }
        }
        self.point(index).values()
    }
    pub fn values(&self, index: usize) -> (f32, f32) {
        self.point(index).values()
    }
}

/*
impl Clone for Member {
    fn clone(&self) -> Member {
        Member {
            source: self.source.clone(),
            point: self.point.clone(),
            mutations: self.mutations.to_vec(),
            size: self.size,
            dimensions: self.dimensions,
        }
    }
}
*/

#[derive(Debug)]
pub enum MemberType {
    Base,
    Mutation(Point),
}

impl Clone for MemberType {
    fn clone(&self) -> MemberType {
        match self {
            MemberType::Base => MemberType::Base,
            MemberType::Mutation(delta) => MemberType::Mutation(delta.clone()),
        }
    }
}

const MAX_DEV: f32 = 10.0;

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    (val - 0.5) * MAX_DEV
}
