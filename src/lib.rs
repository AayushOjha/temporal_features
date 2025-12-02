use pyo3::prelude::*;

/// Adds two numbers together.
#[pyfunction]
fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// A Python module implemented in Rust.
#[pymodule]
fn temporal_features(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
