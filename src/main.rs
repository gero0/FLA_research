pub mod algorithms;
pub mod helpers;
pub mod parsers;

use crate::algorithms::{hillclimb::hillclimb, two_opt::two_opt, SnowballSampler};
use parsers::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, available_parallelism},
};

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();
    // let file = parse_tsp_file("./data/ulysses16.tsp").unwrap();
    // let file = parse_tsp_file("./data/bays29.tsp").unwrap();

    println!("{:?}", file.distance_matrix.iter().max());

    let mut snowball_sampler = SnowballSampler::new(
        5,
        5,
        3,
        2,
        &file.distance_matrix,
        &two_opt,
        Some(2000),
    );
    let (nodes, edges) = snowball_sampler.sample();

    println!("{:?}", nodes.len());
    println!("{:?}", edges.len());
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn sampling_test() {
        let file = parse_tsp_file("./data/bays29.tsp").unwrap();

    let mut snowball_sampler = SnowballSampler::new(
        1,
        5,
        3,
        2,
        &file.distance_matrix,
        &two_opt,
        Some(2000),
    );
    let (nodes, edges) = snowball_sampler.sample();

    assert_eq!(nodes.len(), 4);
    assert_eq!(edges.len(), 6);
    }
}