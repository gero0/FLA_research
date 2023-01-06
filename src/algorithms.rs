pub mod hillclimb;
pub mod two_opt;

use crate::helpers::{random_solution, tour_len};

use rand::{distributions::Uniform, prelude::Distribution, RngCore, SeedableRng};
use rand_chacha::{self, ChaCha8Rng};
use rustc_hash::{FxHashMap, FxHashSet};

pub type HillclimbFunction = dyn Fn(&Vec<usize>, &Vec<Vec<i32>>) -> (Vec<usize>, i32);

pub struct SnowballSampler<'a> {
    walk_len: u32,
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    rng: ChaCha8Rng,
    distance_matrix: &'a Vec<Vec<i32>>,
    hillclimb: &'a HillclimbFunction,
}

impl<'a> SnowballSampler<'a> {
    pub fn new(
        walk_len: u32,
        n_edges: u32,
        depth: u32,
        mut_d: usize,
        distance_matrix: &'a Vec<Vec<i32>>,
        hillclimb_function: &'a HillclimbFunction,
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
        }
    }

    pub fn sample(
        &mut self,
    ) -> (
        FxHashMap<Vec<usize>, i32>,
        FxHashMap<(Vec<usize>, Vec<usize>), i32>,
    ) {
        let mut solutions = FxHashMap::default();
        let mut edges = FxHashMap::default();
        let mut visited_nodes = FxHashSet::default();

        let start = random_solution(self.distance_matrix.len(), Some(self.rng.next_u64()));
        let (mut c_solution, mut c_len) = (self.hillclimb)(&start, self.distance_matrix);
        solutions.insert(c_solution.clone(), c_len);

        for _ in 0..self.walk_len {
            self.snowball(self.depth, &c_solution, &mut solutions, &mut edges);
            visited_nodes.insert(c_solution.clone());
            (c_solution, c_len) =
                self.random_walk_step(&c_solution, &mut solutions, &edges, &visited_nodes);
        }

        (solutions, edges)
    }

    fn snowball(
        &mut self,
        depth: u32,
        c_solution: &Vec<usize>,
        solutions: &mut FxHashMap<Vec<usize>, i32>,
        edges: &mut FxHashMap<(Vec<usize>, Vec<usize>), i32>,
    ) {
        if depth == 0 {
            return;
        }

        for _ in 0..self.n_edges {
            let random_solution = mutate(c_solution, self.mut_d, &mut self.rng);
            let (solution, len) = (self.hillclimb)(&random_solution, self.distance_matrix);
            solutions.insert(solution.clone(), len);
            match edges.get_mut(&(c_solution.to_owned(), solution.clone())) {
                Some(weight) => {
                    *weight += 1;
                }
                None => {
                    edges.insert((c_solution.clone(), solution.clone()), 1);
                    self.snowball(depth - 1, &solution, solutions, edges)
                }
            };
        }
    }

    fn random_walk_step(
        &mut self,
        c_solution: &Vec<usize>,
        solutions: &mut FxHashMap<Vec<usize>, i32>,
        edges: &FxHashMap<(Vec<usize>, Vec<usize>), i32>,
        visited_nodes: &FxHashSet<Vec<usize>>,
    ) -> (Vec<usize>, i32) {
        let mut neighbors = vec![];
        for edge in edges {
            if edge.0 .0 == *c_solution && !visited_nodes.contains(&edge.0 .1) {
                neighbors.push(edge.0 .1.clone());
            }
        }

        if neighbors.is_empty() {
            let random = random_solution(self.distance_matrix.len(), Some(self.rng.next_u64()));
            let (solution, len) = (self.hillclimb)(&random, self.distance_matrix);
            solutions.insert(solution.clone(), len);
            return (solution, len);
        }

        let between = Uniform::from(0..neighbors.len());
        let a = between.sample(&mut self.rng);
        let len = tour_len(&neighbors[a], self.distance_matrix);

        (neighbors[a].clone(), len)
    }
}

pub fn mutate(perm: &Vec<usize>, n_swaps: usize, rng: &mut ChaCha8Rng) -> Vec<usize> {
    let mut mutation = perm.to_owned();
    let mut i = 0;
    while i < n_swaps {
        let between = Uniform::from(0..perm.len());
        let a = between.sample(rng);
        let b = between.sample(rng);

        if a == b {
            continue;
        }

        mutation.swap(a, b);
        i += 1;
    }

    mutation
}
