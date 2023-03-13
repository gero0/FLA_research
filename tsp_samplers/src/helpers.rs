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

//random_solution, tour_len and cmp_permutation implementation taken from tsptools library by Kacper Leśniański and Paweł Szczepaniak
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

pub fn cmp_permutations(perm1: &[u16], perm2: &[u16]) -> u32 {
    //invert first permutation
    let mut perm_1_inv = perm1.to_owned();
    for i in 0..perm1.len() {
        perm_1_inv[perm1[i] as usize] = i as u16;
    }

    //Compose the two permutations
    let mut p = vec![0; perm1.len()];
    for i in 0..perm1.len() {
        p[i] = perm2[perm_1_inv[i] as usize];
    }

    let mut count = 0;
    for i in 0..perm1.len() {
        while p[i] != i as u16 {
            let a = p[p[i] as usize];
            let b = p[i];
            p.swap(a.into(), b.into());
            count += 1;
        }
    }
    count
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
