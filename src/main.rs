pub mod algorithms;
pub mod helpers;
pub mod parsers;

use crate::algorithms::{SnowballSampler, hillclimb, two_opt::two_opt};
use helpers::*;
use parsers::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, available_parallelism},
};

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();
    // let file = parse_tsp_file("./data/ulysses16.tsp").unwrap();

    // const SAMPLE_COUNT: usize = 1000;
    // let thread_count: usize = available_parallelism().unwrap().get();
    // println!("{} threads available", thread_count);
    // let samples_per_thread = SAMPLE_COUNT / thread_count;

    let distance_matrix = Arc::new(generate_distance_matrix(&file.nodes));
    // let local_minimums = Arc::new(Mutex::new(HashMap::new()));
    // let mut handles = vec![];

    // for _ in 0..thread_count {
    //     let local_minimums = Arc::clone(&local_minimums);
    //     let distance_matrix = Arc::clone(&distance_matrix);
    //     let handle = thread::spawn(move || {
    //         for _ in 0..samples_per_thread {
    //             let (hillclimb_tour, hillclimb_len) = hillclimb_rand(&distance_matrix, None);
    //             local_minimums
    //                 .lock()
    //                 .expect("Mutex poisoned, bailing out!")
    //                 .insert(hillclimb_tour, hillclimb_len);
    //         }
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("{}", local_minimums.lock().unwrap().len());
    let mut snowball_sampler = SnowballSampler::new(1, 5, 3, 2, &distance_matrix, &two_opt, Some(2000));
    let (nodes, edges) = snowball_sampler.sample();

    println!("{:?}", nodes);
    // println!("{:?}", edges.len());
}
