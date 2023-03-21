use permutations_iter::Permutations;
use rustc_hash::FxHashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::helpers::cmp_permutations;

use super::{EdgeMap, HillclimbFunction, NodeMap, SamplingAlg};

pub struct ExhaustiveSampler {
    distance_matrix: Vec<Vec<i32>>,
    hillclimb_alg: HillclimbFunction,
    permpath: String,
    mut_d: u32,
    nodes: NodeMap,
    edges: EdgeMap,
    last_node_id: u16,
    hc_counter: u64,
    oracle_counter: u128,
}

impl ExhaustiveSampler {
    pub fn new(
        distance_matrix: Vec<Vec<i32>>,
        mut_d: u32,
        hillclimb_alg: HillclimbFunction,
    ) -> Self {
        let set: Vec<_> = (0..distance_matrix.len() as u16).collect();
        let permpath = String::from(generate_perms(&set));

        Self {
            distance_matrix,
            permpath,
            mut_d,
            nodes: NodeMap::default(),
            edges: EdgeMap::default(),
            last_node_id: 0,
            hc_counter: 0,
            oracle_counter: 0,
            hillclimb_alg,
        }
    }

    pub fn sample(&mut self) {
        let distance_matrix = self.distance_matrix.clone();
        let file = File::open(&self.permpath).unwrap();
        let lines = BufReader::new(file).lines();

        //find all local optima by running hillclimb for every possible permutation.
        //Save which LO the solution led to, we will need it to constuct LON edges
        let mut pairs = FxHashMap::default();

        for line in lines {
            let solution = deserialize(&line.unwrap());
            let (lo, s_len, oracle) = (self.hillclimb_alg)(&solution, &distance_matrix);
            self.hc_counter += 1;
            self.oracle_counter += oracle;
            pairs.insert(solution, lo.clone());

            if self.nodes.get(&lo).is_none() {
                let id = self.get_next_id();
                self.nodes.insert(lo.clone(), (id, s_len));
            }
        }

        for (perm, lo) in pairs.iter() {
            for other_lo in self.nodes.iter() {
                //Compare with each local optimum in search space
                let dist = cmp_permutations(perm, other_lo.0) as i32;
                //If you can get to this optimum from current solution with mut_d swaps, add/update edge in LON
                if dist < self.mut_d as i32 {
                    let lo_id = self.nodes.get(lo).unwrap().0;
                    match self.edges.get_mut(&(other_lo.1 .0, lo_id)) {
                        Some(weight) => {
                            *weight += 1;
                        }
                        None => {
                            self.edges.insert((other_lo.1 .0, lo_id), 1);
                        }
                    };
                }
            }
        }
    }

    fn get_next_id(&mut self) -> u16 {
        self.last_node_id += 1;
        self.last_node_id - 1
    }
}

impl SamplingAlg for ExhaustiveSampler {
    fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.nodes, &self.edges)
    }

    fn get_hc_calls(&self) -> u64 {
        self.hc_counter
    }

    fn get_oracle_calls(&self) -> u128 {
        self.oracle_counter
    }
}

pub fn generate_perms(set: &[u16]) -> &str {
    let set = set.to_owned();
    let n = set.len();

    let mut f = File::create("perm_temp.txt").unwrap();

    //don't move the first element
    for permutation in Permutations::of(n - 1) {
        let mut new_perm = vec![set[0]];
        for elem in permutation {
            new_perm.push(set[elem + 1]);
        }
        f.write_fmt(format_args!("{:?}\n", new_perm)).unwrap();
    }

    return "perm_temp.txt";
}

fn deserialize(line: &str) -> Vec<u16> {
    let line = line.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
    let mut perm: Vec<u16> = vec![];
    for s in line.split(",") {
        let s = s.strip_prefix(" ").unwrap_or(s);
        perm.push(s.parse().unwrap());
    }
    perm
}
