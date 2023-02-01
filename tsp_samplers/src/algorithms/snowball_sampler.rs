use rand::{distributions::Uniform, prelude::Distribution, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rustc_hash::{FxHashMap, FxHashSet};
use tsptools::helpers::{random_solution, tour_len};

use crate::helpers::mutate;

use super::{EdgeMap, HillclimbFunction, NodeMap};

pub struct SnowballSampler {
    walk_len: u32,
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    rng: ChaCha8Rng,
    distance_matrix: Vec<Vec<i32>>,
    hillclimb: HillclimbFunction,
    last_node_id: u16,
    solutions: NodeMap,
    edges: EdgeMap,
    visited_nodes: FxHashSet<u16>,
    hc_counter: u64,
    current_solution: Option<(Vec<u16>, i32)>,
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
            solutions: FxHashMap::default(),
            edges: FxHashMap::default(),
            visited_nodes: FxHashSet::default(),
            hc_counter: 0,
            current_solution: None,
        }
    }

    pub fn reset(&mut self) {
        self.solutions = FxHashMap::default();
        self.edges = FxHashMap::default();
        self.visited_nodes = FxHashSet::default();
        self.hc_counter = 0;
        self.current_solution = None;
    }

    pub fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.solutions, &self.edges)
    }

    pub fn get_hc_calls(&self) -> u64 {
        self.hc_counter
    }

    pub fn sample(&mut self) {
        if self.current_solution.is_none() {
            let start =
                random_solution(self.distance_matrix.len() as u16, Some(self.rng.next_u64()), true);
            let (c_solution, c_len) = (self.hillclimb)(&start, &self.distance_matrix, true);
            self.hc_counter += 1;
            let id = self.get_next_id();
            self.solutions.insert(c_solution.clone(), (id, c_len));
            self.current_solution = Some((c_solution, c_len));
        }

        for _ in 0..self.walk_len {
            let current_solution = self.current_solution.as_ref().unwrap().clone();
            self.snowball(self.depth, &current_solution.0);
            let id = self
                .solutions
                .get(&current_solution.0)
                .expect("Solution must be present in map at this point")
                .0;
            self.visited_nodes.insert(id);
            let new_sol = self.random_walk_step(&current_solution.0);
            self.current_solution = Some(new_sol);
        }
    }

    fn snowball(&mut self, depth: u32, c_solution: &Vec<u16>) {
        if depth == 0 {
            return;
        }

        for _ in 0..self.n_edges {
            let random_solution = mutate(c_solution, self.mut_d, &mut self.rng);
            let (solution, len) = (self.hillclimb)(&random_solution, &self.distance_matrix, true);
            self.hc_counter += 1;
            let solution_id = match self.solutions.get(&solution) {
                Some(s) => {
                    //solution already exists (unlikely but possible)
                    s.0
                }
                None => {
                    let id = self.get_next_id();
                    self.solutions.insert(solution.clone(), (id, len));
                    id
                }
            };
            let c_solution_id = self
                .solutions
                .get(c_solution)
                .expect("Current solution must already be in the map")
                .0;
            match self.edges.get_mut(&(c_solution_id, solution_id)) {
                Some(weight) => {
                    *weight += 1;
                }
                None => {
                    self.edges.insert((c_solution_id, solution_id), 1);
                    self.snowball(depth - 1, &solution)
                }
            };
        }
    }

    fn random_walk_step(&mut self, c_solution: &Vec<u16>) -> (Vec<u16>, i32) {
        let mut neighbors = vec![];
        let c_solution_id = self
            .solutions
            .get(c_solution)
            .expect("Solution must be present in map at this point")
            .0;
        for edge in self.edges.iter() {
            if edge.0 .0 == c_solution_id && !self.visited_nodes.contains(&edge.0 .1) {
                neighbors.push(edge.0 .1.clone());
            }
        }

        if neighbors.is_empty() {
            let random =
                random_solution(self.distance_matrix.len() as u16, Some(self.rng.next_u64()), true);
            let (solution, len) = (self.hillclimb)(&random, &self.distance_matrix, true);
            self.hc_counter += 1;
            match self.solutions.get(&solution) {
                Some(_) => { /*do nothing if solution is already in the map */ }
                None => {
                    let id = self.get_next_id();
                    self.solutions.insert(solution.clone(), (id, len));
                }
            }
            return (solution, len);
        }

        let between = Uniform::from(0..neighbors.len());
        let a = between.sample(&mut self.rng);
        let neighbor = self
            .solutions
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
