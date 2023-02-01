use rand::{distributions::Uniform, prelude::Distribution};
use rand_chacha::ChaCha8Rng;

pub fn generate_perms(set: &[u16], preserve_first: bool) -> Vec<Vec<u16>> {
    let mut perms = vec![];
    let mut set = set.to_owned();

    let n = set.len();

    if preserve_first {
        heap_perm(&mut set[1..], n - 1, &mut perms);
        for perm in perms.iter_mut() {
            let new = vec![set[0]];
            *perm = [new, perm.clone()].concat();
        }
    } else {
        heap_perm(&mut set, n, &mut perms);
    }

    perms
}

fn heap_perm(a: &mut [u16], k: usize, perms_vec: &mut Vec<Vec<u16>>) {
    if k == 1 {
        perms_vec.push(a.to_vec());
    } else {
        for i in 0..k {
            heap_perm(a, k - 1, perms_vec);
            if (k % 2) == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
        }
    }
}

pub fn mutate(perm: &Vec<u16>, n_swaps: usize, rng: &mut ChaCha8Rng) -> Vec<u16> {
    let mut mutation = perm.to_owned();
    let mut i = 0;
    while i < n_swaps {
        let between = Uniform::from(0..perm.len());
        let a = between.sample(rng);
        let b = between.sample(rng);

        if a == b {
            continue;
        }

        mutation.swap(a, b);
        i += 1;
    }

    mutation
}

pub fn tour_len(path: &Vec<u16>, distance_matrix: &Vec<Vec<i32>>) -> i32 {
    let len: i32 = path.windows(2).map(|w| distance_matrix[w[0] as usize][w[1] as usize]).sum();
    len + distance_matrix[path[0] as usize][path[path.len() - 1] as usize]
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn factorial(n: usize) -> usize {
        let mut product = 1;
        for i in 1..n + 1 {
            product *= i;
        }
        product
    }

    #[test]
    fn factorial_test() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn test_perm_generation() {
        let sets: [Box<[u16]>; 3] = [
            Box::new([1, 2, 3]),
            Box::new([1, 2, 3, 4]),
            Box::new([1, 2, 3, 4, 5, 6]),
        ];
        for set in &sets {
            let perms = generate_perms(&set, false);
            let mut hs = HashSet::new();
            for perm in perms {
                hs.insert(perm);
            }
            //ensures we have n! unique permutations
            assert_eq!(hs.len(), factorial(set.len()))
        }

        for set in sets {
            let perms = generate_perms(&set, true);
            println!("{:?}", perms);
            let mut hs = HashSet::new();
            for perm in perms {
                hs.insert(perm);
            }
            //ensures we have n! unique permutations
            assert_eq!(hs.len(), factorial(set.len() - 1));
        }
    }
}
