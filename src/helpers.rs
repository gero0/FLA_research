use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, SeedableRng};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

pub fn dist(n1: &Node, n2: &Node) -> i32 {
    let xd = n2.x - n1.x;
    let yd = n2.y - n1.y;
    // return (xd * xd + yd * yd).sqrt();
    let len = 0.5 + (xd * xd + yd * yd).sqrt();
    len as i32
}

pub fn path_len(path: &Vec<u32>, distance_matrix: &Vec<Vec<i32>>) -> i32 {
    let len: i32 = path
        .windows(2)
        .map(|w| distance_matrix[w[0] as usize][w[1] as usize])
        .sum();
    len + distance_matrix[path[0] as usize][path[path.len() - 1] as usize]
}

pub fn nodes_to_ids(path: &[Node]) -> Vec<u32> {
    return path.iter().map(|node| node.id).collect();
}

pub fn generate_distance_matrix(nodes: &Vec<Node>) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; nodes.len()]; nodes.len()];

    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }
            matrix[i][j] = dist(&nodes[i], &nodes[j]);
        }
    }

    matrix
}

pub fn random_solution(node_count: u32, seed: Option<u64>) -> Vec<u32> {
    let mut nodes_remaining: Vec<u32> = (0..node_count).collect();
    let mut starting_path = vec![];

    let mut rng = match seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_entropy(),
    };

    while !nodes_remaining.is_empty() {
        let between = Uniform::from(0..nodes_remaining.len());
        let i = between.sample(&mut rng);
        starting_path.push(nodes_remaining[i]);
        nodes_remaining.remove(i);
    }

    starting_path
}
