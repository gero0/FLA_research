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
