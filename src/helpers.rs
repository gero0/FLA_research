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
    return len as i32;
}

pub fn path_len(path: &Vec<Node>) -> i32 {
    let len: i32 = path.windows(2).map(|w| dist(&w[0], &w[1])).sum();
    return len + dist(&path[0], &path[path.len() - 1]);
}

pub fn path_to_ids(path: &Vec<Node>) -> Vec<u32> {
    return path.iter().map(|node| node.id).collect();
}

pub fn random_solution(nodes: &Vec<Node>, seed: Option<u64>) -> Vec<Node> {
    let mut nodes_remaining = nodes.clone();
    let mut starting_path = vec![];

    let mut rng = match seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_entropy(),
    };
    
    while !nodes_remaining.is_empty() {
        let between = Uniform::from(0..nodes_remaining.len());
        let i = between.sample(&mut rng);
        starting_path.push(nodes_remaining[i].clone());
        nodes_remaining.remove(i);
    }

    return starting_path;
}
