//Based on implementations from tsptools library by Kacper Leśniański and Paweł Szczepaniak
//https://github.com/gero0/tsptools

use crate::helpers::tour_len;

pub fn hillclimb_steepest(
    starting_tour: &Vec<u16>,
    distance_matrix: &Vec<Vec<i32>>,
) -> (Vec<u16>, i32) {
    let mut current_tour = starting_tour.clone();
    let mut current_len = tour_len(&current_tour, distance_matrix);

    loop {
        let neighbors = get_neighbors(&current_tour);
        let (best_neighbor, best_neighbor_len) = get_best_neighbor(&neighbors, distance_matrix);

        if best_neighbor_len >= current_len {
            break;
        }
        current_tour = best_neighbor;
        current_len = best_neighbor_len;
    }

    (current_tour, current_len)
}

fn get_neighbors(path: &Vec<u16>) -> Vec<Vec<u16>> {
    let mut neighbors = vec![];

    for i in 1..path.len() {
        for j in i + 1..path.len() {
            let mut neighbor = path.clone();
            neighbor.swap(i, j);
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn get_best_neighbor(
    neighbors: &Vec<Vec<u16>>,
    distance_matrix: &Vec<Vec<i32>>,
) -> (Vec<u16>, i32) {
    let mut best_len = tour_len(&neighbors[0], distance_matrix);
    let mut best_neighbor_index = 0;

    for (i, neighbor) in neighbors[1..].iter().enumerate() {
        let len = tour_len(neighbor, distance_matrix);
        if len < best_len {
            best_len = len;
            best_neighbor_index = i + 1;
        }
    }

    (neighbors[best_neighbor_index].clone(), best_len)
}
