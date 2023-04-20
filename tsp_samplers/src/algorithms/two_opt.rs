//Based on implementations from tsptools library by Kacper Leśniański and Paweł Szczepaniak
//https://github.com/gero0/tsptools

use crate::helpers::tour_len;

pub fn two_opt_besti(starting_tour: &Vec<u32>, distance_matrix: &Vec<Vec<i32>>) -> (Vec<u32>, i32, u128) {
    two_opt_base(starting_tour, distance_matrix, false)
}

pub fn two_opt_firsti(
    starting_tour: &Vec<u32>,
    distance_matrix: &Vec<Vec<i32>>,
) -> (Vec<u32>, i32, u128) {
    two_opt_base(starting_tour, distance_matrix, true)
}

fn two_opt_base(
    starting_tour: &Vec<u32>,
    distance_matrix: &Vec<Vec<i32>>,
    first_i: bool,
) -> (Vec<u32>, i32, u128) {
    let mut tour = starting_tour.to_owned();
    let n = tour.len();
    let mut improvement = true;

    while improvement {
        improvement = false;
        let mut min_dist = 0;
        let mut a = 0;
        let mut b = 0;

        'outer: for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let distance = distance_matrix[tour[i] as usize][tour[j] as usize]
                    + distance_matrix[tour[i + 1] as usize][tour[(j + 1) % n] as usize]
                    - distance_matrix[tour[i] as usize][tour[i + 1] as usize]
                    - distance_matrix[tour[j] as usize][tour[(j + 1) % n] as usize];

                if distance < min_dist {
                    min_dist = distance;
                    a = i;
                    b = j;
                    improvement = true;
                    if first_i {
                        break 'outer;
                    }
                }
            }
        }

        if !improvement {
            break;
        }

        //reverse [a+1, b]
        a += 1;
        while a < b {
            tour.swap(a, b);
            a += 1;
            b -= 1;
        }
    }

    let len = tour_len(&tour, distance_matrix);
    (tour, len, 1)
}

