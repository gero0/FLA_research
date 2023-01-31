use tsp_samplers::algorithms::snowball_sampler::SnowballSampler;
use tsptools::{
    algorithms::{hillclimb::hillclimb, two_opt::two_opt},
    parsers::parse_tsp_file,
};

fn main() {
    let file = parse_tsp_file("./data/berlin52.tsp").unwrap();

    let mut snowball_sampler =
        SnowballSampler::new(20, 15, 4, 3, file.distance_matrix, hillclimb, Some(2000));
    snowball_sampler.sample();
    let (nodes, edges) = snowball_sampler.get_samples();

    println!("{}, {}", nodes.len(), edges.len());
}
