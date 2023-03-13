use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::{
        atomic::{AtomicU16, Ordering},
        Mutex,
    },
    thread::{self, available_parallelism},
};

use permutations_iter::Permutations;

use crate::{algorithms::hillclimb::hillclimb_steepest, helpers::cmp_permutations};

use super::{EdgeMap, HillclimbFunction, NodeMap};

pub struct ExhaustiveSampler {
    distance_matrix: Vec<Vec<i32>>,
    permpath: String,
    solutions: NodeMap,
    edges: EdgeMap,
}

impl ExhaustiveSampler {
    pub fn new(distance_matrix: Vec<Vec<i32>>) -> Self {
        let set: Vec<_> = (0..distance_matrix.len() as u16).collect();
        let permpath = String::from(generate_perms(&set));

        Self {
            distance_matrix,
            permpath,
            solutions: NodeMap::default(),
            edges: EdgeMap::default(),
        }
    }

    pub fn sample(&mut self) {
        let distance_matrix = self.distance_matrix.clone();

        let last_id = AtomicU16::new(0);

        let file = File::open(&self.permpath).unwrap();

        let lines = BufReader::new(file).lines();

        //find local optima

        for line in lines {
            let v = deserialize(&line.unwrap());
            let (solution, s_len) = hillclimb_steepest(&v, &distance_matrix);

            if self.solutions.get(&solution).is_none() {
                let id = last_id.fetch_add(1, Ordering::Relaxed);
                self.solutions.insert(solution.clone(), (id, s_len));
            }
        }

        for (ka, va) in self.solutions.iter() {
            for (kb, vb) in self.solutions.iter() {
                let dist = cmp_permutations(ka, kb) as i32;
                self.edges.insert((va.0, vb.0), dist);
            }
        }
    }
    pub fn get_samples(&self) -> (&NodeMap, &EdgeMap) {
        (&self.solutions, &self.edges)
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

// fn heap_perm(a: &mut [u16], k: usize, perms_vec: &mut Vec<Vec<u16>>) {
//     if k == 1 {
//         perms_vec.push(a.to_vec());
//     } else {
//         for i in 0..k {
//             heap_perm(a, k - 1, perms_vec);
//             if (k % 2) == 0 {
//                 a.swap(i, k - 1);
//             } else {
//                 a.swap(0, k - 1);
//             }
//         }
//     }
// }
