use std::{error::Error, fmt::Display, fs};

use rand::{distributions::Uniform, prelude::Distribution, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rustc_hash::FxHashSet;

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

//checks all permutations - it's ok since we're going to use it only in exhaustive search
//on instances with N=12 max
pub fn inrange_2change(perm1: &[u16], perm2: &[u16], mut_d: usize) -> bool {
    let mut permutations = FxHashSet::default();
    permutations.insert(perm1.to_owned());

    //first find all 2-change permutations based on perm1
    //then generate new permutations from all permutations created from perm1
    //repeat the process recursively
    //check if any of the permutations matches perm2

    for _i in 0..mut_d {
        let mut new_perms = FxHashSet::default();
        for perm in permutations {
            let child_perms = two_exchange_allperms(&perm);
            for p in child_perms {
                //if we found perm2 we can return early
                //otherwise add to set of new permutations that will replace original permutations
                if p == perm2 {
                    return true;
                }
                new_perms.insert(p);
            }
        }
        permutations = new_perms;
    }

    //perm 2 not found - not in mut_d distance
    false
}

fn two_exchange_allperms(perm: &[u16]) -> Vec<Vec<u16>> {
    let mut perms = vec![];
    let n = perm.len();
    for a in 0..(n - 2) {
        for b in (a + 2)..n {
            let new_perm = two_exchange(perm, a, b);
            perms.push(new_perm);
        }
    }

    perms
}

pub fn mutate_2exchange(perm: &[u16], n_swaps: usize, rng: &mut ChaCha8Rng) -> Vec<u16> {
    let mut mutation = perm.to_owned();

    for _i in 0..n_swaps {
        let between = Uniform::from(0..perm.len() - 2);
        let a = between.sample(rng);
        let between_a = Uniform::from(a + 2..perm.len());
        let b = between_a.sample(rng);

        mutation = two_exchange(perm, a, b);
    }

    mutation
}

pub fn two_exchange(perm: &[u16], mut a: usize, mut b: usize) -> Vec<u16> {
    assert!(a < b);
    assert!(a < perm.len() && b < perm.len());

    let mut mutation = perm.to_owned();
    a += 1;
    while a < b {
        mutation.swap(a, b);
        a += 1;
        b -= 1;
    }
    mutation
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn two_exchange_test() {
        let perm = [1, 2, 3, 4, 5, 6];
        let exchange = two_exchange(&perm, 1, 4);
        assert_eq!(exchange, [1, 2, 5, 4, 3, 6]);
        let perm2 = [1, 4, 3, 2, 5];
        assert_eq!(two_exchange(&perm2, 0, 3), [1, 2, 3, 4, 5]);
        let perm3 = [1, 2, 3, 4];
        assert_eq!(two_exchange(&perm3, 1, 2), [1, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn two_exchange_panic() {
        let perm3 = [1];
        assert_eq!(two_exchange(&perm3, 0, 0), [1])
    }

    #[test]
    fn test_twoopt_perms() {
        let starting = [1, 2, 3, 4];
        let permutations = two_exchange_allperms(&starting);
        assert_eq!(permutations.len(), 6);
        let uniq: Vec<_> = permutations.into_iter().unique().collect();
        assert_eq!(uniq.len(), 4);

        let starting = [1, 2, 3, 4, 5];
        let permutations = two_exchange_allperms(&starting);
        assert_eq!(permutations.len(), 10);
        let uniq: Vec<_> = permutations.into_iter().unique().collect();
        assert_eq!(uniq.len(), 7);
    }

    #[test]
    fn test_in_range() {
        let perm1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let perm2 = [1, 9, 8, 7, 6, 5, 4, 3, 2];
        let perm3 = [1, 9, 8, 7, 6, 3, 4, 5, 2];
        let perm4 = [1, 7, 8, 9, 6, 3, 4, 5, 2];

        assert!(inrange_2change(&perm1, &perm2, 2));
        assert!(inrange_2change(&perm1, &perm3, 2));
        assert!(!inrange_2change(&perm1, &perm4, 2));
        assert!(inrange_2change(&perm1, &perm4, 3));
    }

    #[test]
    fn test_mutate() {
        let perm1 = [1, 2, 3];
        let mut rng = ChaCha8Rng::from_entropy();
        let mutation = mutate_2exchange(&perm1, 1, &mut rng);
        assert!((mutation == perm1) || mutation == [1, 3, 2]);
    }
}
