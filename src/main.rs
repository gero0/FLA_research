pub mod parsers;
pub mod helpers;
pub mod algorithms;

use helpers::path_len;
use parsers::*;

use crate::algorithms::two_opt::*;

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();
    let opt_tour = parse_tour_file("./data/berlin52.opt.tour", &file.nodes).unwrap();
    let opt_tour_len = path_len(&opt_tour);

    let two_opt_tour = two_opt_random(&file.nodes, Some(2117));
    let two_opt_len = path_len(&two_opt_tour);
    // println!("{:?}", file);
    // println!("{:?}", opt_tour);
    // println!("Optimal: {}", path_len(&opt_tour));
    println!("2opt: {}", two_opt_len);
    println!("Diff: {}", (two_opt_len - opt_tour_len) as f32 * 100.0 / opt_tour_len as f32)
}
