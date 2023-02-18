use pyo3::prelude::*;
use oxide::{add, subtract};

/// Formats the sum of two numbers as string.
///
/// This uses our core library, `oxide` to handle the
/// addition in Rust.
///
/// # Args
///     a (usize): left operand
///     b (usize): right operand
///
/// # Returns
///     Sum of `a` and `b` as a String
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(add(a, b).to_string())
}

/// Formats the difference of two numbers as string.
///
/// This uses our core library, `oxide` to handle the
/// addition in Rust.
///
/// # Args
///     a (usize): left operand
///     b (usize): right operand
///
/// # Returns
///     Difference of `a` and `b` as a String
#[pyfunction]
fn subtract_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(subtract(a, b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    Ok(())
}
