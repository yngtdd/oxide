use pyo3::prelude::*;
use oxide::add;

/// Formats the sum of two numbers as string.
///
/// This uses our core library, `oxide` to handle the
/// addition in Rust.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(add(a, b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
