pub mod algorithms;
pub mod helpers;
pub mod parsers;

use helpers::*;
use parsers::*;

use crate::algorithms::hillclimb::*;
use crate::algorithms::two_opt::*;

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();
    let opt_tour = parse_tour_file("./data/berlin52.opt.tour", &file.nodes).unwrap();
    let opt_tour_len = path_len(&opt_tour);

    let two_opt_tour = two_opt_random(&file.nodes, Some(2117));
    let two_opt_len = path_len(&two_opt_tour);

    // println!("Opt tour len: {}", opt_tour_len);

    // println!("2opt: {}", two_opt_len);
    // println!(
    //     "Diff: {}",
    //     (two_opt_len - opt_tour_len) as f32 * 100.0 / opt_tour_len as f32
    // );

    let mut min_len = 2000000;

    for i in 0..10000{
        let hillclimb_tour = hillclimb(&file.nodes, None);
        let hillclimb_len = path_len(&hillclimb_tour);

        println!("Hillclimb: {}", hillclimb_len);
        println!(
            "Diff: {}",
            (hillclimb_len - opt_tour_len) as f32 * 100.0 / opt_tour_len as f32
        );

        if hillclimb_len < min_len {
            min_len = hillclimb_len
        }
    }

    println!("min len: {}", min_len);
    println!(
        "Diff: {}",
        (min_len - opt_tour_len) as f32 * 100.0 / opt_tour_len as f32
    );
}
