
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(name="SAS")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


// use cpython::{Python, /*PyDict,*/ PyResult, /*PyString*/};
// use cpython::{py_module_initializer, py_fn, /*py_class*/};

#[pyfunction]
fn sumof(n1: isize, n2: isize) -> PyResult<isize> {
    Ok(n1 + n2)
}

#[pyfunction]
#[pyo3(text_signature = "(string, /) -> Basilisks\n\tNandekokoni")]
fn capitalize(strings: String) -> PyResult<String> {
    let mut res: String = String::new();
    for i in strings.split_whitespace() {
        res.push_str( (" ".to_owned() + i[0..1].to_uppercase().as_str() + &i[1..]).as_str() )
    }
    Ok(res)
}


// py_module_initializer!(rust_cpython, |py, m| {
//     m.add(py, "__doc__", "The module for rust-cpython bindings")?;
//     m.add(py, "sumof", py_fn!(py, sumof(n1: isize, n2: isize)))?;
//     m.add(py, "caps", py_fn!(
//         py, capitalize(strings: String)
//     ))?;
//     Ok(())
// });




/// A Python module implemented in Rust.
#[pymodule]
fn pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sumof, m)?)?;
    m.add_function(wrap_pyfunction!(capitalize, m)?)?;
    Ok(())
}