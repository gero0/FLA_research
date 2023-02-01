use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Mutex,
    },
    thread::{self, available_parallelism},
};

use tsptools::algorithms::hillclimb::hillclimb;

use crate::helpers::generate_perms;

use super::{EdgeMap, HillclimbFunction, NodeMap};

pub struct ExhaustiveSampler {
    distance_matrix: Vec<Vec<i32>>,
    permutations: Vec<Vec<usize>>,
    hillclimb_function: HillclimbFunction,
    mut_d: usize,
    solutions: NodeMap,
    edges: EdgeMap,
}

impl ExhaustiveSampler {
    pub fn new(
        mut_d: usize,
        distance_matrix: Vec<Vec<i32>>,
        hillclimb_function: HillclimbFunction,
    ) -> Self {
        let set: Vec<_> = (0..distance_matrix.len()).collect();
        let permutations = generate_perms(&set, true);

        Self {
            mut_d,
            distance_matrix,
            hillclimb_function,
            permutations,
            solutions: NodeMap::default(),
            edges: EdgeMap::default(),
        }
    }

    pub fn sample(&mut self) {
        let distance_matrix = self.distance_matrix.clone();
        let solutions = Mutex::new(NodeMap::default());
        let edges = Mutex::new(EdgeMap::default());

        // let thread_count: usize = available_parallelism().unwrap().get();
        let thread_count = 1;

        let last_id = AtomicU32::new(0);

        thread::scope(|s| {
            for chunk in self.permutations.chunks(thread_count) {
                for permutation in chunk {
                    s.spawn(|| {
                        let (solution, s_len) =
                            (self.hillclimb_function)(permutation, &distance_matrix, true);
                        let id = last_id.fetch_add(1, Ordering::Relaxed);
                        solutions
                            .lock()
                            .expect("ExhaustiveSampler: Mutex poisoned, bailing out!")
                            .insert(solution, (id, s_len));
                    });
                }
            }
        });
    }
    pub fn get_samples(&self) -> (NodeMap, EdgeMap)
    {
        (self.solutions.clone(), self.edges.clone())
    }
}

