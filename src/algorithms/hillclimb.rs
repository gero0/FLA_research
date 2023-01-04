use crate::helpers::{path_len, random_solution, Node};

pub fn hillclimb(nodes: &Vec<Node>, seed: Option<u64>) -> Vec<Node> {
    let mut current_tour = random_solution(nodes, seed);
    let mut current_len = path_len(&current_tour);

    loop {
        let neighbors = get_neighbors(&current_tour);
        let (best_neighbor, best_neighbor_len) = get_best_neighbor(&neighbors);

        if best_neighbor_len >= current_len {
            break;
        }
        current_tour = best_neighbor;
        current_len = best_neighbor_len;
    }

    current_tour
}

fn get_neighbors(path: &Vec<Node>) -> Vec<Vec<Node>> {
    let mut neighbors = vec![];

    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let mut neighbor = path.clone();
            neighbor.swap(i, j);
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn get_best_neighbor(neighbors: &Vec<Vec<Node>>) -> (Vec<Node>, i32) {
    let mut best_len = path_len(&neighbors[0]);
    let mut best_neighbor = neighbors[0].clone();

    for neighbor in &neighbors[1..] {
        let len = path_len(neighbor);
        if len < best_len {
            best_len = len;
            best_neighbor = neighbor.clone();
        }
    }

    return (best_neighbor, best_len);
}
