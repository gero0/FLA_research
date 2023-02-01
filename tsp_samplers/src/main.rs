use tsp_samplers::algorithms::{snowball_sampler::SnowballSampler, exhaustive_sampler::{self, ExhaustiveSampler}, hillclimb::hillclimb_steepest};
use tsptools::{
    parsers::parse_tsp_file,
};

fn main() {
    // let file = parse_tsp_file("./data/berlin52.tsp").unwrap();

    // let mut snowball_sampler =
    //     SnowballSampler::new(20, 15, 4, 3, file.distance_matrix, hillclimb, Some(2000));
    // snowball_sampler.sample();
    // let (nodes, edges) = snowball_sampler.get_samples();

    // println!("{}, {}", nodes.len(), edges.len());

    let file = parse_tsp_file("./data/burma14.tsp").unwrap();
    let mut exhaustive_sampler = ExhaustiveSampler::new(2, file.distance_matrix, hillclimb_steepest);
    exhaustive_sampler.sample();
    let (nodes, edges) = exhaustive_sampler.get_samples();
    println!("{}, {}", nodes.len(), edges.len());
}
