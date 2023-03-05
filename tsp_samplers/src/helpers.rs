use std::{error::Error, fmt::Display, fs};

use rand::{distributions::Uniform, prelude::Distribution, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone)]
pub struct TspFile {
    pub name: String,
    pub dimension: usize,
    pub distance_matrix: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct ParsingError;

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing Error")
    }
}

impl Error for ParsingError {}

//random_solution and tour_len implementation taken from tsptools library by Kacper Leśniański and Paweł Szczepaniak
//https://github.com/gero0/tsptools
pub fn random_solution(node_count: u16, seed: Option<u64>, preserve_first: bool) -> Vec<u16> {
    let (mut nodes_remaining, mut path) = if preserve_first {
        ((1..node_count).collect::<Vec<u16>>(), vec![0])
    } else {
        ((0..node_count).collect(), vec![])
    };

    let mut rng = match seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_entropy(),
    };

    while !nodes_remaining.is_empty() {
        let between = Uniform::from(0..nodes_remaining.len());
        let i = between.sample(&mut rng);
        path.push(nodes_remaining[i]);
        nodes_remaining.remove(i);
    }

    path
}

pub fn tour_len(path: &Vec<u16>, distance_matrix: &Vec<Vec<i32>>) -> i32 {
    let len: i32 = path
        .windows(2)
        .map(|w| distance_matrix[w[0] as usize][w[1] as usize])
        .sum();
    len + distance_matrix[path[0] as usize][path[path.len() - 1] as usize]
}

pub fn parse_intermediate_format(path: &str) -> Result<TspFile, Box<dyn Error>> {
    let file = fs::read_to_string(path).unwrap();
    let mut lines = file.lines();
    let name = lines.next().ok_or(ParsingError)?;
    let dim = lines.next().ok_or(ParsingError)?;
    let dim: usize = dim.parse()?;

    let mut matrix = vec![vec![0; dim]; dim];

    for i in 0..dim {
        let row = lines.next().ok_or(ParsingError)?;
        let tokens = row.split_whitespace();
        for (j, token) in tokens.enumerate() {
            matrix[i][j] = token.parse()?;
        }
    }

    let file = TspFile {
        name: String::from(name),
        dimension: dim,
        distance_matrix: matrix,
    };

    Ok(file)
}

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
