use rustc_hash::FxHashMap;

pub type HillclimbFunction = fn(&Vec<u16>, &Vec<Vec<i32>>, bool) -> (Vec<u16>, i32);
pub type NodeMap = FxHashMap<Vec<u16>, (u16, i32)>;
pub type EdgeMap = FxHashMap<(u16, u16), i32>;

pub mod snowball_sampler;
pub mod exhaustive_sampler;
