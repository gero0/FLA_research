use crate::helpers::*;

pub fn two_opt_random(nodes: &Vec<Node>, seed: Option<u64>) -> Vec<Node> {
    let mut starting_path = random_solution(nodes, seed);
    return two_opt(&starting_path);
}

pub fn two_opt(nodes: &Vec<Node>) -> Vec<Node> {
    let mut tour = nodes.clone();
    let n = tour.len();
    let mut improvement = true;

    while improvement {
        improvement = false;
        let mut min_dist = 0;
        let mut a = 0;
        let mut b = 0;

        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let distance = dist(&tour[i], &tour[j]) + dist(&tour[i + 1], &tour[(j + 1) % n])
                    - dist(&tour[i], &tour[i + 1])
                    - dist(&tour[j], &tour[(j + 1) % n]);

                if distance < min_dist {
                    min_dist = distance;
                    a = i;
                    b = j;
                    improvement = true;
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

    return tour;
}

#[test]
fn rev_test() {
    let mut a = 2;
    let mut b = 5;
    let mut v = vec![1, 2, 3, 4, 5, 6];
    while a < b {
        v.swap(a, b);
        a += 1;
        b -= 1;
    }

    assert_eq!(v, vec![1, 2, 6, 5, 4, 3]);
}
