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
    fitness: f32,
    is_beneficial: bool,
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
            is_beneficial: false,
        }
    }
    pub fn mutation(&self, index: usize) -> &Member {
        if index == 0 {
            &self
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            &self.mutations[index - 1]
        }
    }
    pub fn point(&self, index: usize) -> &Point {
        &self.mutation(index).point
    }
    pub fn fitness(&self) -> f32 {
        self.fitness
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
    pub fn merge_mutations_into_base(&mut self) -> () {
        let mut aggregate = Point::from(self.point.values());
        for mutation in &self.mutations {
            match mutation.source {
                MemberType::Mutation(delta) => {
                    if mutation.is_beneficial {
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
    fn mark_as_beneficial(&mut self) -> () {
        self.is_beneficial = true;
        // self.point.mark_as_beneficial();
    }
    pub fn mark_beneficial_mutations(&mut self, index: usize) -> () {
        match self.source {
            // only base members have mutations
            MemberType::Base => {
                // base members are not mutations
                if index == 0 {
                    return;
                }
                if index > self.size {
                    panic!("size out of bounds");
                }
                if self.fitness < self.mutations[index - 1].fitness {
                // if self.point(0).fitness() < self.point(index).fitness() {
                    // self.mutations[index - 1].mark_as_beneficial();
                    self.mutations[index - 1].is_beneficial = true;
                }
            }
            _ => (),
        }
    }
    pub fn add_fitness(&mut self, index: usize, fitness: f32) -> () {
        if index == 0 {
            self.fitness += fitness;
            //self._add_fitness(fitness);
        } else {
            if index > self.size {
                panic!("size out of bounds");
            }
            self.mutations[index - 1].fitness += fitness;
            // self.mutations[index - 1]._add_fitness(fitness);
        }
    }
    fn _add_fitness(&mut self, fitness: f32) -> () {
        self.fitness += fitness;
        // self.point.add_fitness(fitness);
    }
    pub fn get_best(&mut self) -> (f32, f32) {
        let mut index = 0;
        let mut highest = self.fitness;
        // let mut highest = self.point(0).fitness();
        for (i, mutation) in (&self.mutations).into_iter().enumerate() {
            if mutation.fitness > highest {
            //if mutation.point(0).fitness() > highest {
                index = i;
                highest = mutation.fitness;
                // highest = mutation.point(0).fitness();
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

const MAX_DEV: f32 = 10.0;

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    (val - 0.5) * MAX_DEV
}