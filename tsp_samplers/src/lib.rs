use pyo3::{exceptions::PyRuntimeError, prelude::*};

pub mod algorithms;
pub mod helpers;
use algorithms::*;

#[pyclass]
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
        let hillclimb_function = str_to_hc(hillclimb_function)?;
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
        unpack_to_vec(nmap, emap)
    }

    pub fn get_hc_calls(&self) -> u64 {
        self.inner.get_hc_calls()
    }
}

#[pyclass]
#[pyo3(name = "PwrSampler")]
struct PyPwrSampler {
    inner: Box<PwrSampler>,
}

#[pymethods]
impl PyPwrSampler {
    #[new]
    pub fn __new__(
        distance_matrix: Vec<Vec<i32>>,
        hillclimb_function: &str,
        seed: Option<u64>,
    ) -> PyResult<Self> {
        let hillclimb_function = str_to_hc(hillclimb_function)?;
        let inner = Box::new(PwrSampler::new(distance_matrix, hillclimb_function, seed));
        Ok(Self { inner })
    }

    pub fn sample(&mut self, n_max: u32, n_att: u32, e_att: u32) {
        self.inner.sample(n_max, n_att, e_att);
    }

    pub fn get_results(&mut self) -> (Vec<(Vec<u16>, u16, i32)>, Vec<(u16, u16, i32)>) {
        let (nmap, emap) = self.inner.get_samples();
        unpack_to_vec(nmap, emap)
    }

    pub fn get_hc_calls(&self) -> u64 {
        self.inner.get_hc_calls()
    }
}

#[pymodule]
fn tsp_samplers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySnowballSampler>()?;
    m.add_class::<PyPwrSampler>()?;
    Ok(())
}

fn str_to_hc(hillclimb_str: &str) -> PyResult<HillclimbFunction> {
    let hillclimb_function = match hillclimb_str {
        "2opt" | "twoopt" | "two_opt" => two_opt_besti,
        "2opt_fi" | "twooptfi" | "two_opt_fi" | "twoopt_fi" => two_opt_firsti,
        "hc" | "hillclimb" => hillclimb_steepest,
        _ => {
            return Err(PyErr::new::<PyRuntimeError, _>(
                "Invalid hillclimb algorithm",
            ))
        }
    };

    Ok(hillclimb_function)
}

fn unpack_to_vec(
    nmap: &NodeMap,
    emap: &EdgeMap,
) -> (Vec<(Vec<u16>, u16, i32)>, Vec<(u16, u16, i32)>) {
    let n_vec: Vec<_> = nmap
        .into_iter()
        .map(|a| (a.0.clone(), a.1 .0, a.1 .1))
        .collect();
    let e_vec: Vec<_> = emap.into_iter().map(|a| (a.0 .0, a.0 .1, *a.1)).collect();
    (n_vec, e_vec)
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
