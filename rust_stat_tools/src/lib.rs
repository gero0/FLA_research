use pyo3::prelude::*;
use std::{
    sync::Mutex,
    thread::{self, available_parallelism},
};

#[pyfunction]
fn num_subsinks(nodes: Vec<(u32, Vec<u32>, i32)>, edges: Vec<(u32, u32, u32)>) -> usize {
    subsinks(&nodes, &edges).len()
}

#[pyfunction]
fn num_sinks(nodes: Vec<(u32, Vec<u32>, i32)>, edges: Vec<(u32, u32, u32)>) -> usize {
    let thread_count: usize = available_parallelism().unwrap().get();
    let sinks = Mutex::new(vec![]);

    for subset in nodes.chunks(thread_count) {
        thread::scope(|s| {
            for node in subset {
                s.spawn(|| {
                    if is_sink(node, &edges) {
                        sinks.lock().unwrap().push(node.0);
                    }
                });
            }
        });
    }

    return sinks.into_inner().unwrap().len();
}

fn is_sink(current_node: &(u32, Vec<u32>, i32), edges: &Vec<(u32, u32, u32)>) -> bool {
    let (id, _perm, _path_len) = current_node;
    // Count outgoing edges of this node

    let mut counter = 0;
    for edge in edges {
        let (src, dst, _weight) = edge;
        if *src == *id && *src != *dst {
            counter += 1;
        }
    }

    counter == 0
}

#[pyfunction]
fn num_sources(nodes: Vec<(u32, Vec<u32>, i32)>, edges: Vec<(u32, u32, u32)>) -> usize {
    let thread_count: usize = available_parallelism().unwrap().get();
    let sinks = Mutex::new(vec![]);

    for subset in nodes.chunks(thread_count) {
        thread::scope(|s| {
            for node in subset {
                s.spawn(|| {
                    if is_source(node, &edges) {
                        sinks.lock().unwrap().push(node.0);
                    }
                });
            }
        });
    }

    return sinks.into_inner().unwrap().len();
}

fn is_source(current_node: &(u32, Vec<u32>, i32), edges: &Vec<(u32, u32, u32)>) -> bool {
    let (id, _perm, _path_len) = current_node;
    // Count incoming edges of this node

    let mut counter = 0;
    for edge in edges {
        let (src, dst, _weight) = edge;
        if *dst == *id && src != dst {
            counter += 1;
        }
    }

    counter == 0
}

fn subsinks(nodes: &Vec<(u32, Vec<u32>, i32)>, edges: &Vec<(u32, u32, u32)>) -> Vec<u32> {
    let thread_count: usize = available_parallelism().unwrap().get();
    let subsinks = Mutex::new(vec![]);

    for subset in nodes.chunks(thread_count) {
        thread::scope(|s| {
            for node in subset {
                s.spawn(|| {
                    if is_subsink(node, nodes, edges) {
                        subsinks.lock().unwrap().push(node.0);
                    }
                });
            }
        });
    }

    return subsinks.into_inner().unwrap();
}

fn is_subsink(
    current_node: &(u32, Vec<u32>, i32),
    nodes: &Vec<(u32, Vec<u32>, i32)>,
    edges: &Vec<(u32, u32, u32)>,
) -> bool {
    let (id, _perm, path_len) = current_node;
    //Find all outgoing edges of this node
    let mut outgoing_edges = vec![];
    for edge in edges {
        let (src, dst, _weight) = edge;
        if *src == *id && src != dst {
            outgoing_edges.push(edge)
        }
    }

    let mut counter = 0;
    // Count all nodes with shorter path that are destination of edges
    for edge in outgoing_edges {
        let (_src, dst, _weight) = edge;
        if *dst == *id {
            //ignore loops
            continue;
        }
        for node in nodes {
            let (id, _perm, p_len) = node;
            if *id == *dst && *p_len < *path_len {
                counter += 1;
            }
        }
    }

    if counter == 0 {
        return true;
    }

    false
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_stat_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(num_subsinks, m)?)?;
    m.add_function(wrap_pyfunction!(num_sinks, m)?)?;
    m.add_function(wrap_pyfunction!(num_sources, m)?)?;
    Ok(())
}
