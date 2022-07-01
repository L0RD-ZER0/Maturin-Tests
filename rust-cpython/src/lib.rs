use cpython::{Python, /*PyDict,*/ PyResult, /*PyString*/};
use cpython::{py_module_initializer, py_fn, /*py_class*/};

fn sumof(__py: Python, n1: isize, n2: isize) -> PyResult<isize> {
    Ok(n1 + n2)
}

fn capitalize(__py: Python, strings: String) -> PyResult<String> {
    let mut res: String = String::new();
    for i in strings.split_whitespace() {
        res.push_str( (" ".to_owned() + i[0..1].to_uppercase().as_str() + &i[1..]).as_str() )
    }
    Ok(res)
}


py_module_initializer!(rust_cpython, |py, m| {
    m.add(py, "__doc__", "The module for rust-cpython bindings")?;
    m.add(py, "sumof", py_fn!(py, sumof(n1: isize, n2: isize)))?;
    m.add(py, "caps", py_fn!(
        py, capitalize(strings: String)
    ))?;
    Ok(())
});