use super::point::Point;

use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug)]
pub struct Member {
    pub source: MemberType,
    point: Box<Point>,
    mutations: Vec<Option<Member>>,
    size: usize,
    dimensions: (f32, f32),
    pub fitness: f32,
}

impl Member {
    pub fn new(source: MemberType, point: (f32, f32), dimensions: (f32, f32)) -> Member {
        Member {
            source: source.clone(),
            point: match source {
                MemberType::Base => Box::new(Point::from(point)),
                MemberType::Mutation(delta) => {
                    Box::new(Point::from(point).mutate(delta, dimensions))
                }
            },
            mutations: vec![],
            size: 0,
            dimensions,
            fitness: 0.0,
        }
    }
    pub fn mutation(&self, index: usize) -> &Option<Member> {
        if index == 0 {
            return &None;
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            return &self.mutations[index - 1];
        }
    }
    pub fn point(&self, index: usize) -> &Point {
        match self.mutation(index) {
            Some(m) => &m.point,
            None => &self.point,
        }
    }
    pub fn mutate(&mut self) -> () {
        // will have to mess with normal distribution here
        if should_mutate(1.0 / 5.0) {
            let random_point = Point::new(random(), random());
            self.mutations.push(Option::Some(Member::new(
                MemberType::Mutation(random_point),
                self.point.values(),
                self.dimensions,
            )));
        } else {
            self.mutations.push(Option::None);
        }
        self.size += 1;
    }
    pub fn merge_mutations_into_base(&mut self) -> () {
        let mut aggregate = Point::from(self.point.values());
        let mut beneficial_mutations = vec![];
        let mut sum = 0.0;
        let base_fitness = self.fitness;
        for mutation in &self.mutations {
            if let Some(m) = mutation {
                if let MemberType::Mutation(delta) = m.source {
                    if base_fitness < m.fitness {
                        beneficial_mutations.push((delta, m.fitness));
                        sum += m.fitness;
                    }
                }
            }
        }
        for (delta, fitness) in beneficial_mutations {
            let percent = fitness / sum;
            aggregate.mutate(delta * percent, self.dimensions);
        }
        self.mutations.push(Some(Member::new(
            MemberType::Base,
            aggregate.values(),
            self.dimensions,
        )));
        self.size += 1;
    }
    pub fn add_fitness(&mut self, index: usize, fitness: f32) -> () {
        if index == 0 {
            self.fitness += fitness;
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            if let Some(m) = &mut self.mutations[index - 1] {
                m.fitness += fitness;
            }
        }
    }
    fn _add_fitness(&mut self, fitness: f32) -> () {
        self.fitness += fitness;
    }
    pub fn get_best(&self) -> (f32, f32) {
        let mut index = 0;
        let mut highest = self.fitness;
        for (i, mutation) in (&self.mutations).into_iter().enumerate() {
            if let Some(m) = mutation {
                if m.fitness > highest {
                    index = i;
                    highest = m.fitness;
                }
            }
        }
        self.values(index)
    }
    pub fn values(&self, index: usize) -> (f32, f32) {
        self.point(index).values()
    }
}

#[derive(Debug)]
pub enum MemberType {
    Base,
    Mutation(Point),
}

impl Clone for MemberType {
    fn clone(&self) -> MemberType {
        match self {
            MemberType::Base => MemberType::Base,
            MemberType::Mutation(delta) => MemberType::Mutation(Point::from(delta.values())),
        }
    }
}

const MAX_DEV: f32 = 3.0;

fn should_mutate(rate: f32) -> bool {
    thread_rng().gen_bool(rate as f64)
}

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    val * 10.0
    // (val - 0.5) * MAX_DEV
}
