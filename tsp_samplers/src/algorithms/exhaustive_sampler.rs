use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Mutex,
    },
    thread::{self, available_parallelism}, fs::File, io::Write
};

use permutations_iter::Permutations;

use crate::algorithms::hillclimb::hillclimb_steepest;

use super::{EdgeMap, HillclimbFunction, NodeMap};

pub struct ExhaustiveSampler {
    distance_matrix: Vec<Vec<i32>>,
    solutions: NodeMap,
    edges: EdgeMap,
}

impl ExhaustiveSampler {
    pub fn new(
        distance_matrix: Vec<Vec<i32>>,
    ) -> Self {
        let set: Vec<_> = (0..distance_matrix.len() as u16).collect();
        let permutations = generate_perms(&set);

        Self {
            distance_matrix,
            solutions: NodeMap::default(),
            edges: EdgeMap::default(),
        }
    }

    pub fn sample(&mut self) {
        let distance_matrix = self.distance_matrix.clone();
        let solutions = Mutex::new(NodeMap::default());
        let edges = Mutex::new(EdgeMap::default());

        // let thread_count: usize = available_parallelism().unwrap().get();
        let thread_count = 1;

        let last_id = AtomicU16::new(0);

        // thread::scope(|s| {
        //     for chunk in self.permutations.chunks(thread_count) {
        //         for permutation in chunk {
        //             s.spawn(|| {
        //                 let (solution, s_len) =
        //                     (self.hillclimb_function)(permutation, &distance_matrix);
        //                 let id = last_id.fetch_add(1, Ordering::Relaxed);
        //                 solutions
        //                     .lock()
        //                     .expect("ExhaustiveSampler: Mutex poisoned, bailing out!")
        //                     .insert(solution, (id, s_len));
        //             });
        //         }
        //     }
        // });
    }
    pub fn get_samples(&self) -> (NodeMap, EdgeMap)
    {
        (self.solutions.clone(), self.edges.clone())
    }
}

pub fn generate_perms(set: &[u16]) -> &str {
    let set = set.to_owned();
    let n = set.len();

    let mut f = File::create("perm_temp.txt").unwrap();

    //don't move the first element
    for permutation in Permutations::of(n-1){
        let mut new_perm = vec![set[0]];
        for elem in permutation {
            new_perm.push(set[elem + 1]);
        }
        f.write_fmt(format_args!("{:?}\n", new_perm)).unwrap();
    }

    return "perm_temp.txt";
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


