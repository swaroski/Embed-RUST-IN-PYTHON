use pyo3::prelude::*;
use rand;
use std::time::{Duration, SystemTime};

#[pyfunction]
fn mult_matrix_func(n: usize) -> f64 {
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            a[i][j] = rand::random::<f64>();
            b[i][j] = rand::random::<f64>();
            c[i][j] = 0.0;
        }
    }
    let start = SystemTime::now();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    let end = SystemTime::now();
    let duration: Duration = end.duration_since(start).expect("Time");

    let time_spent =
        duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1_000_000_000.0;
    return time_spent;
}

#[pymodule]
fn mult_matrix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mult_matrix_func, m)?)?;
    Ok(())
}