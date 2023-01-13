pub mod algorithms;
pub mod helpers;
pub mod parsers;

use crate::algorithms::{
    hillclimb::hillclimb, two_opt::two_opt, EdgeMap, NodeMap, SnowballSampler,
};
use parsers::*;

use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread::{self, available_parallelism},
};

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();
    // let file = parse_tsp_file("./data/ulysses16.tsp").unwrap();
    // let file = parse_tsp_file("./data/gr21.tsp").unwrap();

    let mut snowball_sampler =
        SnowballSampler::new(5, 5, 3, 2, &file.distance_matrix, &two_opt, Some(2000));
    let (nodes, edges) = snowball_sampler.sample();

    let mut node_file = File::create("nodes.txt").expect("I assumed the OS will cooperate...");
    let mut edge_file = File::create("edges.txt").expect("I assumed the OS will cooperate...");

    for node in nodes {
        let (perm, (id, h)) = node;
        node_file
            .write_fmt(format_args!("{};{:?};{}\n", id, perm, h))
            .unwrap();
    }

    for edge in edges {
        let ((a, b), w) = edge;
        edge_file
            .write_fmt(format_args!("{};{};{}\n", a, b, w))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampling_test() {
        let file = parse_tsp_file("./data/bays29.tsp").unwrap();

        let mut snowball_sampler =
            SnowballSampler::new(1, 5, 3, 2, &file.distance_matrix, &two_opt, Some(2000));
        let (nodes, edges) = snowball_sampler.sample();

        assert_eq!(nodes.len(), 4);
        assert_eq!(edges.len(), 6);
    }
}
