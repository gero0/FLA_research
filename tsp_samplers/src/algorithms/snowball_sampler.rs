use crate::helpers::{mutate, random_solution, tour_len};
use rand::{distributions::Uniform, prelude::Distribution, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rustc_hash::{FxHashMap, FxHashSet};

use super::{EdgeMap, HillclimbFunction, NodeMap, SamplingAlg};

pub struct SnowballSampler {
    walk_len: u32,
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    rng: ChaCha8Rng,
    distance_matrix: Vec<Vec<i32>>,
    hillclimb: HillclimbFunction,
    last_node_id: u16,
    nodes: NodeMap,
    edges: EdgeMap,
    walk_visited: FxHashSet<u16>,
    hc_counter: u64,
    current_lo: Option<(Vec<u16>, i32)>,
}

impl SnowballSampler {
    pub fn new(
        walk_len: u32,
        n_edges: u32,
        depth: u32,
        mut_d: usize,
        distance_matrix: Vec<Vec<i32>>,
        hillclimb_function: HillclimbFunction,
        seed: Option<u64>,
    ) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        Self {
            walk_len,
            n_edges,
            depth,
            mut_d,
            rng,
            distance_matrix,
            hillclimb: hillclimb_function,
            last_node_id: 0,
            nodes: FxHashMap::default(),
            edges: FxHashMap::default(),
            walk_visited: FxHashSet::default(),
            hc_counter: 0,
            current_lo: None,
        }
    }

    fn climb(&mut self, start: &Vec<u16>) -> (Vec<u16>, i32) {
        self.hc_counter += 1;
        return (self.hillclimb)(start, &self.distance_matrix);
    }

    fn insert_node(&mut self, node: &Vec<u16>, len: i32) -> u16 {
        let id = self.get_next_id();
        self.nodes.insert(node.clone(), (id, len));
        return id;
    }

    pub fn sample(&mut self) {
        if self.current_lo.is_none() {
            let start = random_solution(
                self.distance_matrix.len() as u16,
                Some(self.rng.next_u64()),
                true,
            );
            let (current_lo, lo_len) = self.climb(&start);
            self.insert_node(&current_lo, lo_len);
            self.current_lo = Some((current_lo, lo_len));
        }

        for _ in 0..self.walk_len {
            let current_lo = self.current_lo.as_ref().unwrap().clone();
            self.snowball(self.depth, &current_lo.0);
            let id = self
                .nodes
                .get(&current_lo.0)
                .expect("Solution must be present in map at this point")
                .0;
            self.walk_visited.insert(id);
            let new_lo = self.random_walk_step(&current_lo.0);
            self.current_lo = Some(new_lo);
        }
    }

    fn snowball(&mut self, depth: u32, current_lo: &Vec<u16>) {
        if depth == 0 {
            return;
        }

        for _ in 0..self.n_edges {
            let shuffled = mutate(current_lo, self.mut_d, &mut self.rng);
            let (new_lo, new_lo_len) = self.climb(&shuffled);
            let new_lo_id = match self.nodes.get(&new_lo) {
                Some(s) => s.0,
                None => self.insert_node(&new_lo, new_lo_len),
            };
            let current_lo_id = self
                .nodes
                .get(current_lo)
                .expect("Current solution must already be in the map")
                .0;
            match self.edges.get_mut(&(current_lo_id, new_lo_id)) {
                Some(weight) => {
                    *weight += 1;
                }
                None => {
                    self.edges.insert((current_lo_id, new_lo_id), 1);
                    self.snowball(depth - 1, &new_lo)
                }
            };
        }
    }

    fn random_walk_step(&mut self, c_lo: &Vec<u16>) -> (Vec<u16>, i32) {
        let mut neighbors = vec![];
        let c_solution_id = self
            .nodes
            .get(c_lo)
            .expect("Solution must be present in map at this point")
            .0;
        for edge in self.edges.iter() {
            if edge.0 .0 == c_solution_id && !self.walk_visited.contains(&edge.0 .1) {
                neighbors.push(edge.0 .1.clone());
            }
        }

        if neighbors.is_empty() {
            let random = random_solution(
                self.distance_matrix.len() as u16,
                Some(self.rng.next_u64()),
                true,
            );
            let (lo, len) = self.climb(&random);
            if self.nodes.get(&lo).is_none() {
                self.insert_node(&lo, len);
            }
            return (lo, len);
        }

        let between = Uniform::from(0..neighbors.len());
        let a = between.sample(&mut self.rng);
        let neighbor = self
            .nodes
            .iter()
            .find(|(_k, v)| v.0 == neighbors[a])
            .expect("Solution must be present in map at this point");
        let len = tour_len(neighbor.0, &self.distance_matrix);

        (neighbor.0.clone(), len)
    }

    fn get_next_id(&mut self) -> u16 {
        self.last_node_id += 1;
        self.last_node_id - 1
    }
}

impl SamplingAlg for SnowballSampler {
    fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.nodes, &self.edges)
    }

    fn get_hc_calls(&self) -> u64 {
        self.hc_counter
    }
}
