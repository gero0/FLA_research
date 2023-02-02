use rustc_hash::FxHashMap;

pub type HillclimbFunction = fn(&Vec<u16>, &Vec<Vec<i32>>) -> (Vec<u16>, i32);
pub type NodeMap = FxHashMap<Vec<u16>, (u16, i32)>;
pub type EdgeMap = FxHashMap<(u16, u16), i32>;

pub mod exhaustive_sampler;
pub mod hillclimb;
pub mod pwr_sampler;
pub mod snowball_sampler;
pub mod two_opt;

pub use exhaustive_sampler::ExhaustiveSampler;
pub use hillclimb::hillclimb_steepest;
pub use pwr_sampler::PwrSampler;
pub use snowball_sampler::SnowballSampler;
pub use two_opt::{two_opt_besti, two_opt_firsti};
