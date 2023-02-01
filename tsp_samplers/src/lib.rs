use pyo3::{exceptions::PyRuntimeError, prelude::*};

pub mod algorithms;
pub mod helpers;
use algorithms::{
    hillclimb::hillclimb_steepest,
    snowball_sampler::SnowballSampler,
    two_opt::{two_opt_besti, two_opt_firsti},
};

#[pyclass]
#[pyo3(
    text_signature = "(walk_len, n_edges, depth, mut_d, distance_matrix, hillclimb_function, seed)"
)]
#[pyo3(name = "SnowballSampler")]
struct PySnowballSampler {
    inner: Box<SnowballSampler>,
}

#[pymethods]
impl PySnowballSampler {
    #[new]
    pub fn __new__(
        walk_len: u32,
        n_edges: u32,
        depth: u32,
        mut_d: usize,
        distance_matrix: Vec<Vec<i32>>,
        hillclimb_function: &str,
        seed: Option<u64>,
    ) -> PyResult<Self> {
        let hillclimb_function = match hillclimb_function {
            "2opt" | "twoopt" | "two_opt" => two_opt_besti,
            "2opt_fi" | "twooptfi" | "two_opt_fi" | "twoopt_fi" => two_opt_firsti,
            "hc" | "hillclimb" => hillclimb_steepest,
            _ => {
                return Err(PyErr::new::<PyRuntimeError, _>(
                    "Invalid hillclimb algorithm",
                ))
            }
        };
        let inner = Box::new(SnowballSampler::new(
            walk_len,
            n_edges,
            depth,
            mut_d,
            distance_matrix,
            hillclimb_function,
            seed,
        ));
        Ok(Self { inner })
    }

    pub fn sample(&mut self) {
        self.inner.sample();
    }

    pub fn get_results(&mut self) -> (Vec<(Vec<u16>, u16, i32)>, Vec<(u16, u16, i32)>) {
        let (nmap, emap) = self.inner.get_samples();
        let n_vec: Vec<_> = nmap
            .into_iter()
            .map(|a| (a.0.clone(), a.1 .0, a.1 .1))
            .collect();
        let e_vec: Vec<_> = emap.into_iter().map(|a| (a.0 .0, a.0 .1, *a.1)).collect();

        (n_vec, e_vec)
    }

    pub fn get_hc_calls(&self) -> u64 {
        self.inner.get_hc_calls()
    }
}

#[pymodule]
fn tsp_samplers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySnowballSampler>()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::algorithms::{snowball_sampler::SnowballSampler, two_opt::two_opt_besti};
    use tsptools::parsers::parse_tsp_file;

    #[test]
    fn sampling_test() {
        let file = parse_tsp_file("./data/bays29.tsp").unwrap();

        let mut snowball_sampler =
            SnowballSampler::new(1, 5, 3, 2, file.distance_matrix, two_opt_besti, Some(2000));
        snowball_sampler.sample();
        let (nodes, edges) = snowball_sampler.get_samples();

        assert_eq!(nodes.len(), 16);
        assert_eq!(edges.len(), 23);
    }
}

// fn main() {
//     let file = parse_tsp_file("./data/ulysses16.tsp").unwrap();

//     let mut snowball_sampler =
//         SnowballSampler::new(5, 5, 3, 2, file.distance_matrix, &two_opt, Some(2000));
//     let (nodes, edges) = snowball_sampler.sample();

//     let mut node_file = File::create("nodes.txt").expect("I assumed the OS will cooperate...");
//     let mut edge_file = File::create("edges.txt").expect("I assumed the OS will cooperate...");

//     for node in nodes {
//         let (perm, (id, h)) = node;
//         node_file
//             .write_fmt(format_args!("{};{:?};{}\n", id, perm, h))
//             .unwrap();
//     }

//     for edge in edges {
//         let ((a, b), w) = edge;
//         edge_file
//             .write_fmt(format_args!("{};{};{}\n", a, b, w))
//             .unwrap();
//     }
// }
