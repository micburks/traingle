use super::point::Point;

use rand::prelude::*;
use rand_distr::StandardNormal;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Member {
    pub id: usize,
    pub source: MemberType,
    pub point: Box<Point>,
    mutations: Vec<Rc<RefCell<Member>>>,
    size: usize,
    dimensions: (f32, f32),
    n_points: f32,
    pub fitness: f32,
}

impl Member {
    pub fn new(
        id: usize,
        source: MemberType,
        point: (f32, f32),
        dimensions: (f32, f32),
        n_points: f32,
    ) -> Member {
        Member {
            id,
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
            n_points,
            fitness: 0.0,
        }
    }
    fn clone(&self) -> Member {
        Member {
            id: self.id,
            source: self.source.clone(),
            point: Box::new(*self.point.clone()),
            mutations: vec![],
            size: 0,
            dimensions: self.dimensions,
            n_points: self.n_points,
            fitness: 0.0,
        }
    }
    pub fn mutate(&mut self) -> Rc<RefCell<Member>> {
        self.size += 1;
        if should_mutate(3.0 / 5.0) {
            let random_point = Point::new(random(), random());
            let mutation = Rc::new(RefCell::new(Member::new(
                self.id,
                MemberType::Mutation(random_point),
                self.point.values(),
                self.dimensions,
                self.n_points,
            )));
            self.mutations.push(Rc::clone(&mutation));
            mutation
        } else {
            Rc::new(RefCell::new(self.clone()))
        }
    }
    pub fn merge_mutations_into_base(&mut self) -> Rc<RefCell<Member>> {
        let mut aggregate = Point::from(self.point.values());
        let mut beneficial_mutations = vec![];
        let mut sum = 0.0;
        let base_fitness = self.fitness;
        for m in &self.mutations {
            let m = m.borrow();
            if let MemberType::Mutation(delta) = m.source {
                if base_fitness < m.fitness {
                    beneficial_mutations.push((delta, m.fitness));
                    sum += m.fitness;
                }
            }
        }
        for (delta, fitness) in beneficial_mutations {
            let percent = fitness / sum;
            aggregate.mutate(delta * percent, self.dimensions);
        }
        let mutation = Rc::new(RefCell::new(Member::new(
            self.id,
            MemberType::Base,
            aggregate.values(),
            self.dimensions,
            self.n_points,
        )));
        self.mutations.push(Rc::clone(&mutation));
        self.size += 1;
        mutation
    }
    pub fn add_fitness(&mut self, fitness: f32) -> () {
        self.fitness += fitness;
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

fn should_mutate(rate: f32) -> bool {
    thread_rng().gen_bool(rate as f64)
}

fn random() -> f32 {
    let val: f32 = thread_rng().sample(StandardNormal);
    // val * 5.0 + 1.0
    (val - 0.5) * MAX_DEV
}
