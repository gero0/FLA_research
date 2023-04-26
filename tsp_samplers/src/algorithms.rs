use rustc_hash::FxHashMap;

pub type HillclimbFunction = fn(&Vec<u32>, &Vec<Vec<i32>>) -> (Vec<u32>, i32, u64);
pub type NodeMap = FxHashMap<Vec<u32>, (u32, i32)>;
pub type EdgeMap = FxHashMap<(u32, u32), i32>;

pub mod exhaustive_sampler;
pub mod hillclimb;
pub mod snowball_sampler;
pub mod tp_sampler;
pub mod two_opt;

pub use hillclimb::hillclimb_steepest;
pub use snowball_sampler::SnowballSampling;
pub use tp_sampler::TPSampling;
pub use two_opt::{two_opt_besti, two_opt_firsti};

pub trait SamplingAlg {
    fn get_hc_calls(&self) -> u64;
    fn get_oracle_calls(&self) -> u64;
}
