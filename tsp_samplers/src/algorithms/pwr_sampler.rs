use std::sync::atomic::{AtomicU16, Ordering};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::helpers::{mutate, random_solution};

use super::{two_opt_besti, two_opt_firsti, EdgeMap, NodeMap, SamplingAlg};

//TODO: try to parallelize
// should be used with FIRST IMPROVEMENT two_opt
// kicks leading to solutions not present

pub struct PwrSampler {
    distance_matrix: Vec<Vec<i32>>,
    rng: ChaCha8Rng,
    nodes: NodeMap,
    edges: EdgeMap,
    hc_counter: u64,
}

impl PwrSampler {
    pub fn new(distance_matrix: Vec<Vec<i32>>, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self {
            distance_matrix,
            rng,
            nodes: NodeMap::default(),
            edges: EdgeMap::default(),
            hc_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.nodes = NodeMap::default();
        self.edges = EdgeMap::default();
        self.hc_counter = 0;
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
                let (solution, s_len) = two_opt_besti(&start, distance_matrix);
                self.hc_counter += 1;
                match self.nodes.get(&start) {
                    Some(_) => { /*do nothing if solution is already in the map */ }
                    None => {
                        let id = last_id.fetch_add(1, Ordering::Relaxed);
                        self.nodes.insert(solution, (id, s_len));
                        break;
                    }
                }
            }
        }
    }

    fn sample_edges(&mut self, e_att: u32) {
        let distance_matrix = &self.distance_matrix;
        for s in &self.nodes {
            for _ in 0..e_att {
                let start = mutate(s.0, 2, &mut self.rng);
                let new_s = two_opt_firsti(&start, distance_matrix);
                self.hc_counter += 1;
                match self.nodes.get(&new_s.0) {
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
}

impl SamplingAlg for PwrSampler {
    fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.nodes, &self.edges)
    }

    fn get_hc_calls(&self) -> u64 {
        self.hc_counter
    }
}
