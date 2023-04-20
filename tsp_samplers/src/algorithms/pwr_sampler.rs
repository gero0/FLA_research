use std::{
    cmp::min,
    sync::{
        atomic::{AtomicU16, AtomicU64, AtomicUsize, Ordering},
        Mutex, MutexGuard,
    },
    thread::{self, available_parallelism},
};

// use itertools::Itertools;
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha8Rng;

use crate::helpers::{mutate_2exchange, random_solution};

use super::{two_opt_besti, two_opt_firsti, EdgeMap, NodeMap, SamplingAlg};

pub struct PwrSampler {
    distance_matrix: Vec<Vec<i32>>,
    rng: Mutex<ChaCha8Rng>,
    nodes: Mutex<NodeMap>,
    edges: Mutex<EdgeMap>,
    hc_counter: AtomicU64,
    oracle_counter: AtomicU64,
    next_id: AtomicU16,
    mut_d: usize,
    pub missed: AtomicUsize,
}

impl PwrSampler {
    pub fn new(distance_matrix: Vec<Vec<i32>>, mut_d: usize, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self {
            distance_matrix,
            rng: Mutex::new(rng),
            nodes: Mutex::new(NodeMap::default()),
            edges: Mutex::new(EdgeMap::default()),
            hc_counter: AtomicU64::new(0),
            oracle_counter: AtomicU64::new(0),
            next_id: AtomicU16::new(0),
            mut_d,
            missed: AtomicUsize::new(0),
        }
    }

    pub fn sample(&mut self, n_max: u32, n_att: u32, e_att: u32) {
        self.sample_nodes(n_max, n_att);
        self.sample_edges(e_att);
    }

    fn sample_nodes(&mut self, n_max: u32, n_att: u32) {
        let thread_count: usize = available_parallelism().unwrap().get();
        let distance_matrix = &self.distance_matrix;
        let n = self.distance_matrix.len();

        let n_per_thread = (n_max as f64 / thread_count as f64).ceil() as usize;
        let mut n_scheduled = 0;

        //boilerplate to appease the borrow checker
        let nodes = &self.nodes;
        let hc_counter = &self.hc_counter;
        let oracle_counter = &self.oracle_counter;
        let next_id = &self.next_id;
        let rng = &self.rng;

        thread::scope(|s| {
            for _ in 0..thread_count {
                let to_schedule = min(n_max - n_scheduled, n_per_thread as u32);
                let n_limit = to_schedule;
                n_scheduled += to_schedule;

                s.spawn(move || {
                    for _ in 0..n_limit {
                        for _ in 0..n_att {
                            let mut rnglock = rng.lock().unwrap();
                            let start = random_solution(n as u16, Some(rnglock.next_u64()), true);
                            drop(rnglock);
                            let (solution, s_len, oracle) = two_opt_besti(&start, distance_matrix);
                            hc_counter.fetch_add(1, Ordering::Relaxed);
                            oracle_counter.fetch_add(oracle as u64, Ordering::Relaxed);

                            let mut nodes = nodes.lock().unwrap();

                            if nodes.get(&solution).is_none() {
                                nodes.insert(
                                    solution,
                                    (next_id.fetch_add(1, Ordering::Relaxed), s_len),
                                );
                                break;
                            }
                        }
                    }
                });
            }
        });
    }

    fn sample_edges(&mut self, e_att: u32) {
        let distance_matrix = &self.distance_matrix;
        let thread_count: usize = available_parallelism().unwrap().get();

        let node_lock = self.nodes.lock().unwrap();
        //Copy permutations to vector because iterating over hashmap is a pain
        let node_perms: Vec<_> = node_lock
            .iter()
            .map(|n| (n.0.clone(), n.1.clone()))
            .collect();
        drop(node_lock);
        let n_per_thread = (node_perms.len() as f64 / thread_count as f64).ceil() as usize;
        let chunks = node_perms.chunks(n_per_thread);

        //boilerplate to appease the borrow checker
        let nodes = &self.nodes;
        let edges = &self.edges;
        let rng = &self.rng;
        let missed = &self.missed;
        let hc_counter = &self.hc_counter;
        let oracle_counter = &self.oracle_counter;
        let mut_d = self.mut_d;

        thread::scope(|s| {
            for chunk in chunks {
                s.spawn(move || {
                    for n in chunk {
                        for _ in 0..e_att {
                            let mut rng = rng.lock().unwrap();
                            let start = mutate_2exchange(&n.0, mut_d, &mut rng);
                            drop(rng);
                            let new_s = two_opt_firsti(&start, distance_matrix);
                            hc_counter.fetch_add(1, Ordering::Relaxed);
                            oracle_counter.fetch_add(new_s.2 as u64, Ordering::Relaxed);

                            let mut edges = edges.lock().unwrap();
                            let nodes = nodes.lock().unwrap();

                            match nodes.get(&new_s.0) {
                                Some(new_n) => match edges.get_mut(&((n.1).0, new_n.0)) {
                                    Some(edge) => *edge += 1,
                                    None => {
                                        edges.insert(((n.1).0, new_n.0), 1);
                                    }
                                },
                                None => {
                                    missed.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                });
            }
        });
    }

    pub fn missed(&self) -> usize {
        return self.missed.load(Ordering::Relaxed);
    }

    pub fn get_samples(&self) -> (MutexGuard<NodeMap>, MutexGuard<EdgeMap>) {
        let nodes = self
            .nodes
            .lock()
            .expect("ERROR: Mutex poisoned, bailing out!");
        let edges = self
            .edges
            .lock()
            .expect("ERROR: Mutex poisoned, bailing out!");
        (nodes, edges)
    }
}

impl SamplingAlg for PwrSampler {
    fn get_hc_calls(&self) -> u64 {
        self.hc_counter.load(Ordering::Relaxed)
    }

    fn get_oracle_calls(&self) -> u128 {
        self.oracle_counter.load(Ordering::Relaxed) as u128
    }
}
