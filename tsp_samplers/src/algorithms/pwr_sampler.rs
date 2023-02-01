use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Mutex,
    },
    thread::{self, available_parallelism},
};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use tsptools::helpers::random_solution;

use crate::helpers::mutate;

use super::{EdgeMap, HillclimbFunction, NodeMap};

//TODO: try to parallelize
// should be used with FIRST IMPROVEMENT two_opt
// kicks leading to solutions not present

pub struct PwrSampler {
    distance_matrix: Vec<Vec<i32>>,
    hillclimb_function: HillclimbFunction,
    rng: ChaCha8Rng,
    solutions: NodeMap,
    edges: EdgeMap,
}

impl PwrSampler {
    pub fn new(
        distance_matrix: Vec<Vec<i32>>,
        hillclimb_function: HillclimbFunction,
        seed: Option<u64>,
    ) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self {
            distance_matrix,
            hillclimb_function,
            rng,
            solutions: NodeMap::default(),
            edges: EdgeMap::default(),
        }
    }

    pub fn sample(&mut self, n_max: u32, n_att: u32, e_att: u32) {
        self.sample_nodes(n_max, n_att);
        self.sample_edges(e_att);
    }

    fn sample_nodes(&mut self, n_max: u32, n_att: u32) {
        let distance_matrix = &self.distance_matrix;
        let n = self.distance_matrix.len();
        let last_id = AtomicU16::new(0);
        //sample nodes
        for _ in 0..n_max {
            for _ in 0..n_att {
                let start = random_solution(n as u16, None, true);
                let (solution, s_len) = (self.hillclimb_function)(&start, distance_matrix, true);
                match self.solutions.get(&start) {
                    Some(_) => { /*do nothing if solution is already in the map */ }
                    None => {
                        let id = last_id.fetch_add(1, Ordering::Relaxed);
                        self.solutions.insert(solution, (id, s_len));
                    }
                }
            }
        }
    }

    fn sample_edges(&mut self, e_att: u32) {
        let distance_matrix = &self.distance_matrix;
        for s in &self.solutions {
            for _ in 0..e_att {
                let start = mutate(s.0, 2, &mut self.rng);
                let new_s = (self.hillclimb_function)(&start, distance_matrix, true);
                match self.solutions.get(&new_s.0) {
                    Some(new_s) => match self.edges.get_mut(&((s.1).0, new_s.0)) {
                        Some(edge) => *edge += 1,
                        None => {
                            self.edges.insert(((s.1).0, new_s.0), 1);
                        }
                    },
                    None => {
                        //TODO:
                        //increase the number of kick moves from s leading to solution not
                        //present in NLON by 1;
                    }
                }
            }
        }
    }

    pub fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.solutions, &self.edges)
    }
}
