use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::helpers::{random_solution, mutate_2exchange};

use super::{two_opt_besti, two_opt_firsti, EdgeMap, NodeMap, SamplingAlg};

pub struct PwrSampler {
    distance_matrix: Vec<Vec<i32>>,
    rng: ChaCha8Rng,
    nodes: NodeMap,
    edges: EdgeMap,
    hc_counter: u64,
    oracle_counter: u128,
    next_id: u16,
    mut_d: usize,
}

impl PwrSampler {
    pub fn new(distance_matrix: Vec<Vec<i32>>, mut_d: usize, seed: Option<u64>) -> Self {
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
            oracle_counter: 0,
            next_id: 0,
            mut_d,
        }
    }

    pub fn sample(&mut self, n_max: u32, n_att: u32, e_att: u32) {
        self.sample_nodes(n_max, n_att);
        self.sample_edges(e_att);
    }

    fn sample_nodes(&mut self, n_max: u32, n_att: u32) {
        let distance_matrix = &self.distance_matrix;
        let n = self.distance_matrix.len();
        //sample nodes
        for _ in 0..n_max {
            for _ in 0..n_att {
                let start = random_solution(n as u16, None, true);
                let (solution, s_len, oracle) = two_opt_besti(&start, distance_matrix);
                self.hc_counter += 1;
                self.oracle_counter += oracle;
                if self.nodes.get(&solution).is_none() {
                    self.nodes.insert(solution, (self.next_id, s_len));
                    self.next_id += 1;
                    break;
                }
            }
        }
    }

    fn sample_edges(&mut self, e_att: u32) {
        let distance_matrix = &self.distance_matrix;
        for s in &self.nodes {
            for _ in 0..e_att {
                let start = mutate_2exchange(s.0, self.mut_d, &mut self.rng);
                let new_s = two_opt_firsti(&start, distance_matrix);
                self.hc_counter += 1;
                self.oracle_counter += new_s.2;
                match self.nodes.get(&new_s.0) {
                    Some(new_s) => match self.edges.get_mut(&((s.1).0, new_s.0)) {
                        Some(edge) => *edge += 1,
                        None => {
                            self.edges.insert(((s.1).0, new_s.0), 1);
                        }
                    },
                    None => {}
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

    fn get_oracle_calls(&self) -> u128 {
        self.oracle_counter
    }
}
