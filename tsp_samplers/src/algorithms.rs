use rustc_hash::FxHashMap;

pub type HillclimbFunction = fn(&Vec<usize>, &Vec<Vec<i32>>, bool) -> (Vec<usize>, i32);
pub type NodeMap = FxHashMap<Vec<usize>, (u32, i32)>;
pub type EdgeMap = FxHashMap<(u32, u32), i32>;

pub mod snowball_sampler;
