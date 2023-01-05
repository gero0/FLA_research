pub mod hillclimb;
pub mod two_opt;

use crate::helpers::tour_len;

use self::hillclimb::{hillclimb, hillclimb_rand};

use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, SeedableRng};
use std::collections::{HashMap, HashSet};

pub fn snowball_sampling(
    walk_len: u32,
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    distance_matrix: &Vec<Vec<i32>>,
) -> (
    HashMap<Vec<usize>, i32>,
    HashMap<(Vec<usize>, Vec<usize>), i32>,
) {
    let mut solutions = HashMap::new();
    let mut edges = HashMap::new();

    let mut visited_nodes = HashSet::new();

    let (mut c_solution, mut c_len) = hillclimb_rand(distance_matrix, None);
    solutions.insert(c_solution.clone(), c_len);

    for _ in 0..walk_len {
        snowball(
            n_edges,
            depth,
            mut_d,
            &c_solution,
            distance_matrix,
            &mut solutions,
            &mut edges,
        );
        visited_nodes.insert(c_solution.clone());
        (c_solution, c_len) =
            random_walk_step(&c_solution, distance_matrix, &edges, &visited_nodes);
    }

    (solutions, edges)
}

fn snowball(
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    c_solution: &Vec<usize>,
    distance_matrix: &Vec<Vec<i32>>,
    solutions: &mut HashMap<Vec<usize>, i32>,
    edges: &mut HashMap<(Vec<usize>, Vec<usize>), i32>,
) {
    if depth <= 0 {
        return;
    }

    for _ in 0..n_edges {
        let random_solution = mutate(c_solution, mut_d);
        let (solution, len) = hillclimb(&random_solution, distance_matrix);
        solutions.insert(solution.clone(), len);
        match edges.get_mut(&(c_solution.to_owned(), solution.clone())) {
            Some(weight) => {
                *weight += 1;
            }
            None => {
                edges.insert((c_solution.clone(), solution.clone()), 1);
                snowball(
                    n_edges,
                    depth - 1,
                    mut_d,
                    &solution,
                    distance_matrix,
                    solutions,
                    edges,
                )
            }
        };
    }
}

fn random_walk_step(
    c_solution: &Vec<usize>,
    distance_matrix: &Vec<Vec<i32>>,
    edges: &HashMap<(Vec<usize>, Vec<usize>), i32>,
    visited_nodes: &HashSet<Vec<usize>>,
) -> (Vec<usize>, i32) {
    let mut neighbors = vec![];
    for edge in edges {
        if edge.0 .0 == *c_solution && !visited_nodes.contains(&edge.0 .1) {
            neighbors.push(edge.0 .1.clone());
        }
    }

    if neighbors.is_empty() {
        return hillclimb_rand(distance_matrix, None);
    }

    let mut rng = SmallRng::from_entropy();
    let between = Uniform::from(0..neighbors.len());
    let a = between.sample(&mut rng);
    let len = tour_len(&neighbors[a], distance_matrix);

    (neighbors[a].clone(), len)
}

pub fn mutate(perm: &Vec<usize>, n_swaps: usize) -> Vec<usize> {
    let mut mutation = perm.to_owned();
    let mut rng = SmallRng::from_entropy();
    let mut i = 0;
    while i < n_swaps {
        let between = Uniform::from(0..perm.len());
        let a = between.sample(&mut rng);
        let b = between.sample(&mut rng);

        if a == b {
            continue;
        }

        mutation.swap(a, b);
        i += 1;
    }

    mutation
}
